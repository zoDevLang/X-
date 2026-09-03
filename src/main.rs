mod lexer;
mod parser;
mod ast;
mod compiler;

use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "xp")]
#[command(about = "X+ (XP) Programming Language Compiler", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run an XP program
    Run {
        file: PathBuf,
    },
    /// Check syntax without running
    Check {
        file: PathBuf,
    },
    /// Build an XP program
    Build {
        file: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Format XP code
    Fmt {
        file: PathBuf,
    },
    /// Show version
    Version,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file } => {
            let source = fs::read_to_string(&file)?;
            let tokens = lexer::lex(&source)?;
            let ast = parser::parse(tokens)?;
            compiler::compile_and_run(ast)?;
        }
        Commands::Check { file } => {
            let source = fs::read_to_string(&file)?;
            let tokens = lexer::lex(&source)?;
            let _ast = parser::parse(tokens)?;
            println!("✓ Syntax check passed");
        }
        Commands::Build { file, output } => {
            let source = fs::read_to_string(&file)?;
            let tokens = lexer::lex(&source)?;
            let ast = parser::parse(tokens)?;
            let output_file = output.unwrap_or_else(|| PathBuf::from("a.out"));
            compiler::compile_to_file(ast, &output_file)?;
            println!("✓ Built: {}", output_file.display());
        }
        Commands::Fmt { file } => {
            let source = fs::read_to_string(&file)?;
            let tokens = lexer::lex(&source)?;
            let ast = parser::parse(tokens)?;
            let formatted = ast.format();
            fs::write(&file, formatted)?;
            println!("✓ Formatted: {}", file.display());
        }
        Commands::Version => {
            println!("xp {}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
