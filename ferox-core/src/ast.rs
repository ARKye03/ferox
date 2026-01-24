//! Abstract Syntax Tree (AST) definitions for FEROX language
//!
//! Represents the parsed structure of FEROX programs with position tracking.

use crate::lexer::Span;
use std::fmt;

// ============================================================================
// Binary Operators
// ============================================================================

/// Binary operators in FEROX
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    // Arithmetic
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Pow, // ^

    // Comparison
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=

    // Logical
    And, // &&
    Or,  // ||

    // String concatenation
    Concat, // @
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Pow => "^",
            BinaryOp::Equal => "==",
            BinaryOp::NotEqual => "!=",
            BinaryOp::Less => "<",
            BinaryOp::Greater => ">",
            BinaryOp::LessEqual => "<=",
            BinaryOp::GreaterEqual => ">=",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
            BinaryOp::Concat => "@",
        };
        write!(f, "{}", s)
    }
}

// ============================================================================
// Unary Operators
// ============================================================================

/// Unary operators in FEROX
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Not, // !
    Neg, // - (negation)
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            UnaryOp::Not => "!",
            UnaryOp::Neg => "-",
        };
        write!(f, "{}", s)
    }
}

// ============================================================================
// Expressions
// ============================================================================

/// Expression node with position tracking
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// Expression kinds in FEROX
#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    /// Number literal: 42, 3.14
    Number(f64),

    /// String literal: "hello"
    String(String),

    /// Boolean literal: true, false
    Boolean(bool),

    /// Variable reference: x, foo
    Identifier(String),

    /// Binary operation: a + b, x == y
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    /// Unary operation: !x, -5
    Unary { op: UnaryOp, operand: Box<Expr> },

    /// Function call: print(x), sin(3.14)
    Call { name: String, args: Vec<Expr> },

    /// If-else expression: if (cond) expr1 else expr2
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },

    /// Let-in expression: let x = 1, y = 2 in x + y
    Let {
        declarations: Vec<VarDecl>,
        body: Box<Expr>,
    },
}

// ============================================================================
// Variable Declaration
// ============================================================================

/// Variable declaration in let-in expression
#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub name: String,
    pub value: Expr,
    pub span: Span,
}

impl VarDecl {
    pub fn new(name: String, value: Expr, span: Span) -> Self {
        Self { name, value, span }
    }
}

// ============================================================================
// Statements
// ============================================================================

/// Statement node with position tracking
#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

impl Stmt {
    pub fn new(kind: StmtKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// Statement kinds in FEROX
#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
    /// Expression statement: expr;
    Expression(Expr),

    /// Function definition: function name(params) => body;
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Expr,
    },
}

// ============================================================================
// Program (top-level)
// ============================================================================

/// A complete FEROX program (sequence of statements)
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Self { statements }
    }
}

