# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**FEROX** (Functional Expression Runtime for Operations and eXecution) is an interpreter for a simplified imperative/functional programming language. This is an educational project focused on building an interpreter from scratch using Rust.

## Conding Guidelines

- Do comments sparingly, only comment complex code

The interpreter supports:
- Interactive REPL mode
- File execution mode (.frx files)
- Strong static typing (number, string, boolean)
- Functions with recursion
- Lexical scoping with let-in expressions
- Comprehensive error reporting

## Quick Start

```bash
# Build the project
cargo build

# Run REPL (interactive mode)
cargo run

# Execute a .frx file
cargo run examples/fibonacci.frx

# Run tests
cargo test

# Run a specific test
cargo test <test_name>

# Format and lint
cargo fmt
cargo clippy
```

## Project Structure

```
ferox/
├── src/
│   ├── lib.rs          # Library crate entry point
│   ├── main.rs         # Binary crate (CLI & REPL)
│   ├── lexer.rs        # Tokenization
│   ├── parser.rs       # Parsing
│   ├── ast.rs          # AST definitions
│   ├── evaluator.rs    # Evaluation engine
│   ├── environment.rs  # Scope management
│   ├── error.rs        # Error types
│   └── ...
├── examples/           # Example .frx files
├── .claude/            # Detailed design documentation
└── Cargo.toml
```

## Documentation Structure

This CLAUDE.md provides a high-level overview. **Detailed design documentation is in `.claude/` directory:**

### Core Design Documents

- **[.claude/architecture.md](.claude/architecture.md)** - Complete architecture overview
  - Two-crate structure (library + binary)
  - Module organization and responsibilities
  - Data flow diagrams (REPL vs File mode)
  - Environment and scope management
  - Public API design
  - Memory and ownership patterns

- **[.claude/language-spec.md](.claude/language-spec.md)** - FEROX language specification
  - Type system (number, string, boolean)
  - All operators and precedence rules
  - Expression and statement syntax
  - Built-in functions and constants
  - Formal grammar
  - Example programs

- **[.claude/multiline.md](.claude/multiline.md)** - Multi-line support implementation
  - Extended syntax for multi-line let-in
  - Span/position tracking in lexer
  - Parser changes for multi-line expressions
  - AST modifications with span information
  - REPL continuation prompt implementation
  - Incomplete expression detection
  - Enhanced error reporting with line/column

- **[.claude/file-execution.md](.claude/file-execution.md)** - File execution mode (.frx files)
  - Command-line interface design
  - File vs REPL execution differences
  - File format specification
  - Binary crate architecture
  - Statement vs Expression distinction
  - Error reporting with file context
  - Example .frx files

- **[.claude/error-handling.md](.claude/error-handling.md)** - Error handling strategy
  - Four error types: Lexical, Syntax, Semantic, Runtime
  - Error structure definitions
  - Detection phase for each error type
  - Error message formatting (REPL vs File mode)
  - Single-error rule enforcement
  - Error message guidelines
  - Comprehensive test cases

## Key Implementation Principles

### 1. Two-Crate Architecture

**Library Crate** (`src/lib.rs`):
- Contains all parsing and evaluation logic
- **NO external dependencies** (only Rust std)
- Independently testable
- Exposes clean public API

**Binary Crate** (`src/main.rs`):
- REPL and file execution interface
- May use external dependencies (rustyline, clap, colored)
- Handles user interaction and pretty printing

### 2. Execution Modes

**REPL Mode** (no arguments):
- Interactive line-by-line evaluation
- Continuation prompts (`>` and `...`)
- Auto-prints expression results
- Continues after errors

**File Mode** (with .frx file path):
- Executes entire file sequentially
- No auto-print (only explicit `print()` outputs)
- Exits immediately on first error
- Error messages include filename:line:column

### 3. Error Handling

**Single Error Rule**: Report only the first error, then stop.

