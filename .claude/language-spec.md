# FEROX Language Specification

## Types

FEROX is statically and strongly typed with three basic types:

- `number`: 64-bit floating point (f64)
- `string`: UTF-8 string
- `boolean`: true or false

## Literals

### Number Literals
```js
42
3.14
0.5
123.456
```

### String Literals
```js
"hello world"
"FEROX"
""  // empty string
```

### Boolean Literals
```js
true
false
```

## Operators

### Arithmetic Operators (number → number → number)

| Operator | Operation | Example |
|----------|-----------|---------|
| `+` | Addition | `5 + 3` → `8` |
| `-` | Subtraction | `5 - 3` → `2` |
| `*` | Multiplication | `5 * 3` → `15` |
| `/` | Division | `6 / 3` → `2` |
| `^` | Exponentiation | `2 ^ 3` → `8` |

**Precedence** (highest to lowest):
1. `^` (right-associative)
2. `*`, `/` (left-associative)
3. `+`, `-` (left-associative)

### Comparison Operators (any → any → boolean)

| Operator | Operation | Example |
|----------|-----------|---------|
| `==` | Equal | `5 == 5` → `true` |
| `!=` | Not equal | `5 != 3` → `true` |
| `<` | Less than | `3 < 5` → `true` |
| `>` | Greater than | `5 > 3` → `true` |
| `<=` | Less or equal | `5 <= 5` → `true` |
| `>=` | Greater or equal | `5 >= 3` → `true` |

**Type rule**: Both operands must be the same type.

### Logical Operators (boolean → boolean → boolean)

| Operator | Operation | Example |
|----------|-----------|---------|
| `&&` | Logical AND | `true && false` → `false` |
| `\|\|` | Logical OR | `true \|\| false` → `true` |
| `!` | Logical NOT | `!true` → `false` |

### String Concatenation (string → string → string)

| Operator | Operation | Example |
|----------|-----------|---------|
| `@` | Concatenation | `"hello" @ " world"` → `"hello world"` |

**Type coercion**: When concatenating with `@`, numbers and booleans are automatically converted to strings:
```js
"Answer: " @ 42           // "Answer: 42"
"Result: " @ true         // "Result: true"
"Value: " @ (5 + 3)       // "Value: 8"
```

## Expressions

### Let-in Expression

Introduces local variables with lexical scoping.

**Single-line syntax**:
```js
let x = 42 in print(x);
let x = 1, y = 2 in x + y;
```

**Multi-line syntax**:
```js
let
  x = 42;
  y = 100
in print(x + y);
```

**Scoping rules**:
- Variables exist only within the `let-in` expression body
- Variables shadow outer variables with the same name
- Nested `let-in` expressions are allowed

**Evaluation**:
- Variables are evaluated left-to-right
- Later variables can reference earlier ones:
  ```js
  let
    x = 10;
    y = x * 2
  in print(y);  // 20
  ```
- The `let-in` expression evaluates to the value of its body

### If-else Expression

Conditional expression with both branches required.

**Syntax**:
```js
if (condition) expr1 else expr2
```

**Multi-line**:
```js
if (x > 0)
  print("positive")
else
  print("non-positive");
```

**Type rule**: Both branches must return the same type.

**Evaluation**:
- Condition must be boolean type
- Only the selected branch is evaluated
- Returns value of the evaluated branch

**Examples**:
```js
let x = 5 in if (x % 2 == 0) "even" else "odd";  // "odd"

let x = 42 in print(if (x > 0) "positive" else "negative");  // prints "positive"
```

### Function Call Expression

**Syntax**:
```js
function_name(arg1, arg2, ...)
```

**Examples**:
```js
print("hello");
sin(3.14);
fib(10);
max(5, 3);
```

## Statements

### Expression Statement

Any expression followed by semicolon.

```js
print(42);
let x = 5 in print(x);
42 + 58;  // Valid, but no output in file mode
```

### Function Definition

**Syntax**:
```js
function name(param1, param2, ...) => body;
```

**Rules**:
- Function names must be unique (no redefinition)
- Parameter names must be unique within function
- Body is a single expression
- Functions are global and persist across statements
- Functions support recursion

