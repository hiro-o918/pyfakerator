use clap::{Parser, Subcommand};
use fakerator_core::generator::write_factory_codes;
use std::path::PathBuf;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "fakerator")]
#[command(about = "Python code generator to construct fake data for testing", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate factory codes from a directory of Python files
    Gen {
        /// The root directory containing Python files to process
        #[arg(long, required = true)]
        module_dir: PathBuf,
        /// The output directory where generated factory codes will be saved
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Gen {
            module_dir,
            output_dir,
        } => {
            if !module_dir.exists() {
                eprintln!("module dir does not exist: {}", module_dir.display());
                std::process::exit(1);
            }
            let output_dir = output_dir.unwrap_or_else(|| {
                let mut path = module_dir.clone();
                path.push("testing/fakerator/");
                path
            });
            if let Err(e) = write_factory_codes(&module_dir, &output_dir) {
                eprintln!("Error generating factory codes: {}", e);
            }
            println!(
                "Factory codes generated successfully in {}",
                output_dir.display()
            );
        }
    }
}
