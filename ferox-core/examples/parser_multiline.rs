use ferox_core::Parser;

fn main() {
    println!("FEROX Parser - Multi-line Support");
    println!("{:=<70}", "");

    // Example 1: Multi-line let-in
    let code1 = r#"let
  x = 10;
  y = 20;
  z = x + y
in print(z);"#;

    println!("\nExample 1: Multi-line let-in with semicolons");
    println!("{:-<70}", "");
    println!("Code:");
    println!("{}", code1);
    println!("\nParsed AST:");

    match Parser::from_source(code1) {
        Ok(mut parser) => match parser.parse_program() {
            Ok(program) => println!("{}", program),
            Err(e) => eprintln!("Parse error: {}", e),
        },
        Err(e) => eprintln!("Lexer error: {}", e),
    }

    // Example 2: Multi-line function
    let code2 = r#"function factorial(n) =>
  if (n <= 1)
    1
  else
    n * factorial(n - 1);"#;

    println!("\nExample 2: Multi-line function definition");
    println!("{:-<70}", "");
    println!("Code:");
    println!("{}", code2);
    println!("\nParsed AST:");

    match Parser::from_source(code2) {
        Ok(mut parser) => match parser.parse_program() {
            Ok(program) => println!("{}", program),
            Err(e) => eprintln!("Parse error: {}", e),
        },
        Err(e) => eprintln!("Lexer error: {}", e),
    }

    // Example 3: Complete multi-line program
    let code3 = r#"
function gcd(a, b) =>
  if (b == 0)
    a
  else
    gcd(b, a - (a / b) * b);

let
  x = 48;
  y = 18;
  result = gcd(x, y)
in print("GCD: " @ result);
"#;

    println!("\nExample 3: Complete multi-line program");
    println!("{:-<70}", "");
    println!("Code:");
    println!("{}", code3);
    println!("\nParsed AST:");

    match Parser::from_source(code3) {
        Ok(mut parser) => match parser.parse_program() {
            Ok(program) => println!("{}", program),
            Err(e) => eprintln!("Parse error: {}", e),
        },
        Err(e) => eprintln!("Lexer error: {}", e),
    }

    // Example 4: Mixed separators in let-in
    let code4 = r#"let x = 1, y = 2; z = 3 in x + y + z;"#;

    println!("\nExample 4: Mixed comma and semicolon separators");
    println!("{:-<70}", "");
    println!("Code:");
    println!("{}", code4);
    println!("\nParsed AST:");

    match Parser::from_source(code4) {
        Ok(mut parser) => match parser.parse_program() {
            Ok(program) => println!("{}", program),
            Err(e) => eprintln!("Parse error: {}", e),
        },
        Err(e) => eprintln!("Lexer error: {}", e),
    }
}
