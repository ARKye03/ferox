use clap::Parser;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(name = "ferox")]
#[command(author = "FEROX Team")]
#[command(version = "0.1.0")]
#[command(about = "FEROX - Functional Expression Runtime for Operations and eXecution", long_about = None)]
struct Cli {
    /// Path to a .frx file to execute
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    match cli.file {
        None => {
            // REPL mode
            run_repl();
        }
        Some(path) => {
            // File execution mode
            run_file(path);
        }
    }
}

fn run_repl() {
    println!("FEROX REPL v0.1.0");
    println!("Type expressions to evaluate. Press Ctrl+C to exit.");
    println!();

    // TODO: Implement REPL loop
    eprintln!("Error: REPL mode not yet implemented");
    process::exit(1);
}

fn run_file(path: PathBuf) {
    // Verify the file has .frx extension
    match path.extension().and_then(|s| s.to_str()) {
        Some("frx") => {
            // TODO: Implement file execution
            println!("Executing file: {}", path.display());
            eprintln!("Error: File execution not yet implemented");
            process::exit(1);
        }
        _ => {
            eprintln!("Error: File must have .frx extension");
            eprintln!("Usage: ferox <file.frx>");
            process::exit(1);
        }
    }
}
