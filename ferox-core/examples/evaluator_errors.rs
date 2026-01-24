use ferox_core::{Evaluator, Parser};

fn run_code(code: &str, description: &str) {
    println!("\n{}", description);
    println!("{:-<70}", "");
    println!("Code: {}", code.trim());
    
    match Parser::from_source(code) {
        Ok(mut parser) => {
            match parser.parse_program() {
                Ok(program) => {
                    let mut evaluator = Evaluator::new();
                    match evaluator.eval_program(&program) {
                        Ok(_) => println!("✗ Unexpectedly succeeded!"),
                        Err(e) => println!("✓ Error detected: {}", e),
                    }
                }
                Err(e) => println!("✓ Parse error: {}", e),
            }
        }
        Err(e) => println!("✓ Lexer error: {}", e),
    }
}

fn main() {
    println!("FEROX Evaluator Error Handling");
    println!("{:=<70}", "");

    run_code("5 / 0;", "1. Division by zero");

    run_code("5 + true;", "2. Type mismatch (number + boolean)");

    run_code(r#""hello" + "world";"#, "3. Type error (string + string)");

    run_code("x;", "4. Undefined variable");

    run_code("foo();", "5. Undefined function");

    run_code(
        r#"
function add(a, b) => a + b;
add(1);
"#,
        "6. Wrong number of arguments (too few)",
    );

    run_code(
        r#"
function add(a, b) => a + b;
add(1, 2, 3);
"#,
        "7. Wrong number of arguments (too many)",
    );

    run_code(
        r#"
function test() => 42;
function test() => 100;
"#,
        "8. Function redefinition",
    );

    run_code(
        "if (5) 1 else 2;",
        "9. Type error in condition (expected boolean)",
    );

    run_code(
        "!5;",
        "10. Type error in unary operator (! requires boolean)",
    );

    run_code(
        r#"-"hello";"#,
        "11. Type error in unary operator (- requires number)",
    );

    run_code(
        "true && 5;",
        "12. Type error in logical operator",
    );

    run_code(
        "sqrt(true);",
        "13. Type error in built-in function",
    );

    run_code(
        "log(2);",
        "14. Wrong arity for built-in function",
    );

    run_code(
        r#"
function duplicate_params(x, x) => x;
duplicate_params(1, 2);
"#,
        "15. Duplicate parameters in function definition",
    );

    // Success case for comparison
    println!("\n{:=<70}", "");
    println!("Valid Code (for comparison)");
    println!("{:-<70}", "");
    let valid = "let x = 42 in print(x);";
    println!("Code: {}", valid);
    match Parser::from_source(valid) {
        Ok(mut parser) => {
            match parser.parse_program() {
                Ok(program) => {
                    let mut evaluator = Evaluator::new();
                    match evaluator.eval_program(&program) {
                        Ok(_) => println!("✓ Successfully executed"),
                        Err(e) => println!("Error: {}", e),
                    }
                }
                Err(e) => println!("Parse error: {}", e),
            }
        }
        Err(e) => println!("Lexer error: {}", e),
    }
}
