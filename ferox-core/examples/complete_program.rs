use ferox_core::{Evaluator, Parser};

fn main() {
    println!("FEROX Complete Program Example");
    println!("{:=<70}", "");

    let program = r#"
function celsius_to_fahrenheit(c) => (c * 9 / 5) + 32;
function fahrenheit_to_celsius(f) => (f - 32) * 5 / 9;

function max(a, b) => if (a > b) a else b;
function min(a, b) => if (a < b) a else b;

function factorial(n) => if (n <= 1) 1 else n * factorial(n - 1);
function fibonacci(n) => if (n <= 1) n else fibonacci(n - 1) + fibonacci(n - 2);

function gcd(a, b) => if (b == 0) a else gcd(b, a - floor(a / b) * b);

function pow(base, exp) => 
  if (exp == 0) 
    1 
  else 
    base * pow(base, exp - 1);

print("=== Temperature Conversion ===");
let temp_c = 25 in print("25°C = " @ celsius_to_fahrenheit(temp_c) @ "°F");
let temp_f = 77 in print("77°F = " @ fahrenheit_to_celsius(temp_f) @ "°C");

print("");
print("=== Min/Max ===");
print("max(10, 20) = " @ max(10, 20));
print("min(10, 20) = " @ min(10, 20));

print("");
print("=== Factorial ===");
print("factorial(5) = " @ factorial(5));
print("factorial(10) = " @ factorial(10));

print("");
print("=== Fibonacci ===");
print("fibonacci(10) = " @ fibonacci(10));

print("");
print("=== GCD ===");
print("gcd(48, 18) = " @ gcd(48, 18));
print("gcd(100, 35) = " @ gcd(100, 35));

print("");
print("=== Custom Power Function ===");
print("pow(2, 10) = " @ pow(2, 10));
print("pow(3, 4) = " @ pow(3, 4));

print("");
print("=== Mathematical Constants ===");
print("PI = " @ PI);
print("E = " @ E);
print("sin(PI/2) = " @ sin(PI / 2));

print("");
print("=== Complex Calculation ===");
let
  x = 5;
  y = 10;
  z = 3;
  result = (x * y) + (z ^ 2) - sqrt(16)
in print("(5 * 10) + (3 ^ 2) - sqrt(16) = " @ result);

print("");
print("=== Conditional Logic ===");
let age = 18 in
  if (age >= 18)
    print("Adult")
  else
    print("Minor");

print("");
print("=== String Operations ===");
print("Hello, " @ "World!");
print("2 + 2 = " @ (2 + 2));
print("Boolean: " @ true);
"#;

    println!("Executing FEROX program...\n");
    println!("{:=<70}\n", "");

    match Parser::from_source(program) {
        Ok(mut parser) => {
            match parser.parse_program() {
                Ok(parsed_program) => {
                    let mut evaluator = Evaluator::new();
                    match evaluator.eval_program(&parsed_program) {
                        Ok(_) => {
                            println!("\n{:=<70}", "");
                            println!("✓ Program completed successfully!");
                        }
                        Err(e) => {
                            eprintln!("\n✗ Runtime error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("✗ Parse error: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("✗ Lexer error: {}", e);
        }
    }
}
