use std::path::{Path, PathBuf};

use crate::factories::pandera::PanderaHandler;
use anyhow::Result;
use rustpython_parser::{ast, Parse};

fn walk_dir(dir: &Path, suffix: &str) -> Result<Vec<PathBuf>> {
    let mut files = vec![];
    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.append(&mut walk_dir(&path, suffix)?);
        } else if path.extension().is_some_and(|ext| ext == suffix) {
            files.push(path);
        }
    }
    Ok(files)
}

fn render_factory_code<R: std::fmt::Debug>(stmt: &ast::Stmt<R>) -> Result<Option<String>> {
    match stmt {
        ast::Stmt::ClassDef(class_def) => {
            PanderaHandler::new().generate_pandera_dataframe_factory(class_def)
        }
        _ => Ok(None),
    }
}

pub fn render_factory_code_from_file(file: &Path) -> Result<Option<String>> {
    let content = std::fs::read_to_string(file)?;
    let suite = ast::Suite::parse(&content, file.to_str().unwrap())?;
    let factory_codes: Vec<String> = suite
        .iter()
        .map(render_factory_code)
        .collect::<Result<Vec<Option<String>>>>()?
        .into_iter()
        .flatten()
        .collect();
    if factory_codes.is_empty() {
        return Ok(None);
    }
    let import_statements = r#"import datetime
from typing import TypedDict

import fakerator as f


"#;
    Ok(Some(
        import_statements.to_string() + &factory_codes.join("\n"),
    ))
}

pub fn write_factory_codes(root_dir: &Path, output_dir: &Path) -> Result<()> {
    let files = walk_dir(root_dir, "py")?;
    for file in files {
        let Some(factory_code) = render_factory_code_from_file(&file)? else {
            continue;
        };
        let relative_path = file.strip_prefix(root_dir)?.to_path_buf();
        let factory_file = output_dir.join(relative_path).with_extension("py");
        std::fs::create_dir_all(factory_file.parent().unwrap())?;
        std::fs::write(factory_file, factory_code)?;
    }
    Ok(())
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use tempfile::TempDir;

    #[rstest]
    #[case("pandera_all_fields_input.py", "pandera_all_fields_expected.py")]
    #[case(
        "pandera_field_parameters_input.py",
        "pandera_field_parameters_expected.py"
    )]
    fn test_render_factory_code_from_file_should_generate_pandera_record_factory(
        #[case] input_file: &str,
        #[case] expected_file: &str,
    ) {
        let root_dir = PathBuf::from("./resources/generator/render_factory_code_from_file");
        let file = root_dir.join(input_file);
        let result = render_factory_code_from_file(&file);
        assert!(
            result.is_ok(),
            "Failed to render factory code from file: {:?}",
            result.err()
        );
        let actual_factory_code = result.unwrap();
        let expected_factory_code = std::fs::read_to_string(root_dir.join(expected_file)).unwrap();
        assert_eq!(actual_factory_code, Some(expected_factory_code));
    }

    #[rstest]
    #[case("no_render_code.py")]
    fn test_render_factory_code_from_file_should_none_when_target_class_not_found(
        #[case] input_file: &str,
    ) {
        let root_dir = PathBuf::from("./resources/generator/render_factory_code_from_file");
        let file = root_dir.join(input_file);
        let result = render_factory_code_from_file(&file);
        assert!(
            result.is_ok(),
            "Failed to render factory code from file: {:?}",
            result.err()
        );
        let actual_factory_code = result.unwrap();
        assert_eq!(actual_factory_code, None);
    }

    #[test]
    fn test_write_factory_codes() {
        let root_dir = PathBuf::from("./resources/generator/write_factory_codes/input");
        let output_dir = TempDir::new().unwrap().path().to_path_buf();
        let expected_dir = PathBuf::from("./resources/generator/write_factory_codes/expected");

        write_factory_codes(&root_dir, &output_dir).unwrap();

        assert!(std::fs::read_dir(&output_dir).unwrap().count() > 0);

        for entry in std::fs::read_dir(&output_dir).unwrap() {
            let actual_path = entry.unwrap().path();
            if actual_path.is_dir() {
                continue;
            }
            let relative_path = actual_path.strip_prefix(&output_dir).unwrap();
            let expected_path = expected_dir.join(relative_path);

            assert!(expected_path.exists(), "File {:?} not found", relative_path);
            let actual_content = std::fs::read_to_string(actual_path).unwrap();
            let expected_content = std::fs::read_to_string(expected_path).unwrap();
            assert_eq!(actual_content, expected_content);
        }
    }
}
