use clap::Parser as ClapParser;
use ferox_core::{Evaluator, Parser, Value};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::path::PathBuf;
use std::process;

#[derive(ClapParser)]
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
            if let Err(e) = run_repl() {
                eprintln!("REPL error: {}", e);
                process::exit(1);
            }
        }
        Some(path) => {
            // File execution mode
            run_file(path);
        }
    }
}

fn run_repl() -> Result<(), Box<dyn std::error::Error>> {
    println!("FEROX REPL v0.1.0");
    println!("Type expressions to evaluate. Press Ctrl+D or Ctrl+C to exit.");
    println!();

    let mut rl = DefaultEditor::new()?;
    let mut evaluator = Evaluator::new();
    let mut input_buffer = String::new();

    loop {
        // Choose prompt based on whether we're continuing input
        let prompt = if input_buffer.is_empty() {
            "> "
        } else {
            "... "
        };

        match rl.readline(prompt) {
            Ok(line) => {
                // Add line to buffer
                if !input_buffer.is_empty() {
                    input_buffer.push('\n');
                }
                input_buffer.push_str(&line);

                // Try to parse and evaluate
                match try_eval(&mut evaluator, &input_buffer) {
                    EvalResult::Success(value) => {
                        // Auto-print expression results (not function definitions)
                        if let Some(val) = value {
                            println!("{}", val.to_display_string());
                        }
                        // Clear buffer after successful execution
                        input_buffer.clear();
                    }
                    EvalResult::Incomplete => {
                        // Need more input, continue loop with continuation prompt
                        continue;
                    }
                    EvalResult::Error(msg) => {
                        eprintln!("{}", msg);
                        // Clear buffer after error
                        input_buffer.clear();
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C - clear current input and continue
                if !input_buffer.is_empty() {
                    println!("^C");
                    input_buffer.clear();
                } else {
                    println!("Use Ctrl+D to exit");
                }
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D - exit
                println!();
                break;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }

    Ok(())
}

enum EvalResult {
    Success(Option<Value>),
    Incomplete,
    Error(String),
}

fn try_eval(evaluator: &mut Evaluator, input: &str) -> EvalResult {
    // Check for obviously incomplete input patterns
    if is_obviously_incomplete(input) {
        return EvalResult::Incomplete;
    }

    // First, try to parse the input
    let mut parser = match Parser::from_source(input) {
        Ok(p) => p,
        Err(e) => {
            // Check if it's a lexical error that suggests incompleteness
            let err_msg = e.to_string();
            if err_msg.contains("Unterminated string") {
                return EvalResult::Incomplete;
            }
            return EvalResult::Error(format!("{}", e));
        }
    };

    // Try to parse the program
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            // Check if the error suggests incomplete input
            if is_incomplete_error(&e.message) {
                return EvalResult::Incomplete;
            }
            return EvalResult::Error(format!("{}", e));
        }
    };

    // Evaluate each statement
    let mut last_value = None;
    for stmt in &program.statements {
        match evaluator.eval_statement(stmt) {
            Ok(val) => last_value = val,
            Err(e) => return EvalResult::Error(format!("{}", e)),
        }
    }

    EvalResult::Success(last_value)
}

fn is_obviously_incomplete(input: &str) -> bool {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return false;
    }

    // Check for unclosed parentheses
    let open_parens = trimmed.chars().filter(|&c| c == '(').count();
    let close_parens = trimmed.chars().filter(|&c| c == ')').count();
    if open_parens > close_parens {
        return true;
    }

    // "let" keyword without "in"
    if let Some(let_pos) = trimmed.find("let") {
        // Check if this "let" has a matching "in"
        let after_let = &trimmed[let_pos..];
        if !after_let.contains(" in ") && !after_let.contains("\nin") {
            return true; // We have "let" but no "in" yet
        }
    }

    // "if" without matching "else"
    if trimmed.contains("if ") || trimmed.contains("if(") {
        let if_count = trimmed.matches("if ").count() + trimmed.matches("if(").count();
        let else_count = trimmed.matches(" else ").count() + trimmed.matches("\nelse ").count();
        if if_count > else_count {
            return true;
        }
    }

    // "function" without "=>" or without ending semicolon
    if trimmed.starts_with("function ") {
        if !trimmed.contains("=>") {
            return true; // No arrow yet
        }
        if !trimmed.ends_with(';') {
            return true; // No terminating semicolon
        }
    }

    // Line ends with "=>" (function body incomplete)
    if trimmed.ends_with("=>") {
        return true;
    }

    // Ends with operator (likely incomplete expression)
    if trimmed.ends_with('+') || trimmed.ends_with('-') || trimmed.ends_with('*')
        || trimmed.ends_with('/') || trimmed.ends_with('@') || trimmed.ends_with('^')
    {
        return true;
    }

    false
}

fn is_incomplete_error(msg: &str) -> bool {
    // Check for common patterns that indicate incomplete input
    msg.contains("got end of file")
        || msg.contains("Expected ')' after")
        || msg.contains("Expected 'in'")
        || msg.contains("Expected 'else'")
        || msg.contains("Expected '=>'")
        || msg.contains("Expected '}' after")
        || (msg.contains("Expected ';'") && msg.contains("end of file"))
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
