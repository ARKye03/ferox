# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

FEROX (Functional Expression Runtime for Operations and eXecution) is an interpreter for a simplified imperative/functional programming language. This is an educational project focused on building an interpreter from scratch using Rust.

## Build and Development Commands

```bash
# Build the project
cargo build

# Run the interpreter (REPL)
cargo run

# Run in release mode
cargo run --release

# Run tests
cargo test

# Run a specific test
cargo test <test_name>

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Architecture

The project must be structured as two separate Rust crates:

1. **Library Crate**: Contains all parsing and evaluation logic for FEROX
   - Must use only Rust standard library (no external dependencies)
   - Implements lexer, parser, AST, and evaluator
   - Handles three error types: Lexical, Syntax, and Semantic

2. **Binary Crate**: The interactive REPL console application
   - May use external dependencies for console interaction
   - Provides the user-facing interpreter interface

### Language Features (Required Subset)

The interpreter must support:

- **Basic expressions**: Print statements, arithmetic (`+`, `-`, `*`, `/`, `^`), mathematical functions (`sin`, `cos`, `log`, etc.)
- **Inline functions**: `function name(params) => expression;` with recursion support
- **Variables**: `let-in` expressions with lexical scoping
- **Conditionals**: `if-else` expressions (both branches required)
- **Types**: `string`, `number`, `boolean` with strong typing

All instructions are single-line expressions ending with `;`.

### REPL Behavior

- Functions persist across expressions once declared
- Functions cannot be redefined
- Expressions with return values print automatically (even without `print`)
- Each input line is evaluated immediately

### Error Handling Requirements

Detect and report exactly one error per expression:

1. **Lexical Error**: Invalid tokens (e.g., `14a`)
2. **Syntax Error**: Malformed expressions, unbalanced parentheses, incomplete statements
3. **Semantic Error**: Type mismatches, wrong argument counts, undefined functions

Error messages must be informative and follow the format: `! ERROR_TYPE: description`

### Implementation Guidance

- **Recursive descent parsing** is recommended for parsing the language
- Focus on a clean type hierarchy for expressions and interpreter concepts
- The language grammar is designed to be implementable without advanced compiler theory
- Variables have lexical scope (only exist within their `let-in` expression)
- The `let-in` expression returns the value of its body

## Multi-Line Support Design

### Extended Syntax

Multi-line `let-in` expressions allow variables to be defined across multiple lines:

```js
let
  a = 42;
  b = sin(a);
  c = b * 2
