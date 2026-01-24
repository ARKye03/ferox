use std::sync::Mutex;
use ferox_core::{Evaluator, Parser};

#[derive(serde::Serialize)]
#[serde(tag = "type")]
enum EvalResponse {
    Success {
        value: Option<String>,
    },
    Incomplete,
    Error {
        message: String,
        error_type: String,
        line: usize,
        column: usize,
    },
}

struct InterpreterState {
    evaluator: Mutex<Evaluator>,
}

impl InterpreterState {
    fn new() -> Self {
        Self {
            evaluator: Mutex::new(Evaluator::new()),
        }
    }
}

#[tauri::command]
fn eval_code(code: String, state: tauri::State<InterpreterState>) -> EvalResponse {
    if is_obviously_incomplete(&code) {
        return EvalResponse::Incomplete;
    }

    let mut parser = match Parser::from_source(&code) {
        Ok(p) => p,
        Err(e) => {
            let err_msg = e.to_string();
            if err_msg.contains("Unterminated string") {
                return EvalResponse::Incomplete;
            }
            return EvalResponse::Error {
                message: err_msg,
                error_type: "lexical".to_string(),
                line: 1,
                column: 1,
            };
        }
    };

    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            if is_incomplete_error(&e.message) {
                return EvalResponse::Incomplete;
            }
            return EvalResponse::Error {
                message: e.message.clone(),
                error_type: "syntax".to_string(),
                line: e.span.start.line,
                column: e.span.start.column,
            };
        }
    };

    let mut evaluator = state.evaluator.lock().unwrap();
    let mut last_value = None;

    for stmt in &program.statements {
        match evaluator.eval_statement(stmt) {
            Ok(val) => last_value = val,
            Err(e) => {
                return EvalResponse::Error {
                    message: e.message.clone(),
                    error_type: "semantic".to_string(),
                    line: e.span.start.line,
                    column: e.span.start.column,
                };
            }
        }
    }

    EvalResponse::Success {
        value: last_value.map(|v| v.to_display_string())
    }
}

#[tauri::command]
fn reset_interpreter(state: tauri::State<InterpreterState>) {
    let mut evaluator = state.evaluator.lock().unwrap();
    *evaluator = Evaluator::new();
}

fn is_obviously_incomplete(input: &str) -> bool {
    let trimmed = input.trim();
    if trimmed.is_empty() { return false; }

    let open = trimmed.chars().filter(|&c| c == '(').count();
    let close = trimmed.chars().filter(|&c| c == ')').count();
    if open > close { return true; }

    if let Some(pos) = trimmed.find("let") {
        let after = &trimmed[pos..];
        if !after.contains(" in ") && !after.contains("\nin") {
            return true;
        }
    }

    if trimmed.contains("if ") || trimmed.contains("if(") {
        let if_count = trimmed.matches("if ").count() + trimmed.matches("if(").count();
        let else_count = trimmed.matches(" else ").count() + trimmed.matches("\nelse ").count();
        if if_count > else_count { return true; }
    }

    if trimmed.starts_with("function ") {
        if !trimmed.contains("=>") { return true; }
        if !trimmed.ends_with(';') { return true; }
    }

    if trimmed.ends_with("=>") || trimmed.ends_with('+') || trimmed.ends_with('-')
        || trimmed.ends_with('*') || trimmed.ends_with('/') || trimmed.ends_with('@')
        || trimmed.ends_with('^') {
        return true;
    }

    false
}

fn is_incomplete_error(msg: &str) -> bool {
    msg.contains("got end of file")
        || msg.contains("Expected ')' after")
        || msg.contains("Expected 'in'")
        || msg.contains("Expected 'else'")
        || msg.contains("Expected '=>'")
        || (msg.contains("Expected ';'") && msg.contains("end of file"))
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(InterpreterState::new())
        .invoke_handler(tauri::generate_handler![greet, eval_code, reset_interpreter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