Error detection order:
1. **Lexical** (invalid tokens) → during tokenization
2. **Syntax** (malformed expressions) → during parsing
3. **Semantic** (type errors, undefined refs) → during evaluation
4. **Runtime** (division by zero, etc.) → during execution

See [.claude/error-handling.md](.claude/error-handling.md) for complete strategy.

### 4. Language Features

Core required features:
- Basic expressions: arithmetic, comparison, logical
- Inline functions: `function name(params) => body;`
- Variables: `let x = value in expression`
- Multi-line let-in: variables across multiple lines
- Conditionals: `if (cond) expr else expr`
- Recursion support
- Strong typing with semantic checks

See [.claude/language-spec.md](.claude/language-spec.md) for full language specification.

## Implementation Roadmap

When implementing features, follow this order:

### Phase 1: Single-Line Interpreter
1. Implement lexer with basic tokenization
2. Build parser for single-line expressions
3. Create evaluator with environment management
4. Add basic error handling
5. Implement simple REPL

### Phase 2: Multi-Line Support
1. Add span tracking to lexer (see [.claude/multiline.md](.claude/multiline.md))
2. Update AST nodes with span information
3. Extend let-in parser for multiple declarations
4. Implement incomplete expression detection
5. Update REPL with continuation prompts
6. Enhance error reporting with line/column

### Phase 3: File Execution
1. Add Statement type (Expression vs FunctionDef)
2. Implement `parse_program()` for entire files
3. Update main.rs with CLI argument handling
4. Implement file reading and execution
5. Add filename context to error messages
6. Create example .frx files

### Phase 4: Polish
1. Comprehensive testing (lexer, parser, evaluator, integration)
2. Improve error messages for clarity
3. Add comments and documentation
4. Performance optimization if needed

## Critical Constraints

### Library Crate Rules
- ✅ Only Rust standard library
- ❌ No external crates (no regex, no nom, no logos)
- ✅ Implement lexer and parser from scratch
- ✅ Recommended: Recursive descent parsing

### Language Constraints
- Functions cannot be redefined (enforce at definition time)
- Variables are lexically scoped (only exist in let-in body)
- All instructions end with `;`
- Both if-else branches required
- Strong typing (no implicit conversions except @ operator)

## Testing Strategy

Create tests for each layer:

**Lexer tests**: Tokenization of all constructs, error cases
**Parser tests**: Each expression type, nesting, errors
**Evaluator tests**: Correct values, type checking, scoping
**Integration tests**: Complete programs, REPL scenarios, .frx files
**Error tests**: One test per error type, verify messages

Example test structure:
```rust
#[test]
fn test_fibonacci_recursion() {
    let mut interp = Interpreter::new();
    interp.eval("function fib(n) => if (n > 1) fib(n-1) + fib(n-2) else 1;").unwrap();
    let result = interp.eval("fib(10);").unwrap();
    assert_eq!(result, Some(Value::Number(89.0)));
}
```

## Working with This Codebase

### Before Making Changes
1. Read relevant documentation in `.claude/` directory
2. Understand the two-crate architecture
3. Review error handling strategy
4. Check language specification for semantics

### When Adding Features
1. Update tests first (TDD approach recommended)
2. Follow existing code patterns
3. Update `.claude/` docs if architecture changes
4. Ensure error messages are clear and helpful

### When Debugging
1. Check which phase detects the error (lexer/parser/evaluator)
2. Verify span tracking is correct
3. Test in both REPL and file modes
4. Ensure single-error rule is maintained

## Additional Resources

For detailed information, always refer to the `.claude/` directory:
- Architecture questions → `.claude/architecture.md`
- Language syntax/semantics → `.claude/language-spec.md`
- Multi-line implementation → `.claude/multiline.md`
- File execution → `.claude/file-execution.md`
- Error handling → `.claude/error-handling.md`

These files contain comprehensive implementation details, code examples, and design rationale.