// ============================================================================
// Display implementations for pretty printing
// ============================================================================

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ExprKind::Number(n) => write!(f, "{}", n),
            ExprKind::String(s) => write!(f, "\"{}\"", s),
            ExprKind::Boolean(b) => write!(f, "{}", b),
            ExprKind::Identifier(name) => write!(f, "{}", name),
            ExprKind::Binary { op, left, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
            ExprKind::Unary { op, operand } => {
                write!(f, "({}{})", op, operand)
            }
            ExprKind::Call { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            ExprKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                write!(
                    f,
                    "(if {} then {} else {})",
                    condition, then_branch, else_branch
                )
            }
            ExprKind::Let { declarations, body } => {
                write!(f, "(let ")?;
                for (i, decl) in declarations.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} = {}", decl.name, decl.value)?;
                }
                write!(f, " in {})", body)
            }
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            StmtKind::Expression(expr) => write!(f, "{};", expr),
            StmtKind::FunctionDef { name, params, body } => {
                write!(f, "function {}(", name)?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") => {};", body)
            }
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for stmt in &self.statements {
            writeln!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

// ============================================================================
// Helper functions for building AST nodes
// ============================================================================

impl Expr {
    /// Create a number literal
    pub fn number(value: f64, span: Span) -> Self {
        Self::new(ExprKind::Number(value), span)
    }

    /// Create a string literal
    pub fn string(value: String, span: Span) -> Self {
        Self::new(ExprKind::String(value), span)
    }

    /// Create a boolean literal
    pub fn boolean(value: bool, span: Span) -> Self {
        Self::new(ExprKind::Boolean(value), span)
    }

    /// Create an identifier reference
    pub fn identifier(name: String, span: Span) -> Self {
        Self::new(ExprKind::Identifier(name), span)
    }

    /// Create a binary operation
    pub fn binary(op: BinaryOp, left: Expr, right: Expr, span: Span) -> Self {
        Self::new(
            ExprKind::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            span,
        )
    }

    /// Create a unary operation
    pub fn unary(op: UnaryOp, operand: Expr, span: Span) -> Self {
        Self::new(
            ExprKind::Unary {
                op,
                operand: Box::new(operand),
            },
            span,
        )
    }

    /// Create a function call
    pub fn call(name: String, args: Vec<Expr>, span: Span) -> Self {
        Self::new(ExprKind::Call { name, args }, span)
    }

    /// Create an if-else expression
    pub fn if_else(condition: Expr, then_branch: Expr, else_branch: Expr, span: Span) -> Self {
        Self::new(
            ExprKind::If {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch: Box::new(else_branch),
            },
            span,
        )
    }

    /// Create a let-in expression
    pub fn let_in(declarations: Vec<VarDecl>, body: Expr, span: Span) -> Self {
        Self::new(
            ExprKind::Let {
                declarations,
                body: Box::new(body),
            },
            span,
        )
    }
}

impl Stmt {
    /// Create an expression statement
    pub fn expression(expr: Expr, span: Span) -> Self {
        Self::new(StmtKind::Expression(expr), span)
    }

    /// Create a function definition statement
    pub fn function_def(name: String, params: Vec<String>, body: Expr, span: Span) -> Self {
        Self::new(StmtKind::FunctionDef { name, params, body }, span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Position;

    fn dummy_span() -> Span {
        Span::new(Position::new(), Position::new())
    }

    #[test]
    fn test_number_literal() {
        let expr = Expr::number(42.0, dummy_span());
        assert_eq!(expr.kind, ExprKind::Number(42.0));
        assert_eq!(format!("{}", expr), "42");
    }

    #[test]
    fn test_string_literal() {
        let expr = Expr::string("hello".to_string(), dummy_span());
        assert_eq!(expr.kind, ExprKind::String("hello".to_string()));
        assert_eq!(format!("{}", expr), "\"hello\"");
    }

    #[test]
    fn test_boolean_literal() {
        let expr = Expr::boolean(true, dummy_span());
        assert_eq!(expr.kind, ExprKind::Boolean(true));
        assert_eq!(format!("{}", expr), "true");
    }

    #[test]
    fn test_identifier() {
        let expr = Expr::identifier("x".to_string(), dummy_span());
        assert_eq!(expr.kind, ExprKind::Identifier("x".to_string()));
        assert_eq!(format!("{}", expr), "x");
    }

    #[test]
    fn test_binary_operation() {
        let left = Expr::number(2.0, dummy_span());
        let right = Expr::number(3.0, dummy_span());
        let expr = Expr::binary(BinaryOp::Add, left, right, dummy_span());

        assert_eq!(format!("{}", expr), "(2 + 3)");
    }

    #[test]
    fn test_unary_operation() {
        let operand = Expr::number(5.0, dummy_span());
        let expr = Expr::unary(UnaryOp::Neg, operand, dummy_span());

        assert_eq!(format!("{}", expr), "(-5)");
    }

    #[test]
    fn test_function_call() {
        let arg1 = Expr::number(42.0, dummy_span());
        let expr = Expr::call("print".to_string(), vec![arg1], dummy_span());

        assert_eq!(format!("{}", expr), "print(42)");
    }

    #[test]
    fn test_if_else() {
        let condition = Expr::boolean(true, dummy_span());
        let then_branch = Expr::number(1.0, dummy_span());
        let else_branch = Expr::number(2.0, dummy_span());
        let expr = Expr::if_else(condition, then_branch, else_branch, dummy_span());

        assert_eq!(format!("{}", expr), "(if true then 1 else 2)");
    }

    #[test]
    fn test_let_in() {
        let decl = VarDecl::new(
            "x".to_string(),
            Expr::number(42.0, dummy_span()),
            dummy_span(),
        );
        let body = Expr::identifier("x".to_string(), dummy_span());
        let expr = Expr::let_in(vec![decl], body, dummy_span());

        assert_eq!(format!("{}", expr), "(let x = 42 in x)");
    }

    #[test]
    fn test_nested_expression() {
        // (2 + 3) * 4
        let left = Expr::binary(
            BinaryOp::Add,
            Expr::number(2.0, dummy_span()),
            Expr::number(3.0, dummy_span()),
            dummy_span(),
        );
        let right = Expr::number(4.0, dummy_span());
        let expr = Expr::binary(BinaryOp::Mul, left, right, dummy_span());

        assert_eq!(format!("{}", expr), "((2 + 3) * 4)");
    }

    #[test]
    fn test_expression_statement() {
        let expr = Expr::number(42.0, dummy_span());
        let stmt = Stmt::expression(expr, dummy_span());

        assert_eq!(format!("{}", stmt), "42;");
    }

    #[test]
    fn test_function_definition() {
        let body = Expr::binary(
            BinaryOp::Mul,
            Expr::identifier("x".to_string(), dummy_span()),
            Expr::identifier("x".to_string(), dummy_span()),
            dummy_span(),
        );
        let stmt = Stmt::function_def(
            "square".to_string(),
            vec!["x".to_string()],
            body,
            dummy_span(),
        );

        assert_eq!(format!("{}", stmt), "function square(x) => (x * x);");
    }

    #[test]
    fn test_program() {
        let stmt1 = Stmt::function_def(
            "add".to_string(),
            vec!["a".to_string(), "b".to_string()],
            Expr::binary(
                BinaryOp::Add,
                Expr::identifier("a".to_string(), dummy_span()),
                Expr::identifier("b".to_string(), dummy_span()),
                dummy_span(),
            ),
            dummy_span(),
        );

        let stmt2 = Stmt::expression(
            Expr::call(
                "add".to_string(),
                vec![
                    Expr::number(2.0, dummy_span()),
                    Expr::number(3.0, dummy_span()),
                ],
                dummy_span(),
            ),
            dummy_span(),
        );

        let program = Program::new(vec![stmt1, stmt2]);
        let output = format!("{}", program);

        assert!(output.contains("function add"));
        assert!(output.contains("add(2, 3)"));
    }

    #[test]
    fn test_complex_let_in() {
        // let x = 10, y = x * 2 in y + 5
        let decl1 = VarDecl::new(
            "x".to_string(),
            Expr::number(10.0, dummy_span()),
            dummy_span(),
        );

        let decl2 = VarDecl::new(
            "y".to_string(),
            Expr::binary(
                BinaryOp::Mul,
                Expr::identifier("x".to_string(), dummy_span()),
                Expr::number(2.0, dummy_span()),
                dummy_span(),
            ),
            dummy_span(),
        );

        let body = Expr::binary(
            BinaryOp::Add,
            Expr::identifier("y".to_string(), dummy_span()),
            Expr::number(5.0, dummy_span()),
            dummy_span(),
        );

        let expr = Expr::let_in(vec![decl1, decl2], body, dummy_span());
        assert_eq!(format!("{}", expr), "(let x = 10, y = (x * 2) in (y + 5))");
    }

    #[test]
    fn test_operator_display() {
        assert_eq!(format!("{}", BinaryOp::Add), "+");
        assert_eq!(format!("{}", BinaryOp::Equal), "==");
        assert_eq!(format!("{}", BinaryOp::And), "&&");
        assert_eq!(format!("{}", BinaryOp::Concat), "@");
        assert_eq!(format!("{}", UnaryOp::Not), "!");
        assert_eq!(format!("{}", UnaryOp::Neg), "-");
    }
}
