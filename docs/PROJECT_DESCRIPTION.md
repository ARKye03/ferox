# FEROX: Functional Expression Runtime for Operations and eXecution

In this project, you will implement an interpreter for the programming language **FEROX**.

To complete the project, you must implement a subset of FEROX, which we define below.  
FEROX is a much larger language than what is required for this project, and you are free to implement any additional functionality you want.

> In the 3rd year, in the Compilers course, you will see how to implement a fully functional compiler for the complete FEROX language.

## The FEROX Language (simplified)

FEROX is an imperative, functional, statically and strongly typed programming language. Almost all instructions in FEROX are expressions.  
In particular, the subset of FEROX you must implement consists only of expressions that can be written on a single line.

### Basic Expressions

All instructions in FEROX end with `;`. The simplest instruction in FEROX that performs an action is the following:

```js
print("Hello World");
```

FEROX also supports arithmetic expressions:

```js
print((((1 + 2) ^ 3) * 4) / 5);
```

And basic mathematical functions:

```js
print(sin(2 * PI) ^ (2 + cos((3 * PI) / log(4, 64))));
```

> FEROX also supports multi-line expressions, but those are not required for this project.
> FEROX has three basic types: `string`, `number`, and `boolean`. Additionally, new types can be defined in FEROX, but this is not required for this project.

### Functions

In FEROX there are two types of functions: _inline_ functions and regular functions.
For this project, you only need to implement _inline_ functions. They have the following form:

```js
function tan(x) => sin(x) / cos(x);
```

Once a function is defined, it can be used in any expression:

```js
print(tan(PI / 2));
```

The body of an _inline_ function is any expression, which of course may include other functions, basic expressions, or any combination of them.

### Variables

In FEROX, variables can be declared using the `let-in` expression, which works as follows:

```js
let x = PI / 2 in print(tan(x));
```

In general, a `let-in` expression consists of one or more variable declarations and a body, which can be any expression where the declared variables can be used.
Outside a `let-in` expression, the variables no longer exist.

For example, with two variables:

```js
let number = 42, text = "The meaning of life is" in print(text @ number);
```

Which is equivalent to:

```js
let number = 42 in (let text = "The meaning of life is" in (print(text @ number)));
```

The return value of a `let-in` expression is the return value of its body, which makes the following possible:

```js
print(7 + (let x = 2 in x * x));
```

Which results in `11`.

> The `let-in` expression allows much more functionality, but for this project you only need to implement the features described above.

### Conditionals

Conditions in FEROX are implemented using the `if-else` expression, which takes a boolean expression inside parentheses, and two expressions for the `if` and `else` bodies respectively.
Both branches must always be present:

```js
let a = 42 in if (a % 2 == 0) print("Even") else print("odd");
```

Since `if-else` is an expression, it can be used inside another expression (similar to Rust’s `if` expression):

```js
let a = 42 in print(if (a % 2 == 0) "even" else "odd");
```

> FEROX supports conditional expressions with more than one condition using `elif`, but for this project you do not need to implement them.

### Recursion

Since FEROX supports composed functions, it naturally supports recursion.
An example of a recursive function in FEROX is the following:

```js
function fib(n) => if (n > 1) fib(n-1) + fib(n-2) else 1;
```

You must ensure that your implementation supports this kind of recursive definition.

## The Interpreter

Your FEROX interpreter will be a console application, where the user can enter a FEROX expression, press ENTER, and immediately see the result of evaluating the expression (if any).

This is an example of a possible interaction:

```js
> let x = 42 in print(x);
42
> function fib(n) => if (n > 1) fib(n-1) + fib(n-2) else 1;
> fib(5)
13
> let x = 3 in fib(x+1);
8
> print(fib(6));
21
```

Each line starting with `>` represents user input, and immediately after, the result of evaluating that expression is printed, if any.

> Note that when an expression has a return value (such as a function call), the returned value is printed directly, even if there is no explicit `print` instruction.

All previously declared functions are visible in any subsequent expression.
Functions cannot be redefined.

### Errors

In FEROX there are three types of errors that you must detect.
If an error is detected, the interpreter must print a line describing the error as informatively as possible.

#### Lexical Error

Errors caused by the presence of invalid tokens. For example:

```js
> let 14a = 5 in print(14a);
! LEXICAL ERROR: `14a` is not a valid token.
```

#### Syntax Error

Errors caused by malformed expressions, such as unbalanced parentheses or incomplete expressions. For example:

```js
> let a = 5 in print(a;
! SYNTAX ERROR: Missing closing parenthesis after `a`.
> let a = 5 inn print(a);
! SYNTAX ERROR: Invalid token `inn` in `let-in` expression.
> let a = in print(a);
! SYNTAX ERROR: Missing expression in `let-in` after variable `a`.
```

### Semantic Error

Errors caused by incorrect use of types or arguments. For example:

```js
> let a = "hello world" in print(a + 5);
! SEMANTIC ERROR: Operator `+` cannot be used between `string` and `number`.
> print(fib("hello world"));
! SEMANTIC ERROR: Function `fib` receives `number`, not `string`.
> print(fib(4,3));
! SEMANTIC ERROR: Function `fib` receives 1 argument(s), but 2 were given.
```

If more than one error exists, you must detect **only one** of them.

## Implementation Details

This project mainly focuses on implementing a convenient type hierarchy that represents the FEROX language (or at least the subset you must support).
You should have a set of types that represent expressions and instructions, as well as another set of types that represent the main processes and concepts of your interpreter.

Your solution must consist of at least two Rust crates:

- A **library crate** where all parsing and evaluation logic for the FEROX language is implemented.
  In this crate, you may not use external dependencies beyond Rust’s standard library.
- A **binary crate** (console application or another visualization method) that implements the interactive part of the interpreter.

This project **does not require** advanced knowledge of compiler theory, since the defined subset of FEROX can be solved in a direct manner.
However, you may use any compilation or interpretation algorithm or technique you wish, as long as you can implement it from scratch and explain how it works.

One piece of advice we can give you is to study the concept of **recursive descent parsing**, which will greatly simplify the task of interpreting the FEROX language.
