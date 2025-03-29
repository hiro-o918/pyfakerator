use std::path::{Path, PathBuf};

use crate::factories::pandera::PanderaHandler;
use anyhow::{Context, Result};
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

fn create_dir_all_with_init(from: &Path, target: &Path) -> Result<()> {
    let init_path = target.join("__init__.py");
    // When the target is the same as the from, we need to break the recursion
    // and create the directory and __init__.py file if required
    if from == target {
        if !target.exists() {
            std::fs::create_dir(target)
                .with_context(|| format!("Failed to create directory: {}", target.display()))?;
        }
        if !init_path.exists() {
            std::fs::write(&init_path, "")?;
        }
        return Ok(());
    }

    if let Some(parent) = target.parent() {
        create_dir_all_with_init(from, parent)?;
    }

    if !target.exists() {
        std::fs::create_dir(target)
            .with_context(|| format!("Failed to create directory: {}", target.display()))?;
    }
    if !init_path.exists() {
        std::fs::write(&init_path, "")?;
    }

    Ok(())
}

pub fn write_factory_codes(module_dir: &Path, output_dir: &Path) -> Result<()> {
    let files = walk_dir(module_dir, "py")?;
    for file in files {
        let Some(factory_code) = render_factory_code_from_file(&file)? else {
            continue;
        };
        let relative_path = file.strip_prefix(module_dir)?.to_path_buf();
        let factory_file = output_dir.join(relative_path).with_extension("py");

        if let Some(parent) = factory_file.parent() {
            create_dir_all_with_init(output_dir, parent)?;
        }
        std::fs::write(factory_file, factory_code)?;
    }
    Ok(())
}

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
        let module_dir = PathBuf::from("./resources/generator/render_factory_code_from_file");
        let file = module_dir.join(input_file);
        let result = render_factory_code_from_file(&file);
        assert!(
            result.is_ok(),
            "Failed to render factory code from file: {:?}",
            result.err()
        );
        let actual_factory_code = result.unwrap();
        let expected_factory_code =
            std::fs::read_to_string(module_dir.join(expected_file)).unwrap();
        assert_eq!(actual_factory_code, Some(expected_factory_code));
    }

    #[rstest]
    #[case("no_render_code.py")]
    fn test_render_factory_code_from_file_should_none_when_target_class_not_found(
        #[case] input_file: &str,
    ) {
        let module_dir = PathBuf::from("./resources/generator/render_factory_code_from_file");
        let file = module_dir.join(input_file);
        let result = render_factory_code_from_file(&file);
        assert!(
            result.is_ok(),
            "Failed to render factory code from file: {:?}",
            result.err()
        );
        let actual_factory_code = result.unwrap();
        assert_eq!(actual_factory_code, None);
    }

    fn get_all_files(dir: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.extend(get_all_files(&path)?);
            } else {
                files.push(path);
            }
        }
        Ok(files)
    }

    #[test]
    fn test_write_factory_codes() {
        let module_dir = PathBuf::from("./resources/generator/write_factory_codes/input");
        let output_dir = TempDir::new().unwrap().path().to_path_buf();
        let expected_dir = PathBuf::from("./resources/generator/write_factory_codes/expected");

        write_factory_codes(&module_dir, &output_dir).unwrap();

        // Get all files from both directories
        let mut actual_files = get_all_files(&output_dir).unwrap();
        let mut expected_files = get_all_files(&expected_dir).unwrap();

        // Sort files to ensure consistent ordering
        actual_files.sort();
        expected_files.sort();

        // Compare the number of files
        assert_eq!(
            actual_files.len(),
            expected_files.len(),
            "Different number of files found. Expected: {:?}, Actual: {:?}",
            expected_files
                .iter()
                .map(|p| p.strip_prefix(&expected_dir).unwrap())
                .collect::<Vec<_>>(),
            actual_files
                .iter()
                .map(|p| p.strip_prefix(&output_dir).unwrap())
                .collect::<Vec<_>>()
        );

        // Compare each file
        for (actual_path, expected_path) in actual_files.iter().zip(expected_files.iter()) {
            let actual_relative = actual_path.strip_prefix(&output_dir).unwrap();
            let expected_relative = expected_path.strip_prefix(&expected_dir).unwrap();

            // Compare relative paths
            assert_eq!(
                actual_relative, expected_relative,
                "File paths don't match: {:?} != {:?}",
                actual_relative, expected_relative
            );

            // Compare file contents
            let actual_content = std::fs::read_to_string(actual_path).unwrap();
            let expected_content = std::fs::read_to_string(expected_path).unwrap();
            assert_eq!(
                actual_content, expected_content,
                "Content mismatch for file: {:?}",
                actual_relative
            );
        }
    }
}