**Examples**:
```js
function square(x) => x * x;

function fib(n) => if (n > 1) fib(n-1) + fib(n-2) else 1;

function max(a, b) => if (a > b) a else b;

function greet(name) => print("Hello, " @ name);
```

## Built-in Functions

### Mathematical Functions

| Function | Description | Example |
|----------|-------------|---------|
| `sin(x)` | Sine of x (radians) | `sin(PI)` → `0.0` |
| `cos(x)` | Cosine of x (radians) | `cos(0)` → `1.0` |
| `tan(x)` | Tangent of x (radians) | `tan(PI/4)` → `1.0` |
| `log(base, x)` | Logarithm of x in base | `log(2, 8)` → `3.0` |
| `sqrt(x)` | Square root of x | `sqrt(16)` → `4.0` |
| `abs(x)` | Absolute value | `abs(-5)` → `5.0` |
| `floor(x)` | Round down | `floor(3.7)` → `3.0` |
| `ceil(x)` | Round up | `ceil(3.2)` → `4.0` |

### Mathematical Constants

| Constant | Value |
|----------|-------|
| `PI` | 3.141592653589793 |
| `E` | 2.718281828459045 |

### I/O Functions

| Function | Description | Example |
|----------|-------------|---------|
| `print(x)` | Print value to stdout | `print("hello")` |

**Print behavior**:
- Numbers: printed as written (no trailing zeros for integers)
- Strings: printed without quotes
- Booleans: printed as `true` or `false`
- Returns Unit type

## Grammar (Informal)

```
program     := statement*

statement   := function_def
             | expression ';'

function_def := 'function' IDENTIFIER '(' param_list? ')' '=>' expression ';'

param_list  := IDENTIFIER (',' IDENTIFIER)*

expression  := let_in
             | if_else
             | logical_or

let_in      := 'let' var_decls 'in' expression
var_decls   := var_decl (';' var_decl)* ';'?
var_decl    := IDENTIFIER '=' expression

if_else     := 'if' '(' expression ')' expression 'else' expression

logical_or  := logical_and ('||' logical_and)*
logical_and := equality ('&&' equality)*
equality    := comparison (('==' | '!=') comparison)*
comparison  := concat (('<' | '>' | '<=' | '>=') concat)*
concat      := term ('@' term)*
term        := factor (('+' | '-') factor)*
factor      := unary (('*' | '/') unary)*
unary       := ('!' | '-') unary
             | power
power       := call ('^' unary)*
call        := primary ('(' arg_list? ')')?
arg_list    := expression (',' expression)*
primary     := NUMBER
             | STRING
             | BOOLEAN
             | IDENTIFIER
             | '(' expression ')'

IDENTIFIER  := [a-zA-Z_][a-zA-Z0-9_]*
NUMBER      := [0-9]+ ('.' [0-9]+)?
STRING      := '"' [^"]* '"'
BOOLEAN     := 'true' | 'false'
```

## Keywords

Reserved words that cannot be used as identifiers:

- `let`
- `in`
- `if`
- `else`
- `function`
- `true`
- `false`
- `print` (built-in function, but should be treated as keyword)

## Comments (Future Feature)

Not required for this project, but recommended syntax:

```js
// Single-line comment
/* Multi-line
   comment */
```

## Example Programs

### Factorial
```js
function factorial(n) => if (n <= 1) 1 else n * factorial(n - 1);

print(factorial(5));  // 120
```

### Greatest Common Divisor
```js
function gcd(a, b) => if (b == 0) a else gcd(b, a - (a / b) * b);

print(gcd(48, 18));  // 6
```

### Temperature Conversion
```js
function celsius_to_fahrenheit(c) => (c * 9 / 5) + 32;

let
  temp_c = 25;
  temp_f = celsius_to_fahrenheit(temp_c)
in print("Temperature: " @ temp_c @ "°C = " @ temp_f @ "°F");
```

### Quadratic Formula
```js
function discriminant(a, b, c) => (b ^ 2) - (4 * a * c);

function root1(a, b, c) => (-b + sqrt(discriminant(a, b, c))) / (2 * a);
function root2(a, b, c) => (-b - sqrt(discriminant(a, b, c))) / (2 * a);

let
  a = 1;
  b = -5;
  c = 6
in print("Roots: " @ root1(a, b, c) @ " and " @ root2(a, b, c));
```
