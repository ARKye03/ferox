# Multi-Line Support

## Overview

Extend the single-line FEROX interpreter to support multi-line expressions and statements.

## Extended Syntax

### Multi-line `let-in` Expression

**Single-line** (original):
```js
let x = 42, y = 100 in print(x + y);
```

**Multi-line** (new):
```js
let
  x = 42;
  y = 100;
  z = x + y
in print(z);
```

### Syntax Rules

1. **Semicolons** separate variable declarations
2. Last declaration before `in` **may omit** the semicolon
3. The `in` keyword transitions from declarations to body
4. Body expression still ends with `;`
5. Newlines are **whitespace** - allowed anywhere, don't change semantics
6. Indentation is **cosmetic** - not significant to parsing

### Multi-line Function Bodies (Future)

Not required for current project, but architecture should support:
```js
function factorial(n) => {
  if (n <= 1)
    1
  else
    n * factorial(n - 1)
};
```

## Implementation Components

### 1. Lexer Changes

#### Add Span Tracking

**Before**:
```rust
pub enum TokenKind {
    Number(f64),
    Identifier(String),
    Plus,
    // ...
}
```

**After**:
```rust
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: usize,    // 1-indexed
    pub column: usize,  // 1-indexed
    pub offset: usize,  // 0-indexed byte offset
}
```

#### Track Newlines

```rust
impl Lexer {
    fn advance(&mut self) -> Option<char> {
        let ch = self.current()?;

        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        self.offset += 1;
        // ...
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current() {
            if ch.is_whitespace() {  // includes '\n', '\t', ' ', etc.
                self.advance();
            } else {
                break;
            }
        }
    }
}
```

### 2. Parser Changes

#### Multi-variable `let-in` Parsing

```rust
fn parse_let_in(&mut self) -> Result<Expr, ParseError> {
    let start = self.current_position();
    self.expect(TokenKind::Let)?;

    let mut declarations = Vec::new();

    // Parse variable declarations
    loop {
        let name = self.expect_identifier()?;
        self.expect(TokenKind::Equals)?;
        let value = self.parse_expression()?;
        declarations.push((name, value));

        // Check what comes next
        match self.peek() {
            Some(TokenKind::Semicolon) => {
                self.advance(); // consume semicolon

                // Is 'in' next?
                if self.check(TokenKind::In) {
                    break; // Semicolon before 'in' - exit loop
                }
                // Otherwise, continue to next declaration
            }
            Some(TokenKind::In) => {
                // No semicolon before 'in' - allowed for last declaration
                break;
            }
            _ => {
                return Err(ParseError::syntax(
                    "Expected ';' or 'in' after variable declaration",
                    self.current_span()
                ));
            }
        }
    }

    self.expect(TokenKind::In)?;
    let body = self.parse_expression()?;

    let end = self.current_position();
    Ok(Expr::LetIn {
        declarations,
        body: Box::new(body),
        span: Span::new(start, end),
    })
}
```

#### Expression Completeness Detection

Used by REPL to determine if more input is needed:

```rust
pub fn is_incomplete(source: &str) -> bool {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(_) => return false,  // Lexical error = not incomplete, just wrong
    };

    // Track unclosed delimiters
    let mut paren_depth = 0;
    let mut seen_let = false;
    let mut seen_in = false;

    for token in tokens {
        match token.kind {
            TokenKind::LeftParen => paren_depth += 1,
            TokenKind::RightParen => paren_depth -= 1,
            TokenKind::Let => {
                seen_let = true;
                seen_in = false;
            }
            TokenKind::In => {
                if seen_let {
                    seen_in = true;
                }
            }
            TokenKind::Semicolon => {
                // Reset after complete statement
                if paren_depth == 0 && seen_in {
                    seen_let = false;
                    seen_in = false;
                }
            }
            _ => {}
        }
    }

    // Incomplete if:
    // - Unclosed parentheses
    // - 'let' without 'in'
    paren_depth > 0 || (seen_let && !seen_in)
}
```

### 3. AST Changes

#### Add Span to All Nodes

**Before**:
```rust
pub enum Expr {
    Number(f64),
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    // ...
}
```

