# Architecture

## Crate Structure

The project consists of two Rust crates:

### 1. Library Crate (ferox_lib)

**Location**: `src/lib.rs` (create this file)

**Purpose**: Core interpreter logic for FEROX language

**Constraints**:
- **NO external dependencies** - Only Rust standard library allowed
- All parsing and evaluation logic lives here
- Should be testable independently of the binary crate

**Main Components**:

```
src/
├── lib.rs              # Public API exports
├── lexer.rs            # Tokenization (source → tokens)
├── token.rs            # Token types and Span tracking
├── parser.rs           # Parsing (tokens → AST)
├── ast.rs              # Abstract Syntax Tree definitions
├── evaluator.rs        # Expression/statement evaluation
├── environment.rs      # Variable and function scope management
├── value.rs            # Runtime value types (Number, String, Boolean)
├── error.rs            # Error types (Lexical, Syntax, Semantic)
└── builtins.rs         # Built-in functions (sin, cos, print, etc.)
```

**Public API** (exported from `lib.rs`):

```rust
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    /// Create new interpreter with built-in functions
    pub fn new() -> Self;

    /// REPL mode: evaluate single expression/statement
    /// Returns Some(value) if expression produces output
    pub fn eval(&mut self, source: &str) -> Result<Option<Value>, Error>;

    /// File mode: parse entire program
    pub fn parse_program(&self, source: &str) -> Result<Program, ParseError>;

    /// File mode: execute single statement
    pub fn execute(&mut self, stmt: Statement) -> Result<Option<Value>, RuntimeError>;

    /// REPL mode: check if input is incomplete (needs more lines)
    pub fn is_incomplete(&self, source: &str) -> bool;
}

pub struct Program {
    pub statements: Vec<Statement>,
}

pub enum Statement {
    Expression(Expr),
    FunctionDef { name: String, params: Vec<String>, body: Expr, span: Span },
}

pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Unit,  // For functions/expressions that don't return a value
}

pub enum Error {
    Lexical(LexicalError),
    Syntax(SyntaxError),
    Semantic(SemanticError),
    Runtime(RuntimeError),
}
```

### 2. Binary Crate (ferox)

**Location**: `src/main.rs`

**Purpose**: Interactive REPL and file execution interface

**Allowed**: Can use external dependencies for:
- Better REPL experience (e.g., `rustyline` for line editing, history)
- Command-line argument parsing (e.g., `clap`)
- Terminal colors (e.g., `colored` for error highlighting)

**Responsibilities**:
- Command-line argument parsing
- REPL loop with continuation prompts
- File reading and execution
- Pretty error formatting and display
- User interaction

**Structure**:

```
src/
├── main.rs           # Entry point, CLI argument handling
├── repl.rs           # Interactive REPL implementation (optional separate file)
└── file_runner.rs    # File execution logic (optional separate file)
```

## Data Flow

### REPL Mode Flow

```
User Input (stdin)
    ↓
[REPL] Accumulate lines until complete
    ↓
[Library] Interpreter::eval(source)
    ↓
[Lexer] Source → Tokens
    ↓
[Parser] Tokens → AST (Expr/Statement)
    ↓
[Evaluator] AST → Value (with Environment)
    ↓
[REPL] Display value or error
```

### File Mode Flow

```
.frx File
    ↓
[Binary] Read file to string
    ↓
[Library] Interpreter::parse_program(source)
    ↓
[Lexer] Source → Tokens
    ↓
[Parser] Tokens → Program (Vec<Statement>)
    ↓
[Binary] Loop through statements
    ↓
[Library] Interpreter::execute(statement)
    ↓
[Evaluator] Statement → Option<Value>
    ↓
[Binary] Display only explicit outputs (print)
```

## Environment Management

The `Environment` struct manages:
- **Function definitions**: Persist across all expressions/statements
- **Variable bindings**: Scoped to `let-in` expressions only
- **Built-in functions**: Initialized once at startup

```rust
pub struct Environment {
    functions: HashMap<String, FunctionDef>,  // Global function storage
    variables: Vec<HashMap<String, Value>>,   // Stack of variable scopes
}

impl Environment {
    pub fn new() -> Self {
        // Initialize with built-in functions
    }

    pub fn define_function(&mut self, name: String, def: FunctionDef) -> Result<(), SemanticError>;
    pub fn get_function(&self, name: &str) -> Option<&FunctionDef>;

    pub fn push_scope(&mut self);  // Enter let-in
    pub fn pop_scope(&mut self);   // Exit let-in
    pub fn define_variable(&mut self, name: String, value: Value);
    pub fn get_variable(&self, name: &str) -> Option<&Value>;
}
```

## Key Design Principles

1. **Separation of Concerns**: Lexer, Parser, Evaluator are independent
2. **Error Early**: Detect lexical errors before parsing, syntax before evaluation
3. **Immutable AST**: Once parsed, AST nodes are not modified
4. **Explicit Lifetimes**: Spans reference source string - ensure proper lifetime management
5. **Single Error Reporting**: First error stops processing, return immediately
6. **Persistent Functions**: Functions defined once, never redefined (enforce at definition time)
7. **Scoped Variables**: Variables only exist within their `let-in` expression scope

## Memory Considerations

- **Source String**: May need to keep alive for span references in errors
- **AST Ownership**: Use `Box<Expr>` for recursive structures (Binary, LetIn, etc.)
- **Value Cloning**: Consider `Rc<String>` for string values to avoid excessive cloning
- **Environment Cloning**: Avoid cloning entire environment; use references where possible

## Testing Strategy Per Module

- **Lexer**: Test tokenization of all language constructs, edge cases (whitespace, comments if added)
- **Parser**: Test each expression type, nested expressions, error recovery
- **Evaluator**: Test correct value computation, type checking, scope management
- **Integration**: Test complete programs (REPL scenarios, .frx files)
