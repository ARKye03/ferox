use ferox_core::Lexer;

fn main() {
    let code = r#"function factorial(n) => if (n <= 1) 1 else n * factorial(n - 1);

let x = 5 in print(factorial(x));"#;

    println!("Source code:");
    println!("{}", code);
    println!("\nTokens:");
    println!("{:-<70}", "");

    let mut lexer = Lexer::new(code);

    loop {
        match lexer.next_token() {
            Ok(token) => {
                println!("{:4}:{:<3} {:?}", 
                    token.span.start.line,
                    token.span.start.column,
                    token.kind
                );
                
                if token.kind == ferox_core::TokenKind::Eof {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}
