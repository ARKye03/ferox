# Error Handling Strategy

## Error Types

FEROX has four error types, detected in order:

1. **Lexical Error** - Invalid tokens in source
2. **Syntax Error** - Malformed expressions/statements
3. **Semantic Error** - Type mismatches, undefined references
4. **Runtime Error** - Division by zero, stack overflow, etc.

## Single Error Rule

**CRITICAL**: Detect and report **only the first error** encountered.

- Stop processing immediately upon finding an error
- Do not attempt error recovery
- Do not collect multiple errors

## Error Structure

### Error Type Definitions

```rust
// src/error.rs

use crate::token::Span;

#[derive(Debug, Clone)]
pub enum Error {
    Lexical(LexicalError),
    Syntax(SyntaxError),
    Semantic(SemanticError),
    Runtime(RuntimeError),
}

#[derive(Debug, Clone)]
pub struct LexicalError {
    pub message: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct SyntaxError {
    pub message: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct SemanticError {
    pub message: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub span: Span,
}

impl Error {
    pub fn lexical(message: impl Into<String>, span: Span) -> Self {
        Error::Lexical(LexicalError {
            message: message.into(),
            span,
        })
    }

    pub fn syntax(message: impl Into<String>, span: Span) -> Self {
        Error::Syntax(SyntaxError {
            message: message.into(),
            span,
        })
    }

    pub fn semantic(message: impl Into<String>, span: Span) -> Self {
        Error::Semantic(SemanticError {
            message: message.into(),
            span,
        })
    }

    pub fn runtime(message: impl Into<String>, span: Span) -> Self {
        Error::Runtime(RuntimeError {
            message: message.into(),
            span,
        })
    }
}
```

## Error Detection by Phase

### 1. Lexical Errors (Lexer)

Detected during tokenization.

**Examples**:
- Invalid characters: `@#$` (where not expected)
- Malformed numbers: `3.14.159`, `1e`, `0xGHI`
- Malformed identifiers: `14abc` (starts with digit)
- Unterminated strings: `"hello world` (missing closing quote)

**Implementation**:
```rust
impl Lexer {
    fn scan_number(&mut self) -> Result<Token, LexicalError> {
        let start = self.position();
        let mut has_dot = false;

        while let Some(ch) = self.current() {
            if ch.is_ascii_digit() {
                self.advance();
            } else if ch == '.' && !has_dot {
                has_dot = true;
                self.advance();

                // Must have digit after dot
                if !self.current().map_or(false, |c| c.is_ascii_digit()) {
                    return Err(LexicalError {
                        message: "Expected digit after decimal point".to_string(),
                        span: Span::new(start, self.position()),
                    });
                }
            } else {
                break;
            }
        }

        let end = self.position();
        let text = &self.source[start.offset..end.offset];
        let value = text.parse::<f64>().unwrap();

        Ok(Token {
            kind: TokenKind::Number(value),
            span: Span::new(start, end),
        })
    }
}
```

### 2. Syntax Errors (Parser)

Detected during parsing (tokens → AST).

**Examples**:
- Unexpected token: `let x = in print(x);` (missing expression)
- Missing token: `print(42` (missing closing paren)
- Invalid structure: `let x = 5 inn print(x);` (typo in keyword)
- Incomplete expression: `2 + * 3` (operator without operand)

**Implementation**:
```rust
impl Parser {
    fn expect(&mut self, expected: TokenKind) -> Result<Token, ParseError> {
        match self.current() {
            Some(token) if token.kind == expected => {
                Ok(self.advance().unwrap())
            }
            Some(token) => {
                Err(ParseError::syntax(
                    format!("Expected {:?}, found {:?}", expected, token.kind),
                    token.span
                ))
            }
            None => {
                Err(ParseError::syntax(
                    format!("Expected {:?}, found end of input", expected),
                    self.last_span()
                ))
            }
        }
    }
}
```

### 3. Semantic Errors (Type Checker / Evaluator)

Detected during semantic analysis or evaluation.

**Examples**:
- Type mismatch: `"hello" + 5` (string + number)
- Undefined function: `unknownFunc(42)`
- Undefined variable: `print(x)` (x not defined)
- Wrong argument count: `fib(1, 2, 3)` (fib expects 1 arg)
- Wrong argument type: `fib("hello")` (fib expects number)
- Function redefinition: defining `fib` twice
- Parameter name conflict: `function test(x, x) => x;` (duplicate param)

**Implementation**:
```rust
impl Evaluator {
    fn eval_binary(
        &self,
        left: &Expr,
        op: BinOp,
        right: &Expr,
        span: Span
    ) -> Result<Value, SemanticError> {
        let left_val = self.eval_expr(left)?;
        let right_val = self.eval_expr(right)?;

        match (left_val, op, right_val) {
            (Value::Number(l), BinOp::Plus, Value::Number(r)) => {
                Ok(Value::Number(l + r))
            }
            (Value::String(l), BinOp::Concat, Value::String(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            (left, op, right) => {
                Err(SemanticError {
                    message: format!(
                        "Operator '{}' cannot be used between '{}' and '{}'",
                        op.symbol(),
                        left.type_name(),
                        right.type_name()
                    ),
                    span,
                })
            }
        }
    }

    fn eval_call(
        &mut self,
        name: &str,
        args: &[Expr],
        span: Span
    ) -> Result<Value, SemanticError> {
        // Get function definition
        let func = self.env.get_function(name)
            .ok_or_else(|| SemanticError {
                message: format!("Undefined function '{}'", name),
                span,
            })?;

        // Check argument count
        if args.len() != func.params.len() {
            return Err(SemanticError {
                message: format!(
                    "Function '{}' expects {} argument(s), but {} were given",
                    name, func.params.len(), args.len()
                ),
                span,
            });
        }

        // Evaluate arguments and check types
        let mut arg_values = Vec::new();
        for (arg_expr, param_name) in args.iter().zip(&func.params) {
            let arg_val = self.eval_expr(arg_expr)?;

            // If function has type annotations (future feature), check here
            // For now, we check types during operation execution

            arg_values.push(arg_val);
        }

        // Execute function body
        // ...
    }
}
```

