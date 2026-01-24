//! Lexer for the FEROX language
//!
//! Converts source code into a stream of tokens with position tracking.

use std::fmt;

/// Position in source code (1-indexed line and column, 0-indexed byte offset)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new() -> Self {
        Self {
            line: 1,
            column: 1,
            offset: 0,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Span representing a range in source code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.start)
    }
}

/// Token kinds in the FEROX language
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Number(f64),
    String(String),
    True,
    False,
    Identifier(String),

    // Keywords
    Let,
    In,
    If,
    Else,
    Function,

    // Operators - Arithmetic
    Plus,  // +
    Minus, // -
    Star,  // *
    Slash, // /
    Caret, // ^
    At,    // @ (string concatenation)

    // Operators - Comparison
    EqualEqual,   // ==
    BangEqual,    // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=

    // Operators - Logical
    AmpAmp,   // &&
    PipePipe, // ||
    Bang,     // !

    // Punctuation
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // { (for future use)
    RightBrace, // } (for future use)
    Semicolon,  // ;
    Comma,      // ,
    Equal,      // =
    Arrow,      // =>

    // Special
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Number(n) => write!(f, "number {}", n),
            TokenKind::String(s) => write!(f, "string \"{}\"", s),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::Identifier(s) => write!(f, "identifier '{}'", s),
            TokenKind::Let => write!(f, "let"),
            TokenKind::In => write!(f, "in"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::Function => write!(f, "function"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Caret => write!(f, "^"),
            TokenKind::At => write!(f, "@"),
            TokenKind::EqualEqual => write!(f, "=="),
            TokenKind::BangEqual => write!(f, "!="),
            TokenKind::Less => write!(f, "<"),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::LessEqual => write!(f, "<="),
            TokenKind::GreaterEqual => write!(f, ">="),
            TokenKind::AmpAmp => write!(f, "&&"),
            TokenKind::PipePipe => write!(f, "||"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Equal => write!(f, "="),
            TokenKind::Arrow => write!(f, "=>"),
            TokenKind::Eof => write!(f, "end of file"),
        }
    }
}

/// A token with position information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at {}", self.kind, self.span)
    }
}

/// Lexical error
#[derive(Debug, Clone, PartialEq)]
pub struct LexError {
    pub message: String,
    pub span: Span,
}

impl LexError {
    pub fn new(message: String, span: Span) -> Self {
        Self { message, span }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexical error at {}: {}", self.span, self.message)
    }
}

impl std::error::Error for LexError {}

/// Lexer for tokenizing FEROX source code
pub struct Lexer {
    input: Vec<char>,
    position: Position,
    current_idx: usize,
}

