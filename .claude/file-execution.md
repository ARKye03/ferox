# File Execution Mode (.frx Files)

## Overview

The interpreter supports two execution modes:
1. **REPL Mode**: Interactive, no arguments
2. **File Mode**: Execute .frx source files

## Command-Line Interface

```bash
# REPL mode
cargo run
./ferox

# File mode
cargo run script.frx
cargo run examples/fibonacci.frx
./ferox script.frx
```

### Argument Handling in main.rs

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            // No arguments - start REPL
            repl::run();
        }
        2 => {
            // One argument - execute file
            let file_path = &args[1];
            if let Err(e) = file_runner::run_file(file_path) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        _ => {
            // Invalid arguments
            eprintln!("Usage: ferox [script.frx]");
            eprintln!("  No arguments: Start interactive REPL");
            eprintln!("  script.frx: Execute FEROX script file");
            std::process::exit(1);
        }
    }
}
```

## File Format

### .frx Extension

All FEROX source files use `.frx` extension.

### File Structure

A .frx file contains a sequence of statements:
- Function definitions
- Expressions (evaluated sequentially)
- Each statement ends with `;`

Example `fibonacci.frx`:
```js
function fib(n) => if (n > 1) fib(n-1) + fib(n-2) else 1;

print("Fibonacci sequence:");
print(fib(1));
print(fib(5));
print(fib(10));

let
  x = 15;
  result = fib(x)
in print("fib(" @ x @ ") = " @ result);
```

## File vs REPL Execution Differences

| Feature | REPL Mode | File Mode |
|---------|-----------|-----------|
| **Input** | Line-by-line with continuation | Entire file at once |
| **Prompt** | `>` and `...` | None |
| **Auto-print expressions** | Yes (all non-definition values) | **No** (only explicit `print`) |
| **Error handling** | Show error, continue | Show error, **exit immediately** |
| **Incomplete expressions** | Wait for more input | **Syntax error** |
| **Function persistence** | Yes, across prompts | Yes, across statements |

### Critical Difference: Auto-print

**REPL**:
```
> 42 + 58;
100
> fib(5);
8
```

**File** (same code):
```js
42 + 58;  // No output
fib(5);   // No output
```

**File with print**:
```js
print(42 + 58);  // Output: 100
print(fib(5));   // Output: 8
```

## Implementation

### File Runner (src/file_runner.rs or in main.rs)

```rust
use std::fs;
use std::io;
use ferox_lib::{Interpreter, Error};

pub fn run_file(path: &str) -> Result<(), String> {
    // 1. Validate file extension
    if !path.ends_with(".frx") {
        return Err(format!(
            "Error: File must have .frx extension, got: {}",
            path
        ));
    }

    // 2. Read file contents
    let source = fs::read_to_string(path)
        .map_err(|e| format!("Error reading file '{}': {}", path, e))?;

    // 3. Create interpreter
    let mut interpreter = Interpreter::new();

    // 4. Parse entire program
    let program = match interpreter.parse_program(&source) {
        Ok(prog) => prog,
        Err(e) => {
            let error_msg = format_file_error(path, &source, &e.into());
            return Err(error_msg);
        }
    };

    // 5. Execute each statement sequentially
    for (index, statement) in program.statements.iter().enumerate() {
        match interpreter.execute(statement.clone()) {
            Ok(Some(value)) => {
                // Only print if statement explicitly outputs (i.e., print function)
                println!("{}", value);
            }
            Ok(None) => {
                // Statement executed successfully, no output
            }
            Err(e) => {
                let error_msg = format_file_error(path, &source, &e);
                return Err(error_msg);
            }
        }
    }

    Ok(())
}

fn format_file_error(filename: &str, source: &str, error: &Error) -> String {
    let (kind, message, span) = match error {
        Error::Lexical(e) => ("LEXICAL ERROR", &e.message, e.span),
        Error::Syntax(e) => ("SYNTAX ERROR", &e.message, e.span),
        Error::Semantic(e) => ("SEMANTIC ERROR", &e.message, e.span),
        Error::Runtime(e) => ("RUNTIME ERROR", &e.message, e.span),
    };

    let lines: Vec<&str> = source.lines().collect();
    let error_line = lines.get(span.start.line.saturating_sub(1));

    let mut output = format!(
        "! {} ({}:{}:{}): {}\n",
        kind, filename, span.start.line, span.start.column, message
    );

    // Show source context
    if let Some(line) = error_line {
        output.push_str("  |\n");
        output.push_str(&format!("{} | {}\n", span.start.line, line));
        output.push_str(&format!("  | {}^\n", " ".repeat(span.start.column.saturating_sub(1))));
    }

    output
}
```

## Library API for File Mode

### Program and Statement Types

```rust
// In src/ast.rs

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    /// Expression statement (e.g., print(x); or let x = 5 in x;)
    Expression(Expr),

    /// Function definition
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Expr,
        span: Span,
    },
}
```

### Interpreter Methods

```rust
// In src/lib.rs

