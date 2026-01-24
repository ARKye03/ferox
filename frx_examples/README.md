# FEROX Example Programs

This directory contains example `.frx` files demonstrating FEROX language features.

## Running Examples

```bash
# From the workspace root
cargo run -p ferox-cli -- frx_examples/<example>.frx

# Or using the built binary
./target/debug/ferox frx_examples/<example>.frx
```

## Examples

### Basic Examples

- **01_hello.frx** - Hello World program
- **02_arithmetic.frx** - Arithmetic operations and operator precedence
- **03_variables.frx** - Let-in expressions, multiple variables, scoping
- **04_conditionals.frx** - If-else expressions, nested conditionals
- **05_functions.frx** - Function definitions and calls
- **06_recursion.frx** - Recursive functions (factorial, fibonacci, gcd, power, sum)
- **07_strings.frx** - String concatenation and type coercion

### Complete Programs

- **complete_demo.frx** - Comprehensive demonstration of multiple features
- **error_test.frx** - Example showing error handling (division by zero)

## Language Features Demonstrated

- **Variables**: `let x = 42 in x + 1`
- **Functions**: `function square(x) => x * x;`
- **Conditionals**: `if (x > 0) "positive" else "negative"`
- **Recursion**: Factorial, Fibonacci, GCD
- **Operators**: Arithmetic (+, -, *, /, ^), Comparison, Logical, String (@)
- **Built-in Functions**: print, sqrt, abs, floor, ceil, sin, cos, tan, log
- **Constants**: PI, E
- **Scoping**: Lexical scoping with variable shadowing
- **Type System**: Strong typing with automatic string coercion for @ operator
