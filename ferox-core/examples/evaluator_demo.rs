use ferox_core::{Evaluator, Parser};

fn run_code(code: &str, description: &str) {
    println!("\n{}", description);
    println!("{:-<70}", "");
    println!("Code: {}", code.trim());
    println!("Output:");
    
    match Parser::from_source(code) {
        Ok(mut parser) => {
            match parser.parse_program() {
                Ok(program) => {
                    let mut evaluator = Evaluator::new();
                    match evaluator.eval_program(&program) {
                        Ok(_) => {}
                        Err(e) => eprintln!("Runtime error: {}", e),
                    }
                }
                Err(e) => eprintln!("Parse error: {}", e),
            }
        }
        Err(e) => eprintln!("Lexer error: {}", e),
    }
}

fn main() {
    println!("FEROX Evaluator Demo");
    println!("{:=<70}", "");

    run_code("print(42);", "1. Simple print");

    run_code("print(2 + 3 * 4);", "2. Arithmetic expression");

    run_code(
        r#"print("Hello, " @ "World!");"#,
        "3. String concatenation",
    );

    run_code(
        "let x = 10, y = 20 in print(x + y);",
        "4. Let-in expression",
    );

    run_code(
        "if (5 > 3) print(\"yes\") else print(\"no\");",
        "5. If-else expression",
    );

    run_code(
        r#"
function square(x) => x * x;
print(square(5));
"#,
        "6. Function definition and call",
    );

    run_code(
        r#"
function factorial(n) => if (n <= 1) 1 else n * factorial(n - 1);
print(factorial(5));
"#,
        "7. Recursive factorial",
    );

    run_code(
        r#"
function fib(n) => if (n <= 1) n else fib(n - 1) + fib(n - 2);
print(fib(10));
"#,
        "8. Recursive fibonacci",
    );

    run_code(
        r#"
print(sqrt(16));
print(abs(-42));
print(floor(3.7));
print(ceil(3.2));
"#,
        "9. Built-in math functions",
    );

    run_code(
        r#"
print("PI = " @ PI);
print("E = " @ E);
print(sin(PI));
"#,
        "10. Mathematical constants",
    );

    run_code(
        r#"
let
  a = 1;
  b = -5;
  c = 6;
  discriminant = (b ^ 2) - (4 * a * c);
  root1 = (-b + sqrt(discriminant)) / (2 * a);
  root2 = (-b - sqrt(discriminant)) / (2 * a)
in print("Roots: " @ root1 @ " and " @ root2);
"#,
        "11. Quadratic formula",
    );

    run_code(
        r#"
function gcd(a, b) => if (b == 0) a else gcd(b, a - floor(a / b) * b);
print(gcd(48, 18));
"#,
        "12. Greatest common divisor",
    );

    run_code(
        r#"
print(true && false);
print(true || false);
print(!true);
print((5 > 3) && (10 < 20));
"#,
        "13. Logical operations",
    );

    run_code(
        r#"
print(2 ^ 3);
print(2 ^ 3 ^ 2);
"#,
        "14. Power operator (right-associative)",
    );

    run_code(
        r#"
let x = 10 in
  let y = x * 2 in
    let z = y + 5 in
      print(z);
"#,
        "15. Nested scopes",
    );
}