impl Interpreter {
    /// Parse a complete program (for file mode)
    pub fn parse_program(&self, source: &str) -> Result<Program, ParseError> {
        let tokens = Lexer::new(source).tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse_program()
    }

    /// Execute a single statement (for file mode)
    pub fn execute(&mut self, stmt: Statement) -> Result<Option<Value>, RuntimeError> {
        match stmt {
            Statement::Expression(expr) => {
                let value = self.eval_expr(&expr)?;

                // Only return value if it's from an explicit output expression
                // In practice, check if expr is a Call to 'print' function
                match expr {
                    Expr::Call { function, .. } if function == "print" => {
                        Ok(Some(value))
                    }
                    _ => Ok(None)  // Expression evaluated, but no output
                }
            }
            Statement::FunctionDef { name, params, body, span } => {
                self.environment.define_function(name, params, body, span)?;
                Ok(None)  // No output for definitions
            }
        }
    }
}
```

### Parser::parse_program Implementation

```rust
// In src/parser.rs

impl Parser {
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        // Check if it's a function definition
        if self.check(TokenKind::Function) {
            self.parse_function_def()
        } else {
            // Otherwise, it's an expression statement
            let expr = self.parse_expression()?;
            self.expect(TokenKind::Semicolon)?;
            Ok(Statement::Expression(expr))
        }
    }

    fn parse_function_def(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_position();

        self.expect(TokenKind::Function)?;
        let name = self.expect_identifier()?;

        self.expect(TokenKind::LeftParen)?;
        let params = self.parse_parameter_list()?;
        self.expect(TokenKind::RightParen)?;

        self.expect(TokenKind::Arrow)?;  // =>

        let body = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;

        let end = self.current_position();

        Ok(Statement::FunctionDef {
            name,
            params,
            body,
            span: Span::new(start, end),
        })
    }
}
```

## Error Handling in File Mode

### Error Examples

**Lexical Error**:
```
! LEXICAL ERROR (test.frx:5:12): Invalid token '14a'
  |
5 | let x = 14a in print(x);
  |            ^
```

**Syntax Error**:
```
! SYNTAX ERROR (test.frx:3:1): Expected ';' after expression
  |
3 | print(42)
  | ^
```

**Semantic Error**:
```
! SEMANTIC ERROR (fibonacci.frx:10:20): Operator '+' cannot be used between 'string' and 'number'
  |
10 | print("Result: " + fib(5));
   |                    ^
```

**Runtime Error** (e.g., division by zero):
```
! RUNTIME ERROR (math.frx:7:15): Division by zero
  |
7 | let x = 10 / 0 in print(x);
  |               ^
```

### Exit Codes

```rust
// In main.rs

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            repl::run();
            std::process::exit(0);  // Normal exit
        }
        2 => {
            match file_runner::run_file(&args[1]) {
                Ok(()) => std::process::exit(0),   // Success
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);  // Error exit
                }
            }
        }
        _ => {
            eprintln!("Usage: ferox [script.frx]");
            std::process::exit(1);  // Usage error
        }
    }
}
```

## Example .frx Files

Create an `examples/` directory with test files:

### examples/hello.frx
```js
print("Hello from FEROX!");
```

### examples/arithmetic.frx
```js
print("Basic arithmetic:");
print(2 + 3);
print(10 - 4);
print(5 * 6);
print(20 / 4);
print(2 ^ 8);
```

### examples/functions.frx
```js
function square(x) => x * x;
function cube(x) => x * square(x);

print("Square of 5: " @ square(5));
print("Cube of 3: " @ cube(3));
```

### examples/fibonacci.frx
```js
function fib(n) => if (n > 1) fib(n-1) + fib(n-2) else 1;

print("Fibonacci sequence:");
print(fib(1));
print(fib(5));
print(fib(10));
print(fib(15));
```

### examples/let_in.frx
```js
let
  radius = 5;
  pi = 3.14159;
  area = pi * (radius ^ 2)
in print("Area of circle with radius " @ radius @ " is " @ area);
```

### examples/error.frx
```js
function double(x) => x * 2;

print(double(21));
print(double("not a number"));  // Should error here
print("This won't execute");
```

## Testing File Execution

```bash
# Should all succeed
cargo run examples/hello.frx
cargo run examples/arithmetic.frx
cargo run examples/functions.frx
cargo run examples/fibonacci.frx
cargo run examples/let_in.frx

# Should fail with semantic error
cargo run examples/error.frx

# Should fail with file not found
cargo run nonexistent.frx

# Should fail with invalid extension
cargo run examples/hello.txt
```
