//! Parser for the FEROX language
//!
//! Implements a recursive descent parser that converts tokens into an AST.

use crate::ast::*;
use crate::lexer::{Lexer, Span, Token, TokenKind};
use std::fmt;

// ============================================================================
// Parse Error
// ============================================================================

/// Syntax error with position information
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

impl ParseError {
    pub fn new(message: String, span: Span) -> Self {
        Self { message, span }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax error at {}: {}", self.span, self.message)
    }
}

impl std::error::Error for ParseError {}

// ============================================================================
// Parser
// ============================================================================

/// Recursive descent parser for FEROX
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Create a new parser from a token stream
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Create a parser directly from source code
    pub fn from_source(source: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        Ok(Self::new(tokens))
    }

    // ------------------------------------------------------------------------
    // Token navigation helpers
    // ------------------------------------------------------------------------

    /// Get the current token
    fn current_token(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Check if we're at the end of input
    fn is_at_end(&self) -> bool {
        self.current_token().kind == TokenKind::Eof
    }

    /// Peek at the current token kind
    fn peek(&self) -> &TokenKind {
        &self.current_token().kind
    }

    /// Get the span of the current token
    fn current_span(&self) -> Span {
        self.current_token().span
    }

    /// Advance to the next token and return the previous one
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens[self.current - 1].clone()
    }

    /// Check if current token matches the given kind
    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(self.peek()) == std::mem::discriminant(kind)
    }

    /// Consume a token if it matches, otherwise return error
    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<Token, ParseError> {
        if self.check(&kind) {
            Ok(self.advance())
        } else {
            Err(ParseError::new(
                format!("{}, got {}", message, self.peek()),
                self.current_span(),
            ))
        }
    }

    /// Try to match and consume any of the given token kinds
    #[allow(dead_code)]
    fn match_any(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    // ------------------------------------------------------------------------
    // Parsing methods (following the grammar)
    // ------------------------------------------------------------------------

    /// Parse a complete program
    /// program := statement*
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        Ok(Program::new(statements))
    }

    /// Parse a single statement
    /// statement := function_def | expression ';'
    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        if self.check(&TokenKind::Function) {
            self.parse_function_def()
        } else {
            self.parse_expression_statement()
        }
    }

    /// Parse a function definition
    /// function_def := 'function' IDENTIFIER '(' param_list? ')' '=>' expression ';'
    fn parse_function_def(&mut self) -> Result<Stmt, ParseError> {
        let start_span = self.current_span();

        self.consume(TokenKind::Function, "Expected 'function'")?;

        let name = match self.advance().kind {
            TokenKind::Identifier(name) => name,
            _ => {
                return Err(ParseError::new(
                    "Expected function name".to_string(),
                    self.current_span(),
                ))
            }
        };

        self.consume(TokenKind::LeftParen, "Expected '(' after function name")?;

        let params = if !self.check(&TokenKind::RightParen) {
            self.parse_param_list()?
        } else {
            Vec::new()
        };

        self.consume(TokenKind::RightParen, "Expected ')' after parameters")?;
        self.consume(TokenKind::Arrow, "Expected '=>' before function body")?;

        let body = self.parse_expression()?;

        let semicolon_token =
            self.consume(TokenKind::Semicolon, "Expected ';' after function body")?;
        let end_span = semicolon_token.span;

        Ok(Stmt::function_def(
            name,
            params,
            body,
            Span::new(start_span.start, end_span.end),
        ))
    }

    /// Parse parameter list
    /// param_list := IDENTIFIER (',' IDENTIFIER)*
    fn parse_param_list(&mut self) -> Result<Vec<String>, ParseError> {
        let mut params = Vec::new();

        loop {
            match self.advance().kind {
                TokenKind::Identifier(name) => params.push(name),
                _ => {
                    return Err(ParseError::new(
                        "Expected parameter name".to_string(),
                        self.current_span(),
                    ))
                }
            }

            if !self.check(&TokenKind::Comma) {
                break;
            }
            self.advance(); // consume comma
        }

        Ok(params)
    }

    /// Parse an expression statement
    /// expression ';'
    fn parse_expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.parse_expression()?;
        let semicolon = self.consume(TokenKind::Semicolon, "Expected ';' after expression")?;

        let span = Span::new(expr.span.start, semicolon.span.end);
        Ok(Stmt::expression(expr, span))
    }

    /// Parse an expression
    /// expression := let_in | if_else | logical_or
    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        if self.check(&TokenKind::Let) {
            self.parse_let_in()
        } else if self.check(&TokenKind::If) {
            self.parse_if_else()
        } else {
            self.parse_logical_or()
        }
    }

    /// Parse let-in expression
    /// let_in := 'let' var_decls 'in' expression
    /// var_decls := var_decl (';' var_decl)* ';'?
    /// var_decl := IDENTIFIER '=' expression
    fn parse_let_in(&mut self) -> Result<Expr, ParseError> {
        let start_span = self.current_span();

        self.consume(TokenKind::Let, "Expected 'let'")?;

        let mut declarations = Vec::new();

        // Parse variable declarations
        loop {
            let decl_start = self.current_span();

            let name = match self.advance().kind {
                TokenKind::Identifier(name) => name,
                _ => {
                    return Err(ParseError::new(
                        "Expected variable name in let declaration".to_string(),
                        self.current_span(),
                    ))
                }
            };

            self.consume(TokenKind::Equal, "Expected '=' after variable name")?;

            let value = self.parse_expression()?;
            let decl_span = Span::new(decl_start.start, value.span.end);

            declarations.push(VarDecl::new(name, value, decl_span));

            // Check for semicolon or comma separator
            if self.check(&TokenKind::Semicolon) {
                self.advance();
                // Continue if not followed by 'in'
                if self.check(&TokenKind::In) {
                    break;
                }
            } else if self.check(&TokenKind::Comma) {
                self.advance();
                // Continue parsing more declarations
            } else if self.check(&TokenKind::In) {
                break;
            } else {
                return Err(ParseError::new(
                    "Expected ';', ',' or 'in' in let expression".to_string(),
                    self.current_span(),
                ));
            }
        }

        self.consume(TokenKind::In, "Expected 'in' after let declarations")?;

        let body = self.parse_expression()?;
        let end_span = body.span;

        Ok(Expr::let_in(
            declarations,
            body,
            Span::new(start_span.start, end_span.end),
        ))
    }

    /// Parse if-else expression
    /// if_else := 'if' '(' expression ')' expression 'else' expression
    fn parse_if_else(&mut self) -> Result<Expr, ParseError> {
        let start_span = self.current_span();

        self.consume(TokenKind::If, "Expected 'if'")?;
        self.consume(TokenKind::LeftParen, "Expected '(' after 'if'")?;

        let condition = self.parse_expression()?;

        self.consume(TokenKind::RightParen, "Expected ')' after condition")?;

        let then_branch = self.parse_expression()?;

        self.consume(TokenKind::Else, "Expected 'else' after then branch")?;

        let else_branch = self.parse_expression()?;
        let end_span = else_branch.span;

        Ok(Expr::if_else(
            condition,
            then_branch,
            else_branch,
            Span::new(start_span.start, end_span.end),
        ))
    }

    /// Parse logical OR expression
    /// logical_or := logical_and ('||' logical_and)*
    fn parse_logical_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_logical_and()?;

        while self.check(&TokenKind::PipePipe) {
            self.advance();
            let right = self.parse_logical_and()?;
            let span = Span::new(expr.span.start, right.span.end);
            expr = Expr::binary(BinaryOp::Or, expr, right, span);
        }

        Ok(expr)
    }

    /// Parse logical AND expression
    /// logical_and := equality ('&&' equality)*
    fn parse_logical_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_equality()?;

        while self.check(&TokenKind::AmpAmp) {
            self.advance();
            let right = self.parse_equality()?;
            let span = Span::new(expr.span.start, right.span.end);
            expr = Expr::binary(BinaryOp::And, expr, right, span);
        }

        Ok(expr)
    }

    /// Parse equality expression
    /// equality := comparison (('==' | '!=') comparison)*
    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_comparison()?;

        loop {
            let op = if self.check(&TokenKind::EqualEqual) {
                self.advance();
                BinaryOp::Equal
            } else if self.check(&TokenKind::BangEqual) {
                self.advance();
                BinaryOp::NotEqual
            } else {
                break;
            };

            let right = self.parse_comparison()?;
            let span = Span::new(expr.span.start, right.span.end);
            expr = Expr::binary(op, expr, right, span);
        }

        Ok(expr)
    }

    /// Parse comparison expression
    /// comparison := concat (('<' | '>' | '<=' | '>=') concat)*
    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_concat()?;

        loop {
            let op = if self.check(&TokenKind::Less) {
                self.advance();
                BinaryOp::Less
            } else if self.check(&TokenKind::Greater) {
                self.advance();
                BinaryOp::Greater
            } else if self.check(&TokenKind::LessEqual) {
                self.advance();
                BinaryOp::LessEqual
            } else if self.check(&TokenKind::GreaterEqual) {
                self.advance();
                BinaryOp::GreaterEqual
            } else {
                break;
            };

            let right = self.parse_concat()?;
            let span = Span::new(expr.span.start, right.span.end);
            expr = Expr::binary(op, expr, right, span);
        }

        Ok(expr)
    }

    /// Parse concatenation expression
    /// concat := term ('@' term)*
    fn parse_concat(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_term()?;

        while self.check(&TokenKind::At) {
            self.advance();
            let right = self.parse_term()?;
            let span = Span::new(expr.span.start, right.span.end);
            expr = Expr::binary(BinaryOp::Concat, expr, right, span);
        }

        Ok(expr)
    }

    /// Parse term (addition/subtraction)
    /// term := factor (('+' | '-') factor)*
    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_factor()?;

        loop {
            let op = if self.check(&TokenKind::Plus) {
                self.advance();
                BinaryOp::Add
            } else if self.check(&TokenKind::Minus) {
                self.advance();
                BinaryOp::Sub
            } else {
                break;
            };

            let right = self.parse_factor()?;
            let span = Span::new(expr.span.start, right.span.end);
            expr = Expr::binary(op, expr, right, span);
        }

        Ok(expr)
    }

    /// Parse factor (multiplication/division)
    /// factor := unary (('*' | '/') unary)*
    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_unary()?;

        loop {
            let op = if self.check(&TokenKind::Star) {
                self.advance();
                BinaryOp::Mul
            } else if self.check(&TokenKind::Slash) {
                self.advance();
                BinaryOp::Div
            } else {
                break;
            };

            let right = self.parse_unary()?;
            let span = Span::new(expr.span.start, right.span.end);
            expr = Expr::binary(op, expr, right, span);
        }

        Ok(expr)
    }

    /// Parse unary expression
    /// unary := ('!' | '-') unary | power
    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if self.check(&TokenKind::Bang) || self.check(&TokenKind::Minus) {
            let start_span = self.current_span();
            let op_token = self.advance();
            let op = match op_token.kind {
                TokenKind::Bang => UnaryOp::Not,
                TokenKind::Minus => UnaryOp::Neg,
                _ => unreachable!(),
            };

            let operand = self.parse_unary()?;
            let span = Span::new(start_span.start, operand.span.end);

            Ok(Expr::unary(op, operand, span))
        } else {
            self.parse_power()
        }
    }

    /// Parse power expression (right-associative)
    /// power := call ('^' unary)*
    fn parse_power(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_call()?;

        if self.check(&TokenKind::Caret) {
            self.advance();
            let right = self.parse_unary()?; // Right-associative
            let span = Span::new(expr.span.start, right.span.end);
            expr = Expr::binary(BinaryOp::Pow, expr, right, span);
        }

        Ok(expr)
    }

    /// Parse function call
    /// call := primary ('(' arg_list? ')')?
    fn parse_call(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_primary()?;

        // Check if it's a function call
        if let ExprKind::Identifier(name) = &expr.kind {
            if self.check(&TokenKind::LeftParen) {
                self.advance();

                let args = if !self.check(&TokenKind::RightParen) {
                    self.parse_arg_list()?
                } else {
                    Vec::new()
                };

                let end_token =
                    self.consume(TokenKind::RightParen, "Expected ')' after arguments")?;
                let span = Span::new(expr.span.start, end_token.span.end);

                return Ok(Expr::call(name.clone(), args, span));
            }
        }

        Ok(expr)
    }

    /// Parse argument list
    /// arg_list := expression (',' expression)*
    fn parse_arg_list(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut args = Vec::new();

        loop {
            args.push(self.parse_expression()?);

            if !self.check(&TokenKind::Comma) {
                break;
            }
            self.advance(); // consume comma
        }

        Ok(args)
    }

    /// Parse primary expression
    /// primary := NUMBER | STRING | BOOLEAN | IDENTIFIER | '(' expression ')'
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.advance();

        match token.kind {
            TokenKind::Number(n) => Ok(Expr::number(n, token.span)),

            TokenKind::String(s) => Ok(Expr::string(s, token.span)),

            TokenKind::True => Ok(Expr::boolean(true, token.span)),

            TokenKind::False => Ok(Expr::boolean(false, token.span)),

            TokenKind::Identifier(name) => Ok(Expr::identifier(name, token.span)),

            TokenKind::LeftParen => {
                let expr = self.parse_expression()?;
                let end_token =
                    self.consume(TokenKind::RightParen, "Expected ')' after expression")?;
                let span = Span::new(token.span.start, end_token.span.end);

                // Update the expression's span to include the parentheses
                Ok(Expr::new(expr.kind, span))
            }

            _ => Err(ParseError::new(
                format!("Unexpected token: {}", token.kind),
                token.span,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let mut parser = Parser::from_source("42;").unwrap();
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);

        if let StmtKind::Expression(expr) = &program.statements[0].kind {
            assert_eq!(expr.kind, ExprKind::Number(42.0));
        } else {
            panic!("Expected expression statement");
        }
    }

    #[test]
    fn test_parse_string() {
        let mut parser = Parser::from_source(r#""hello";"#).unwrap();
        let program = parser.parse_program().unwrap();

        if let StmtKind::Expression(expr) = &program.statements[0].kind {
            assert_eq!(expr.kind, ExprKind::String("hello".to_string()));
        } else {
            panic!("Expected expression statement");
        }
    }

    #[test]
    fn test_parse_boolean() {
        let mut parser = Parser::from_source("true;").unwrap();
        let program = parser.parse_program().unwrap();

        if let StmtKind::Expression(expr) = &program.statements[0].kind {
            assert_eq!(expr.kind, ExprKind::Boolean(true));
        } else {
            panic!("Expected expression statement");
        }
    }

    #[test]
    fn test_parse_binary_addition() {
        let mut parser = Parser::from_source("2 + 3;").unwrap();
        let program = parser.parse_program().unwrap();

        if let StmtKind::Expression(expr) = &program.statements[0].kind {
            if let ExprKind::Binary { op, .. } = &expr.kind {
                assert_eq!(*op, BinaryOp::Add);
            } else {
                panic!("Expected binary expression");
            }
        }
    }

    #[test]
    fn test_parse_operator_precedence() {
        // 2 + 3 * 4 should parse as 2 + (3 * 4)
        let mut parser = Parser::from_source("2 + 3 * 4;").unwrap();
        let program = parser.parse_program().unwrap();

        if let StmtKind::Expression(expr) = &program.statements[0].kind {
            if let ExprKind::Binary { op, left, right } = &expr.kind {
                assert_eq!(*op, BinaryOp::Add);
                assert!(matches!(left.kind, ExprKind::Number(2.0)));
                assert!(matches!(
                    right.kind,
                    ExprKind::Binary {
                        op: BinaryOp::Mul,
                        ..
                    }
                ));
            } else {
                panic!("Expected binary expression");
            }
        }
    }

    #[test]
    fn test_parse_power_right_associative() {
        // 2 ^ 3 ^ 4 should parse as 2 ^ (3 ^ 4)
        let mut parser = Parser::from_source("2 ^ 3 ^ 4;").unwrap();
        let program = parser.parse_program().unwrap();

        if let StmtKind::Expression(expr) = &program.statements[0].kind {
            if let ExprKind::Binary { op, left, right } = &expr.kind {
                assert_eq!(*op, BinaryOp::Pow);
                assert!(matches!(left.kind, ExprKind::Number(2.0)));
                assert!(matches!(
                    right.kind,
                    ExprKind::Binary {
                        op: BinaryOp::Pow,
                        ..
                    }
                ));
            }
        }
    }

    #[test]
    fn test_parse_unary() {
        let mut parser = Parser::from_source("-5;").unwrap();
        let program = parser.parse_program().unwrap();

        if let StmtKind::Expression(expr) = &program.statements[0].kind {
            if let ExprKind::Unary { op, .. } = &expr.kind {
                assert_eq!(*op, UnaryOp::Neg);
            } else {
                panic!("Expected unary expression");
            }
        }
    }

    #[test]
    fn test_parse_function_call() {
        let mut parser = Parser::from_source("print(42);").unwrap();
        let program = parser.parse_program().unwrap();

        if let StmtKind::Expression(expr) = &program.statements[0].kind {
            if let ExprKind::Call { name, args } = &expr.kind {
                assert_eq!(name, "print");
                assert_eq!(args.len(), 1);
            } else {
                panic!("Expected function call");
            }
        }
    }

    #[test]
    fn test_parse_function_call_multiple_args() {
        let mut parser = Parser::from_source("max(5, 3);").unwrap();
        let program = parser.parse_program().unwrap();

        if let StmtKind::Expression(expr) = &program.statements[0].kind {
            if let ExprKind::Call { name, args } = &expr.kind {
                assert_eq!(name, "max");
                assert_eq!(args.len(), 2);
            }
        }
    }

    #[test]
    fn test_parse_if_else() {
        let mut parser = Parser::from_source("if (x > 0) 1 else 0;").unwrap();
        let program = parser.parse_program().unwrap();

        if let StmtKind::Expression(expr) = &program.statements[0].kind {
            assert!(matches!(expr.kind, ExprKind::If { .. }));
        }
    }

    #[test]
    fn test_parse_let_in_single() {
        let mut parser = Parser::from_source("let x = 42 in x;").unwrap();
        let program = parser.parse_program().unwrap();

        if let StmtKind::Expression(expr) = &program.statements[0].kind {
            if let ExprKind::Let { declarations, .. } = &expr.kind {
                assert_eq!(declarations.len(), 1);
                assert_eq!(declarations[0].name, "x");
            } else {
                panic!("Expected let-in expression");
            }
        }
    }

    #[test]
    fn test_parse_let_in_multiple_comma() {
        let mut parser = Parser::from_source("let x = 1, y = 2 in x + y;").unwrap();
        let program = parser.parse_program().unwrap();

        if let StmtKind::Expression(expr) = &program.statements[0].kind {
            if let ExprKind::Let { declarations, .. } = &expr.kind {
                assert_eq!(declarations.len(), 2);
            }
        }
    }

    #[test]
    fn test_parse_let_in_multiple_semicolon() {
        let input = r#"let
  x = 1;
  y = 2
in x + y;"#;
        let mut parser = Parser::from_source(input).unwrap();
        let program = parser.parse_program().unwrap();

        if let StmtKind::Expression(expr) = &program.statements[0].kind {
            if let ExprKind::Let { declarations, .. } = &expr.kind {
                assert_eq!(declarations.len(), 2);
            }
        }
    }

    #[test]
    fn test_parse_function_def() {
        let mut parser = Parser::from_source("function square(x) => x * x;").unwrap();
        let program = parser.parse_program().unwrap();

        if let StmtKind::FunctionDef { name, params, .. } = &program.statements[0].kind {
            assert_eq!(name, "square");
            assert_eq!(params.len(), 1);
            assert_eq!(params[0], "x");
        } else {
            panic!("Expected function definition");
        }
    }

    #[test]
    fn test_parse_function_def_multiple_params() {
        let mut parser = Parser::from_source("function add(a, b) => a + b;").unwrap();
        let program = parser.parse_program().unwrap();

        if let StmtKind::FunctionDef { name, params, .. } = &program.statements[0].kind {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
        }
    }

    #[test]
    fn test_parse_factorial() {
        let input = "function factorial(n) => if (n <= 1) 1 else n * factorial(n - 1);";
        let mut parser = Parser::from_source(input).unwrap();
        let program = parser.parse_program().unwrap();

        assert_eq!(program.statements.len(), 1);
        assert!(matches!(
            program.statements[0].kind,
            StmtKind::FunctionDef { .. }
        ));
    }

    #[test]
    fn test_parse_complete_program() {
        let input = r#"
function square(x) => x * x;
function add(a, b) => a + b;
let x = 5 in print(square(x));
"#;
        let mut parser = Parser::from_source(input).unwrap();
        let program = parser.parse_program().unwrap();

        assert_eq!(program.statements.len(), 3);
    }

    #[test]
    fn test_parse_parenthesized() {
        let mut parser = Parser::from_source("(2 + 3) * 4;").unwrap();
        let program = parser.parse_program().unwrap();

        if let StmtKind::Expression(expr) = &program.statements[0].kind {
            if let ExprKind::Binary { op, left, .. } = &expr.kind {
                assert_eq!(*op, BinaryOp::Mul);
                assert!(matches!(
                    left.kind,
                    ExprKind::Binary {
                        op: BinaryOp::Add,
                        ..
                    }
                ));
            }
        }
    }

    #[test]
    fn test_parse_logical_operators() {
        let mut parser = Parser::from_source("true && false || true;").unwrap();
        let program = parser.parse_program().unwrap();

        if let StmtKind::Expression(expr) = &program.statements[0].kind {
            // Should parse as (true && false) || true
            if let ExprKind::Binary { op, .. } = &expr.kind {
                assert_eq!(*op, BinaryOp::Or);
            }
        }
    }

    #[test]
    fn test_parse_string_concat() {
        let mut parser = Parser::from_source(r#""hello" @ " " @ "world";"#).unwrap();
        let program = parser.parse_program().unwrap();

        if let StmtKind::Expression(expr) = &program.statements[0].kind {
            assert!(matches!(
                expr.kind,
                ExprKind::Binary {
                    op: BinaryOp::Concat,
                    ..
                }
            ));
        }
    }

    #[test]
    fn test_parse_error_missing_semicolon() {
        let mut parser = Parser::from_source("42").unwrap();
        let result = parser.parse_program();
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_unclosed_paren() {
        let mut parser = Parser::from_source("(42;").unwrap();
        let result = parser.parse_program();
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_missing_else() {
        let mut parser = Parser::from_source("if (x > 0) 1;").unwrap();
        let result = parser.parse_program();
        assert!(result.is_err());
    }
}