impl Lexer {
    /// Create a new lexer from source code
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: Position::new(),
            current_idx: 0,
        }
    }

    /// Get current character without advancing
    fn current(&self) -> Option<char> {
        if self.current_idx < self.input.len() {
            Some(self.input[self.current_idx])
        } else {
            None
        }
    }

    /// Peek at the next character without advancing
    fn peek(&self) -> Option<char> {
        if self.current_idx + 1 < self.input.len() {
            Some(self.input[self.current_idx + 1])
        } else {
            None
        }
    }

    /// Advance to the next character
    fn advance(&mut self) -> Option<char> {
        let ch = self.current()?;

        if ch == '\n' {
            self.position.line += 1;
            self.position.column = 1;
        } else {
            self.position.column += 1;
        }

        self.position.offset += ch.len_utf8();
        self.current_idx += 1;

        Some(ch)
    }

    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Tokenize a number
    fn tokenize_number(&mut self, start_pos: Position) -> Result<Token, LexError> {
        let mut num_str = String::new();

        // Read integer part
        while let Some(ch) = self.current() {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check for decimal part
        if self.current() == Some('.') && self.peek().map_or(false, |c| c.is_ascii_digit()) {
            num_str.push('.');
            self.advance();

            while let Some(ch) = self.current() {
                if ch.is_ascii_digit() {
                    num_str.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let end_pos = self.position;
        let span = Span::new(start_pos, end_pos);

        match num_str.parse::<f64>() {
            Ok(value) => Ok(Token::new(TokenKind::Number(value), span)),
            Err(_) => Err(LexError::new(format!("Invalid number: {}", num_str), span)),
        }
    }

    /// Tokenize a string literal
    fn tokenize_string(&mut self, start_pos: Position) -> Result<Token, LexError> {
        // Skip opening quote
        self.advance();

        let mut string_value = String::new();

        loop {
            match self.current() {
                Some('"') => {
                    self.advance();
                    let end_pos = self.position;
                    let span = Span::new(start_pos, end_pos);
                    return Ok(Token::new(TokenKind::String(string_value), span));
                }
                Some('\n') | None => {
                    let end_pos = self.position;
                    let span = Span::new(start_pos, end_pos);
                    return Err(LexError::new("Unterminated string".to_string(), span));
                }
                Some(ch) => {
                    string_value.push(ch);
                    self.advance();
                }
            }
        }
    }

    /// Tokenize an identifier or keyword
    fn tokenize_identifier(&mut self, start_pos: Position) -> Token {
        let mut ident = String::new();

        while let Some(ch) = self.current() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let end_pos = self.position;
        let span = Span::new(start_pos, end_pos);

        let kind = match ident.as_str() {
            "let" => TokenKind::Let,
            "in" => TokenKind::In,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "function" => TokenKind::Function,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => TokenKind::Identifier(ident),
        };

        Token::new(kind, span)
    }

    /// Get the next token
    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace();

        let start_pos = self.position;

        match self.current() {
            None => Ok(Token::new(TokenKind::Eof, Span::new(start_pos, start_pos))),

            Some(ch) if ch.is_ascii_digit() => self.tokenize_number(start_pos),

            Some('"') => self.tokenize_string(start_pos),

            Some(ch) if ch.is_alphabetic() || ch == '_' => Ok(self.tokenize_identifier(start_pos)),

            Some('(') => {
                self.advance();
                Ok(Token::new(
                    TokenKind::LeftParen,
                    Span::new(start_pos, self.position),
                ))
            }

            Some(')') => {
                self.advance();
                Ok(Token::new(
                    TokenKind::RightParen,
                    Span::new(start_pos, self.position),
                ))
            }

            Some('{') => {
                self.advance();
                Ok(Token::new(
                    TokenKind::LeftBrace,
                    Span::new(start_pos, self.position),
                ))
            }

            Some('}') => {
                self.advance();
                Ok(Token::new(
                    TokenKind::RightBrace,
                    Span::new(start_pos, self.position),
                ))
            }

            Some(';') => {
                self.advance();
                Ok(Token::new(
                    TokenKind::Semicolon,
                    Span::new(start_pos, self.position),
                ))
            }

            Some(',') => {
                self.advance();
                Ok(Token::new(
                    TokenKind::Comma,
                    Span::new(start_pos, self.position),
                ))
            }

            Some('+') => {
                self.advance();
                Ok(Token::new(
                    TokenKind::Plus,
                    Span::new(start_pos, self.position),
                ))
            }

            Some('-') => {
                self.advance();
                Ok(Token::new(
                    TokenKind::Minus,
                    Span::new(start_pos, self.position),
                ))
            }

            Some('*') => {
                self.advance();
                Ok(Token::new(
                    TokenKind::Star,
                    Span::new(start_pos, self.position),
                ))
            }

            Some('/') => {
                self.advance();
                Ok(Token::new(
                    TokenKind::Slash,
                    Span::new(start_pos, self.position),
                ))
            }

            Some('^') => {
                self.advance();
                Ok(Token::new(
                    TokenKind::Caret,
                    Span::new(start_pos, self.position),
                ))
            }

            Some('@') => {
                self.advance();
                Ok(Token::new(
                    TokenKind::At,
                    Span::new(start_pos, self.position),
                ))
            }

            Some('=') => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Ok(Token::new(
                        TokenKind::EqualEqual,
                        Span::new(start_pos, self.position),
                    ))
                } else if self.current() == Some('>') {
                    self.advance();
                    Ok(Token::new(
                        TokenKind::Arrow,
                        Span::new(start_pos, self.position),
                    ))
                } else {
                    Ok(Token::new(
                        TokenKind::Equal,
                        Span::new(start_pos, self.position),
                    ))
                }
            }

            Some('!') => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Ok(Token::new(
                        TokenKind::BangEqual,
                        Span::new(start_pos, self.position),
                    ))
                } else {
                    Ok(Token::new(
                        TokenKind::Bang,
                        Span::new(start_pos, self.position),
                    ))
                }
            }

            Some('<') => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Ok(Token::new(
                        TokenKind::LessEqual,
                        Span::new(start_pos, self.position),
                    ))
                } else {
                    Ok(Token::new(
                        TokenKind::Less,
                        Span::new(start_pos, self.position),
                    ))
                }
            }

            Some('>') => {
                self.advance();
                if self.current() == Some('=') {
                    self.advance();
                    Ok(Token::new(
                        TokenKind::GreaterEqual,
                        Span::new(start_pos, self.position),
                    ))
                } else {
                    Ok(Token::new(
                        TokenKind::Greater,
                        Span::new(start_pos, self.position),
                    ))
                }
            }

            Some('&') => {
                self.advance();
                if self.current() == Some('&') {
                    self.advance();
                    Ok(Token::new(
                        TokenKind::AmpAmp,
                        Span::new(start_pos, self.position),
                    ))
                } else {
                    let span = Span::new(start_pos, self.position);
                    Err(LexError::new(
                        "Unexpected character '&' (did you mean '&&'?)".to_string(),
                        span,
                    ))
                }
            }

            Some('|') => {
                self.advance();
                if self.current() == Some('|') {
                    self.advance();
                    Ok(Token::new(
                        TokenKind::PipePipe,
                        Span::new(start_pos, self.position),
                    ))
                } else {
                    let span = Span::new(start_pos, self.position);
                    Err(LexError::new(
                        "Unexpected character '|' (did you mean '||'?)".to_string(),
                        span,
                    ))
                }
            }

            Some(ch) => {
                self.advance();
                let span = Span::new(start_pos, self.position);
                Err(LexError::new(
                    format!("Unexpected character '{}'", ch),
                    span,
                ))
            }
        }
    }

    /// Tokenize the entire input into a vector of tokens
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            let is_eof = token.kind == TokenKind::Eof;
            tokens.push(token);

            if is_eof {
                break;
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("42 3.14 0.5");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4); // 3 numbers + EOF
        assert_eq!(tokens[0].kind, TokenKind::Number(42.0));
        assert_eq!(tokens[1].kind, TokenKind::Number(3.14));
        assert_eq!(tokens[2].kind, TokenKind::Number(0.5));
        assert_eq!(tokens[3].kind, TokenKind::Eof);
    }

    #[test]
    fn test_strings() {
        let mut lexer = Lexer::new(r#""hello" "world" """#);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].kind, TokenKind::String("hello".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::String("world".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::String("".to_string()));
    }

    #[test]
    fn test_unterminated_string() {
        let mut lexer = Lexer::new(r#""hello"#);
        let result = lexer.tokenize();
        assert!(result.is_err());
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("let in if else function true false");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::In);
        assert_eq!(tokens[2].kind, TokenKind::If);
        assert_eq!(tokens[3].kind, TokenKind::Else);
        assert_eq!(tokens[4].kind, TokenKind::Function);
        assert_eq!(tokens[5].kind, TokenKind::True);
        assert_eq!(tokens[6].kind, TokenKind::False);
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::new("x foo_bar _test abc123");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Identifier("x".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::Identifier("foo_bar".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::Identifier("_test".to_string()));
        assert_eq!(tokens[3].kind, TokenKind::Identifier("abc123".to_string()));
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("+ - * / ^ @ == != < > <= >= && || !");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Plus);
        assert_eq!(tokens[1].kind, TokenKind::Minus);
        assert_eq!(tokens[2].kind, TokenKind::Star);
        assert_eq!(tokens[3].kind, TokenKind::Slash);
        assert_eq!(tokens[4].kind, TokenKind::Caret);
        assert_eq!(tokens[5].kind, TokenKind::At);
        assert_eq!(tokens[6].kind, TokenKind::EqualEqual);
        assert_eq!(tokens[7].kind, TokenKind::BangEqual);
        assert_eq!(tokens[8].kind, TokenKind::Less);
        assert_eq!(tokens[9].kind, TokenKind::Greater);
        assert_eq!(tokens[10].kind, TokenKind::LessEqual);
        assert_eq!(tokens[11].kind, TokenKind::GreaterEqual);
        assert_eq!(tokens[12].kind, TokenKind::AmpAmp);
        assert_eq!(tokens[13].kind, TokenKind::PipePipe);
        assert_eq!(tokens[14].kind, TokenKind::Bang);
    }

    #[test]
    fn test_punctuation() {
        let mut lexer = Lexer::new("( ) { } ; , = =>");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::LeftParen);
        assert_eq!(tokens[1].kind, TokenKind::RightParen);
        assert_eq!(tokens[2].kind, TokenKind::LeftBrace);
        assert_eq!(tokens[3].kind, TokenKind::RightBrace);
        assert_eq!(tokens[4].kind, TokenKind::Semicolon);
        assert_eq!(tokens[5].kind, TokenKind::Comma);
        assert_eq!(tokens[6].kind, TokenKind::Equal);
        assert_eq!(tokens[7].kind, TokenKind::Arrow);
    }

    #[test]
    fn test_expression() {
        let mut lexer = Lexer::new("let x = 42 in x + 1;");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Identifier("x".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::Equal);
        assert_eq!(tokens[3].kind, TokenKind::Number(42.0));
        assert_eq!(tokens[4].kind, TokenKind::In);
        assert_eq!(tokens[5].kind, TokenKind::Identifier("x".to_string()));
        assert_eq!(tokens[6].kind, TokenKind::Plus);
        assert_eq!(tokens[7].kind, TokenKind::Number(1.0));
        assert_eq!(tokens[8].kind, TokenKind::Semicolon);
        assert_eq!(tokens[9].kind, TokenKind::Eof);
    }

    #[test]
    fn test_function_definition() {
        let mut lexer = Lexer::new("function square(x) => x * x;");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Function);
        assert_eq!(tokens[1].kind, TokenKind::Identifier("square".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::LeftParen);
        assert_eq!(tokens[3].kind, TokenKind::Identifier("x".to_string()));
        assert_eq!(tokens[4].kind, TokenKind::RightParen);
        assert_eq!(tokens[5].kind, TokenKind::Arrow);
    }

    #[test]
    fn test_multiline() {
        let input = r#"let
  x = 42;
  y = 100
in print(x + y);"#;

        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Identifier("x".to_string()));
        assert_eq!(tokens[0].span.start.line, 1);
        assert_eq!(tokens[1].span.start.line, 2);
    }

    #[test]
    fn test_position_tracking() {
        let mut lexer = Lexer::new("42\n100");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].span.start.line, 1);
        assert_eq!(tokens[0].span.start.column, 1);
        assert_eq!(tokens[1].span.start.line, 2);
        assert_eq!(tokens[1].span.start.column, 1);
    }

    #[test]
    fn test_invalid_character() {
        let mut lexer = Lexer::new("42 $ 100");
        let result = lexer.tokenize();
        assert!(result.is_err());
    }

    #[test]
    fn test_incomplete_operators() {
        let mut lexer = Lexer::new("&");
        let result = lexer.tokenize();
        assert!(result.is_err());

        let mut lexer = Lexer::new("|");
        let result = lexer.tokenize();
        assert!(result.is_err());
    }
}
