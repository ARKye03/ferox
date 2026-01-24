# 🦊 FEROX

**F**unctional **E**xpression **R**untime for **O**perations and e**X**ecution

A clean, educational interpreter for a functional programming language built from scratch in Rust.

## What is FEROX?

FEROX is a simple yet powerful interpreted language that combines functional programming concepts with imperative features. It's designed to be easy to learn while demonstrating core interpreter implementation concepts.

This project is a rewrite and modernization of an old college project of mine.

## Features

**Interactive REPL** - Write code and see results instantly  
**Script Execution** - Run `.frx` files  
**Strong Typing** - Number, String, and Boolean types  
**Recursion** - Full support for recursive functions  
**Lexical Scoping** - Variables shadow outer scopes cleanly  
**Built-in Math** - Trigonometry, logarithms, and more  
**No Dependencies** - Core library uses only Rust's standard library

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/ARKye03/ferox.git
cd ferox

# Build the project
cargo build --release

# Run FEROX
./target/release/ferox
```

### Your First FEROX Program

**Interactive Mode (REPL):**

```bash
$ cargo run -p ferox-cli
FEROX REPL v0.1.0
> print("Hello, World!");
Hello, World!
> let x = 42 in print("The answer is " @ x);
The answer is 42
```

**File Mode:**
Create a file `hello.frx`:

```javascript
function greet(name) => print("Hello, " @ name @ "!");

greet("World");
```

Run it:

```bash
$ cargo run -p ferox-cli -- hello.frx
Hello, World!
```

## Language Overview

### Variables and Scoping

```javascript
let x = 10 in x + 5; // 15

let x = 1,
  y = 2,
  z = 3 in x + y + z; // 6

// Multi-line let expressions
let x = 10;
y = 20;
z = x + y in print(z); // 30
```

### Functions

```javascript
// Define a function
function square(x) => x * x;

square(5);  // 25

// Recursive functions work great!
function factorial(n) =>
  if (n <= 1)
    1
  else
    n * factorial(n - 1);

factorial(5);  // 120
```

### Conditionals

```javascript
let age = 18 in
  if (age >= 18)
    print("Adult")
  else
    print("Minor");
```

### Built-in Functions

**Math Functions:**

- `sqrt(x)` - Square root
- `abs(x)` - Absolute value
- `floor(x)`, `ceil(x)` - Rounding
- `sin(x)`, `cos(x)`, `tan(x)` - Trigonometry (in radians)
- `log(base, x)` - Logarithm
- `pow(base, exp)` - Power

**Constants:**

- `PI` - 3.141592653589793
- `E` - 2.718281828459045

**I/O:**

- `print(x)` - Output to console

### Operators

**Arithmetic:** `+` `-` `*` `/` `^` (power)  
**Comparison:** `==` `!=` `<` `>` `<=` `>=`  
**Logical:** `&&` `||` `!`  
**String:** `@` (concatenation with automatic type coercion)

### Examples

**Temperature Conversion:**

```javascript
function celsius_to_fahrenheit(c) => (c * 9 / 5) + 32;

let temp = 25 in
  print("25°C = " @ celsius_to_fahrenheit(temp) @ "°F");
```

**Fibonacci:**

```javascript
function fib(n) =>
  if (n <= 1)
    n
  else
    fib(n - 1) + fib(n - 2);

print(fib(10));  // 55
```

**String Concatenation:**

```javascript
print("2 + 2 = " @ (2 + 2));          // Automatic type conversion
print("Result: " @ true);              // Works with booleans too
```

## Architecture Highlights

- **Two-Crate Design**: Clean separation between core logic and CLI
- **Zero External Dependencies**: Core library uses only Rust's standard library
- **Recursive Descent Parser**: Hand-written parser for full control
- **Tree-Walking Interpreter**: Direct AST evaluation
- **Stack-Based Scoping**: Efficient lexical scope management

## Running Examples

Check out the `frx_examples/` directory for sample programs:

```bash
# Hello World
cargo run -p ferox-cli -- frx_examples/01_hello.frx

# Recursive functions
cargo run -p ferox-cli -- frx_examples/06_recursion.frx

# Complete demo
cargo run -p ferox-cli -- frx_examples/complete_demo.frx
```

## Development

**Build:**

```bash
cargo build
```

**Run Tests:**

```bash
cargo test
```

**Run Specific Tests:**

```bash
cargo test -p ferox-core     # Core library tests only
cargo test test_eval_        # Tests matching pattern
```

**Format Code:**

```bash
cargo fmt
```

**Lint:**

```bash
cargo clippy
```

## Language Design Philosophy

FEROX was designed with these principles:

1. **Simplicity**: Easy to understand and implement
2. **Safety**: Strong typing prevents runtime type errors
3. **Expression-Oriented**: Everything returns a value
4. **Functional**: Immutable variables, first-class functions
5. **Educational**: Perfect for learning interpreter construction

## Error Handling

FEROX provides clear, helpful error messages (I hope so) with line and column numbers:

```error
Runtime error at 2:5: Division by zero
Syntax error at 1:15: Expected ';' after expression
Type error: expected number, got string
```

---

### **Made with 🦊 and Rust**