### 4. Runtime Errors (Evaluator)

Detected during execution.

**Examples**:
- Division by zero: `10 / 0`
- Mathematical domain errors: `log(0, 10)`, `sqrt(-1)` (if implemented)
- Stack overflow: infinite recursion
- Numeric overflow: `2 ^ 10000` (if it matters)

**Implementation**:
```rust
impl Evaluator {
    fn eval_binary(
        &self,
        left: &Expr,
        op: BinOp,
        right: &Expr,
        span: Span
    ) -> Result<Value, RuntimeError> {
        let left_val = self.eval_expr(left)?;
        let right_val = self.eval_expr(right)?;

        match (left_val, op, right_val) {
            (Value::Number(l), BinOp::Divide, Value::Number(r)) => {
                if r == 0.0 {
                    return Err(RuntimeError {
                        message: "Division by zero".to_string(),
                        span,
                    });
                }
                Ok(Value::Number(l / r))
            }
            // ... other operations
        }
    }
}
```

## Error Message Format

### REPL Mode

Simple format without filename:
```
! ERROR_TYPE (line X, column Y): message
  |
X | source line here
  |    ^
```

### File Mode

Include filename:
```
! ERROR_TYPE (filename.frx:line:column): message
  |
X | source line here
  |    ^
```

### Error Formatting Function

```rust
pub fn format_error(error: &Error, source: &str, filename: Option<&str>) -> String {
    let (kind, message, span) = match error {
        Error::Lexical(e) => ("LEXICAL ERROR", &e.message, e.span),
        Error::Syntax(e) => ("SYNTAX ERROR", &e.message, e.span),
        Error::Semantic(e) => ("SEMANTIC ERROR", &e.message, e.span),
        Error::Runtime(e) => ("RUNTIME ERROR", &e.message, e.span),
    };

    let location = if let Some(fname) = filename {
        format!("{}:{}:{}", fname, span.start.line, span.start.column)
    } else {
        format!("line {}, column {}", span.start.line, span.start.column)
    };

    let mut output = format!("! {} ({}): {}\n", kind, location, message);

    // Add source line with caret
    if let Some(source_line) = get_line(source, span.start.line) {
        output.push_str("  |\n");
        output.push_str(&format!("{} | {}\n", span.start.line, source_line));

        let caret_padding = " ".repeat(span.start.column.saturating_sub(1));
        output.push_str(&format!("  | {}^\n", caret_padding));
    }

    output
}

fn get_line(source: &str, line_number: usize) -> Option<&str> {
    source.lines().nth(line_number.saturating_sub(1))
}
```

## Error Message Guidelines

### Be Specific and Helpful

**Bad**: "Parse error"
**Good**: "Expected ';' or 'in' after variable declaration"

**Bad**: "Type error"
**Good**: "Operator '+' cannot be used between 'string' and 'number'"

### Include Context

When reporting errors:
1. State what was expected
2. State what was found
3. Suggest a fix if obvious

**Example**:
```
Expected ';' after expression, found 'let'
Did you forget a semicolon?
```

### Use Consistent Terminology

- "argument" not "parameter" in error messages (user perspective)
- "function" not "procedure" or "subroutine"
- "expression" not "expr"
- Type names: `string`, `number`, `boolean` (lowercase, matching language)

## Testing Error Handling

### Lexical Error Tests

```rust
#[test]
fn test_lexical_error_invalid_identifier() {
    let source = "let 14a = 5 in print(14a);";
    let result = Interpreter::new().eval(source);
    assert!(matches!(result, Err(Error::Lexical(_))));
}

#[test]
fn test_lexical_error_unterminated_string() {
    let source = "print(\"hello);";
    let result = Interpreter::new().eval(source);
    assert!(matches!(result, Err(Error::Lexical(_))));
}
```

### Syntax Error Tests

```rust
#[test]
fn test_syntax_error_missing_semicolon() {
    let source = "print(42)";
    let result = Interpreter::new().eval(source);
    assert!(matches!(result, Err(Error::Syntax(_))));
}

#[test]
fn test_syntax_error_unbalanced_parens() {
    let source = "print(42;";
    let result = Interpreter::new().eval(source);
    assert!(matches!(result, Err(Error::Syntax(_))));
}
```

### Semantic Error Tests

```rust
#[test]
fn test_semantic_error_type_mismatch() {
    let source = "print(\"hello\" + 5);";
    let result = Interpreter::new().eval(source);
    assert!(matches!(result, Err(Error::Semantic(_))));
}

#[test]
fn test_semantic_error_undefined_function() {
    let source = "print(unknownFunc(42));";
    let result = Interpreter::new().eval(source);
    assert!(matches!(result, Err(Error::Semantic(_))));
}

#[test]
fn test_semantic_error_wrong_arg_count() {
    let mut interp = Interpreter::new();
    interp.eval("function double(x) => x * 2;").unwrap();
    let result = interp.eval("print(double(1, 2));");
    assert!(matches!(result, Err(Error::Semantic(_))));
}
```

### Runtime Error Tests

```rust
#[test]
fn test_runtime_error_division_by_zero() {
    let source = "print(10 / 0);";
    let result = Interpreter::new().eval(source);
    assert!(matches!(result, Err(Error::Runtime(_))));
}
```
