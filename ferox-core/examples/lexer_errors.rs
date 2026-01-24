use ferox_core::Lexer;

fn main() {
    let test_cases = vec![
        ("Unterminated string", r#"let x = "hello"#),
        ("Invalid character", "let x = 42 $ 100;"),
        ("Incomplete operator &", "x & y"),
        ("Incomplete operator |", "x | y"),
    ];

    for (description, code) in test_cases {
        println!("\n{}: {}", description, code);
        println!("{:-<60}", "");

        let mut lexer = Lexer::new(code);
        match lexer.tokenize() {
            Ok(_) => println!("Unexpectedly succeeded!"),
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("\n\nValid code:");
    println!("{:-<60}", "");
    let code = r#"let x = "hello" @ " world" in print(x);"#;
    println!("{}", code);

    let mut lexer = Lexer::new(code);
    match lexer.tokenize() {
        Ok(tokens) => println!("Success! {} tokens generated", tokens.len()),
        Err(e) => println!("Error: {}", e),
    }
}