**After**:
```rust
pub enum Expr {
    Number {
        value: f64,
        span: Span,
    },
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
        span: Span,
    },
    LetIn {
        declarations: Vec<(String, Expr)>,  // Changed from single to Vec
        body: Box<Expr>,
        span: Span,
    },
    // ...
}

impl Expr {
    /// Get the span of any expression
    pub fn span(&self) -> Span {
        match self {
            Expr::Number { span, .. } => *span,
            Expr::Binary { span, .. } => *span,
            Expr::LetIn { span, .. } => *span,
            // ...
        }
    }
}
```

### 4. REPL Implementation

#### Continuation Prompt

```rust
pub fn run_repl() {
    let mut interpreter = Interpreter::new();
    let mut input_buffer = String::new();

    println!("FEROX Interpreter v0.1.0");
    println!("Type expressions to evaluate. Press Ctrl+C to exit.\n");

    loop {
        // Prompt based on buffer state
        let prompt = if input_buffer.is_empty() { "> " } else { "... " };
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        // Read line
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,  // EOF
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        }

        // Accumulate input
        input_buffer.push_str(&line);

        // Try to parse
        if interpreter.is_incomplete(&input_buffer) {
            // Need more input
            continue;
        }

        // Complete expression - evaluate
        match interpreter.eval(&input_buffer) {
            Ok(Some(value)) => {
                println!("{}", value);
            }
            Ok(None) => {
                // Definition or statement with no output
            }
            Err(e) => {
                eprintln!("{}", format_error(&e, &input_buffer));
            }
        }

        input_buffer.clear();
    }
}
```

#### Handle Ctrl+C During Multi-line Input

```rust
// Optional: Use ctrlc crate to reset buffer on interrupt
use ctrlc;

fn setup_interrupt_handler(buffer: Arc<Mutex<String>>) {
    ctrlc::set_handler(move || {
        let mut buf = buffer.lock().unwrap();
        if !buf.is_empty() {
            buf.clear();
            println!("\n^C");
        } else {
            std::process::exit(0);
        }
    }).expect("Error setting Ctrl-C handler");
}
```

## Error Reporting with Multi-line

### Enhanced Error Display

```rust
pub fn format_error(error: &Error, source: &str) -> String {
    let (kind, message, span) = match error {
        Error::Lexical(e) => ("LEXICAL ERROR", e.message.as_str(), e.span),
        Error::Syntax(e) => ("SYNTAX ERROR", e.message.as_str(), e.span),
        Error::Semantic(e) => ("SEMANTIC ERROR", e.message.as_str(), e.span),
        Error::Runtime(e) => ("RUNTIME ERROR", e.message.as_str(), e.span),
    };

    let lines: Vec<&str> = source.lines().collect();
    let error_line = lines.get(span.start.line - 1);

    let mut output = format!("! {} (line {}, column {}): {}\n",
        kind, span.start.line, span.start.column, message);

    // Show source line with error marker
    if let Some(line) = error_line {
        output.push_str(&format!("  |\n"));
        output.push_str(&format!("{} | {}\n", span.start.line, line));
        output.push_str(&format!("  | {}^\n", " ".repeat(span.start.column - 1)));
    }

    output
}
```

### Example Error Output

```
> let
... a = 42;
... b = "text" + a
... in print(b);
! SEMANTIC ERROR (line 3, column 12): Operator '+' cannot be used between 'string' and 'number'
  |
3 | b = "text" + a
  |            ^
```

## Testing Multi-line Features

### Test Cases

1. **Simple multi-line let-in**:
   ```js
   let
     a = 10;
     b = 20
   in print(a + b);
   ```
   Expected: `30`

2. **Nested let-in**:
   ```js
   let
     x = 10
   in (let
         y = 20
       in x + y);
   ```
   Expected: `30`

3. **Incomplete input detection**:
   ```js
   let
     a = 10;
   ```
   Expected: Continue with `...` prompt

4. **Syntax error in multi-line**:
   ```js
   let
     a = 10
     b = 20
   in print(a);
   ```
   Expected: Error on line 3 (missing semicolon or 'in')

5. **Semantic error with line tracking**:
   ```js
   let
     a = "hello";
     b = 42
   in print(a + b);
   ```
   Expected: Semantic error on line 4 (string + number)
