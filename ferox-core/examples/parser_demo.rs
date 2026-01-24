use ferox_core::Parser;

fn main() {
    println!("FEROX Parser Demo");
    println!("{:=<70}", "");

    let examples = vec![
        ("Simple arithmetic", "2 + 3 * 4;"),
        ("Function call", "print(42);"),
        ("Let-in expression", "let x = 10, y = 20 in x + y;"),
        ("If-else", "if (x > 0) \"positive\" else \"negative\";"),
        ("Function definition", "function square(x) => x * x;"),
        ("Factorial", "function factorial(n) => if (n <= 1) 1 else n * factorial(n - 1);"),
        ("String concatenation", r#""Hello, " @ "World!";"#),
        ("Logical operators", "true && false || !true;"),
        ("Power operator", "2 ^ 3 ^ 4;"),
        ("Nested let-in", "let x = 1 in let y = 2 in x + y;"),
    ];

    for (description, code) in examples {
        println!("\n{}", description);
        println!("{:-<70}", "");
        println!("Code: {}", code);
        
        match Parser::from_source(code) {
            Ok(mut parser) => {
                match parser.parse_program() {
                    Ok(program) => {
                        println!("AST:  {}", program);
                    }
                    Err(e) => {
                        eprintln!("Parse error: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Lexer error: {}", e);
            }
        }
    }

    // Complete program example
    println!("\n{:=<70}", "");
    println!("Complete Program");
    println!("{:-<70}", "");
    let program = r#"
function factorial(n) => if (n <= 1) 1 else n * factorial(n - 1);
function square(x) => x * x;

let x = 5 in print(factorial(x));
let y = 10 in print(square(y));
"#;
    
    println!("Code:");
    println!("{}", program);
    println!("\nParsed AST:");
    
    match Parser::from_source(program) {
        Ok(mut parser) => {
            match parser.parse_program() {
                Ok(prog) => {
                    println!("{}", prog);
                }
                Err(e) => {
                    eprintln!("Parse error: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Lexer error: {}", e);
        }
    }
}
