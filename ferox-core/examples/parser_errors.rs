use ferox_core::Parser;

fn main() {
    println!("FEROX Parser Error Handling");
    println!("{:=<70}", "");

    let error_cases = vec![
        ("Missing semicolon", "42"),
        ("Unclosed parenthesis", "(2 + 3;"),
        ("Unopened parenthesis", "2 + 3);"),
        ("Missing else", "if (x > 0) 1;"),
        ("Missing 'in' keyword", "let x = 5 x;"),
        ("Missing function body", "function test() =>;"),
        ("Missing arrow", "function test() x * x;"),
        ("Invalid expression start", ";"),
        ("Missing closing quote", r#"let x = "hello in x;"#),
    ];

    for (description, code) in error_cases {
        println!("\n{}", description);
        println!("{:-<70}", "");
        println!("Code: {}", code);
        
        match Parser::from_source(code) {
            Ok(mut parser) => {
                match parser.parse_program() {
                    Ok(_) => {
                        println!("❌ Unexpectedly succeeded!");
                    }
                    Err(e) => {
                        println!("✓ Error detected: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("✓ Lexer error: {}", e);
            }
        }
    }

    println!("\n{:=<70}", "");
    println!("Valid Code (for comparison)");
    println!("{:-<70}", "");
    let valid_code = "let x = 42 in print(x);";
    println!("Code: {}", valid_code);
    
    match Parser::from_source(valid_code) {
        Ok(mut parser) => {
            match parser.parse_program() {
                Ok(program) => {
                    println!("✓ Successfully parsed: {}", program);
                }
                Err(e) => {
                    println!("Parse error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Lexer error: {}", e);
        }
    }
}