in print(a + b + c);
```

Key syntax rules:
- Semicolons separate variable declarations (last declaration before `in` may omit semicolon)
- The `in` keyword marks the transition from declarations to body
- The body expression still ends with `;`
- Newlines are allowed anywhere but don't change semantics

### Architecture Components for Multi-Line

#### 1. Lexer/Tokenizer Enhancements

**Position Tracking**:
- Add `Span` or `Location` struct containing:
  - `line: usize` - Line number (1-indexed)
  - `column: usize` - Column number (1-indexed)
  - `offset: usize` - Byte offset in source (optional, useful for slicing)
- Every token should carry its span: `Token { kind: TokenKind, span: Span }`
- Track newlines but treat them as whitespace in most contexts

**Newline Handling**:
- Newlines are generally insignificant (like whitespace)
- However, track them for error reporting
- Consider: Should a newline in the middle of an expression be allowed? (Answer: Yes, for flexibility)

#### 2. Parser Enhancements

**Multi-line `let-in` Parsing**:
```rust
// Pseudocode structure
fn parse_let_in() -> Result<LetInExpr, ParseError> {
    expect_token(Let)?;

    // Parse variable declarations
    let mut declarations = vec![];
    loop {
        let name = expect_identifier()?;
        expect_token(Equals)?;
        let value = parse_expression()?;
        declarations.push((name, value));

        // Check for semicolon or 'in' keyword
        if check_token(Semicolon) {
            consume_token(); // consume semicolon
            if check_token(In) {
                break; // Last declaration before 'in'
            }
        } else if check_token(In) {
            break; // 'in' without semicolon (allowed for last decl)
        } else {
            return syntax_error("Expected ';' or 'in'");
        }
    }

    expect_token(In)?;
    let body = parse_expression()?;
    Ok(LetInExpr { declarations, body })
}
```

**Expression Completeness Detection**:
- Parser needs to detect incomplete expressions (for REPL)
- Add method `is_complete_expression(&str) -> bool` or return specific error type
- Check for:
  - Unmatched parentheses/braces
  - Dangling operators
  - `let` without `in`
  - Missing semicolon at end

#### 3. AST Node Changes

**Span Tracking in AST**:
```rust
// Every expression node should include span
enum Expr {
    Literal { value: Value, span: Span },
    Binary { left: Box<Expr>, op: BinOp, right: Box<Expr>, span: Span },
    LetIn { declarations: Vec<(String, Expr)>, body: Box<Expr>, span: Span },
    // ... other variants
}
```

Benefits:
- Precise error reporting ("Error on line 3, column 5")
- Ability to show code snippets with error markers
- Debugging and source mapping

**Multi-variable `let-in`**:
```rust
struct LetInExpr {
    declarations: Vec<(Identifier, Expr)>, // Changed from single to Vec
    body: Box<Expr>,
    span: Span,
}
```

#### 4. REPL Multi-line Input

**Continuation Prompt**:
```
> let
... a = 42;
... b = 100
... in print(a + b);
142
```

**Implementation Strategy**:
```rust
fn repl_loop() {
    let mut input_buffer = String::new();

    loop {
        let prompt = if input_buffer.is_empty() { "> " } else { "... " };
        print!("{}", prompt);

        let mut line = String::new();
        stdin().read_line(&mut line)?;
        input_buffer.push_str(&line);

        // Try to parse
        match try_parse(&input_buffer) {
            Ok(expr) => {
                // Complete expression, evaluate it
                evaluate_and_print(expr);
                input_buffer.clear();
            },
            Err(ParseError::Incomplete) => {
                // Need more input, continue loop
                continue;
            },
            Err(e) => {
                // Real error, report it
                print_error(e);
                input_buffer.clear();
            }
        }
    }
}
```

**Incomplete Expression Detection**:
- Track unclosed delimiters: `()`, `{}`, `[]`
- Check for `let` without matching `in`
- Check for unterminated strings
- Return `ParseError::Incomplete` vs `ParseError::Syntax`

#### 5. Enhanced Error Reporting

**Rich Error Messages**:
```
! SYNTAX ERROR (line 3, column 12): Expected ';' or 'in' after variable declaration
  |
3 | b = 100 c = 200
  |         ^ unexpected identifier
```

**Error Struct**:
```rust
struct ParseError {
    kind: ErrorKind,  // Lexical, Syntax, Semantic
    message: String,
    span: Span,
    source_snippet: Option<String>,  // Optional code excerpt
}
```

**Display Implementation**:
- Show line number and column
- Optionally show source line with caret (^) pointing to error
- Keep single-error rule (report only first error encountered)

#### 6. Alternative Syntax Considerations

**Comma-separated variables (alternative)**:
```js
let a = 42,
    b = sin(a),
    c = b * 2
in print(a + b + c);
```

This mirrors the existing single-line syntax: `let a = 42, b = 100 in expr`

**Decision needed**: Semicolon-separated vs comma-separated for multi-line
- **Semicolon**: More imperative, clearer separation (recommended)
- **Comma**: Matches existing single-line syntax better

#### 7. Implementation Order

Recommended implementation sequence:

1. **Add Span tracking to lexer** - Foundation for everything else
2. **Update Token and AST nodes** - Include span information
3. **Extend `let-in` parser** - Support Vec of declarations
4. **Add incomplete expression detection** - Parser returns `Incomplete` error
5. **Update REPL** - Implement continuation prompt and buffering
6. **Enhance error reporting** - Use span info for line/column display
7. **Test thoroughly** - Multi-line expressions, edge cases, error conditions

### Testing Strategy for Multi-Line

Test cases to cover:
- Simple multi-line `let-in` with 2-3 variables
- Nested `let-in` expressions across lines
- Multi-line with function calls and complex expressions
- Error cases: missing `in`, unclosed parens, type errors on specific lines
- Mixed single-line and multi-line code
- REPL behavior: incomplete input, multi-line continuation, cancellation
