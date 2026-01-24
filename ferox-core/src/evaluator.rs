//! Evaluator for the FEROX language
//!
//! Executes AST nodes and produces runtime values.

use crate::ast::*;
use crate::environment::{Environment, EvalError, Function, Value};
use crate::lexer::Span;
use std::f64::consts;

// ============================================================================
// Evaluator
// ============================================================================

/// Evaluator for FEROX programs
pub struct Evaluator {
    env: Environment,
}

impl Evaluator {
    /// Create a new evaluator
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    // ------------------------------------------------------------------------
    // Program and Statement Evaluation
    // ------------------------------------------------------------------------

    /// Evaluate a complete program
    pub fn eval_program(&mut self, program: &Program) -> Result<(), EvalError> {
        for stmt in &program.statements {
            self.eval_statement(stmt)?;
        }
        Ok(())
    }

    /// Evaluate a single statement
    pub fn eval_statement(&mut self, stmt: &Stmt) -> Result<Option<Value>, EvalError> {
        match &stmt.kind {
            StmtKind::Expression(expr) => {
                let value = self.eval_expr(expr)?;
                Ok(Some(value))
            }
            StmtKind::FunctionDef { name, params, body } => {
                // Check for duplicate parameters
                for (i, param) in params.iter().enumerate() {
                    if params[i + 1..].contains(param) {
                        return Err(EvalError::new(
                            format!("Duplicate parameter '{}' in function '{}'", param, name),
                            stmt.span,
                        ));
                    }
                }

                let func = Function {
                    params: params.clone(),
                    body: body.clone(),
                };

                self.env
                    .define_function(name.clone(), func)
                    .map_err(|msg| EvalError::new(msg, stmt.span))?;

                Ok(None)
            }
        }
    }

    // ------------------------------------------------------------------------
    // Expression Evaluation
    // ------------------------------------------------------------------------

    /// Evaluate an expression
    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, EvalError> {
        match &expr.kind {
            ExprKind::Number(n) => Ok(Value::Number(*n)),
            ExprKind::String(s) => Ok(Value::String(s.clone())),
            ExprKind::Boolean(b) => Ok(Value::Boolean(*b)),
            ExprKind::Identifier(name) => self.eval_identifier(name, expr.span),
            ExprKind::Binary { op, left, right } => self.eval_binary(*op, left, right, expr.span),
            ExprKind::Unary { op, operand } => self.eval_unary(*op, operand, expr.span),
            ExprKind::Call { name, args } => self.eval_call(name, args, expr.span),
            ExprKind::If {
                condition,
                then_branch,
                else_branch,
            } => self.eval_if(condition, then_branch, else_branch),
            ExprKind::Let { declarations, body } => self.eval_let(declarations, body),
        }
    }

    /// Evaluate an identifier
    fn eval_identifier(&self, name: &str, span: Span) -> Result<Value, EvalError> {
        // Check for built-in constants
        match name {
            "PI" => return Ok(Value::Number(consts::PI)),
            "E" => return Ok(Value::Number(consts::E)),
            _ => {}
        }

        // Look up variable
        self.env
            .get(name)
            .cloned()
            .ok_or_else(|| EvalError::undefined_var(name, span))
    }

    /// Evaluate a binary operation
    fn eval_binary(
        &mut self,
        op: BinaryOp,
        left: &Expr,
        right: &Expr,
        span: Span,
    ) -> Result<Value, EvalError> {
        let left_val = self.eval_expr(left)?;
        let right_val = self.eval_expr(right)?;

        match op {
            // Arithmetic operators
            BinaryOp::Add => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                (l, r) => Err(EvalError::type_error(
                    "number + number",
                    &format!("{} + {}", l.type_name(), r.type_name()),
                    span,
                )),
            },

            BinaryOp::Sub => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                (l, r) => Err(EvalError::type_error(
                    "number - number",
                    &format!("{} - {}", l.type_name(), r.type_name()),
                    span,
                )),
            },

            BinaryOp::Mul => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                (l, r) => Err(EvalError::type_error(
                    "number * number",
                    &format!("{} * {}", l.type_name(), r.type_name()),
                    span,
                )),
            },

            BinaryOp::Div => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => {
                    if r == 0.0 {
                        Err(EvalError::div_by_zero(span))
                    } else {
                        Ok(Value::Number(l / r))
                    }
                }
                (l, r) => Err(EvalError::type_error(
                    "number / number",
                    &format!("{} / {}", l.type_name(), r.type_name()),
                    span,
                )),
            },

            BinaryOp::Pow => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l.powf(r))),
                (l, r) => Err(EvalError::type_error(
                    "number ^ number",
                    &format!("{} ^ {}", l.type_name(), r.type_name()),
                    span,
                )),
            },

            // Comparison operators (require same types)
            BinaryOp::Equal => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l == r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l == r)),
                (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l == r)),
                (l, r) => Err(EvalError::type_error(
                    "matching types",
                    &format!("{} == {}", l.type_name(), r.type_name()),
                    span,
                )),
            },

            BinaryOp::NotEqual => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l != r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l != r)),
                (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l != r)),
                (l, r) => Err(EvalError::type_error(
                    "matching types",
                    &format!("{} != {}", l.type_name(), r.type_name()),
                    span,
                )),
            },

            BinaryOp::Less => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l < r)),
                (l, r) => Err(EvalError::type_error(
                    "number < number or string < string",
                    &format!("{} < {}", l.type_name(), r.type_name()),
                    span,
                )),
            },

            BinaryOp::Greater => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l > r)),
                (l, r) => Err(EvalError::type_error(
                    "number > number or string > string",
                    &format!("{} > {}", l.type_name(), r.type_name()),
                    span,
                )),
            },

            BinaryOp::LessEqual => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l <= r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l <= r)),
                (l, r) => Err(EvalError::type_error(
                    "number <= number or string <= string",
                    &format!("{} <= {}", l.type_name(), r.type_name()),
                    span,
                )),
            },

            BinaryOp::GreaterEqual => match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l >= r)),
                (l, r) => Err(EvalError::type_error(
                    "number >= number or string >= string",
                    &format!("{} >= {}", l.type_name(), r.type_name()),
                    span,
                )),
            },

            // Logical operators
            BinaryOp::And => match (left_val, right_val) {
                (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l && r)),
                (l, r) => Err(EvalError::type_error(
                    "boolean && boolean",
                    &format!("{} && {}", l.type_name(), r.type_name()),
                    span,
                )),
            },

            BinaryOp::Or => match (left_val, right_val) {
                (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l || r)),
                (l, r) => Err(EvalError::type_error(
                    "boolean || boolean",
                    &format!("{} || {}", l.type_name(), r.type_name()),
                    span,
                )),
            },

            // String concatenation (with type coercion)
            BinaryOp::Concat => {
                let left_str = left_val.to_string_coerced();
                let right_str = right_val.to_string_coerced();
                Ok(Value::String(left_str + &right_str))
            }
        }
    }

    /// Evaluate a unary operation
    fn eval_unary(&mut self, op: UnaryOp, operand: &Expr, span: Span) -> Result<Value, EvalError> {
        let val = self.eval_expr(operand)?;

        match op {
            UnaryOp::Not => match val {
                Value::Boolean(b) => Ok(Value::Boolean(!b)),
                _ => Err(EvalError::type_error("boolean", val.type_name(), span)),
            },
            UnaryOp::Neg => match val {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(EvalError::type_error("number", val.type_name(), span)),
            },
        }
    }

    /// Evaluate a function call
    fn eval_call(&mut self, name: &str, args: &[Expr], span: Span) -> Result<Value, EvalError> {
        // Check for built-in functions first
        if let Some(result) = self.eval_builtin(name, args, span)? {
            return Ok(result);
        }

        // Look up user-defined function
        let func = self
            .env
            .get_function(name)
            .ok_or_else(|| EvalError::undefined_func(name, span))?
            .clone();

        // Check arity
        if args.len() != func.params.len() {
            return Err(EvalError::arity_error(
                name,
                func.params.len(),
                args.len(),
                span,
            ));
        }

        // Evaluate arguments
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.eval_expr(arg)?);
        }

        // Create new scope for function
        self.env.push_scope();

        // Bind parameters
        for (param, value) in func.params.iter().zip(arg_values.iter()) {
            self.env.define(param.clone(), value.clone());
        }

        // Evaluate function body
        let result = self.eval_expr(&func.body)?;

        // Pop function scope
        self.env.pop_scope();

        Ok(result)
    }

    /// Evaluate built-in functions
    fn eval_builtin(
        &mut self,
        name: &str,
        args: &[Expr],
        span: Span,
    ) -> Result<Option<Value>, EvalError> {
        match name {
            "print" => {
                if args.len() != 1 {
                    return Err(EvalError::arity_error("print", 1, args.len(), span));
                }
                let val = self.eval_expr(&args[0])?;
                println!("{}", val.to_display_string());
                Ok(Some(Value::Unit))
            }

            "sin" => {
                if args.len() != 1 {
                    return Err(EvalError::arity_error("sin", 1, args.len(), span));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::Number(n) => Ok(Some(Value::Number(n.sin()))),
                    _ => Err(EvalError::type_error("number", val.type_name(), span)),
                }
            }

            "cos" => {
                if args.len() != 1 {
                    return Err(EvalError::arity_error("cos", 1, args.len(), span));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::Number(n) => Ok(Some(Value::Number(n.cos()))),
                    _ => Err(EvalError::type_error("number", val.type_name(), span)),
                }
            }

            "tan" => {
                if args.len() != 1 {
                    return Err(EvalError::arity_error("tan", 1, args.len(), span));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::Number(n) => Ok(Some(Value::Number(n.tan()))),
                    _ => Err(EvalError::type_error("number", val.type_name(), span)),
                }
            }

            "sqrt" => {
                if args.len() != 1 {
                    return Err(EvalError::arity_error("sqrt", 1, args.len(), span));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::Number(n) => Ok(Some(Value::Number(n.sqrt()))),
                    _ => Err(EvalError::type_error("number", val.type_name(), span)),
                }
            }

            "abs" => {
                if args.len() != 1 {
                    return Err(EvalError::arity_error("abs", 1, args.len(), span));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::Number(n) => Ok(Some(Value::Number(n.abs()))),
                    _ => Err(EvalError::type_error("number", val.type_name(), span)),
                }
            }

            "floor" => {
                if args.len() != 1 {
                    return Err(EvalError::arity_error("floor", 1, args.len(), span));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::Number(n) => Ok(Some(Value::Number(n.floor()))),
                    _ => Err(EvalError::type_error("number", val.type_name(), span)),
                }
            }

            "ceil" => {
                if args.len() != 1 {
                    return Err(EvalError::arity_error("ceil", 1, args.len(), span));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::Number(n) => Ok(Some(Value::Number(n.ceil()))),
                    _ => Err(EvalError::type_error("number", val.type_name(), span)),
                }
            }

            "log" => {
                if args.len() != 2 {
                    return Err(EvalError::arity_error("log", 2, args.len(), span));
                }
                let base = self.eval_expr(&args[0])?;
                let val = self.eval_expr(&args[1])?;
                match (base, val) {
                    (Value::Number(b), Value::Number(n)) => Ok(Some(Value::Number(n.log(b)))),
                    (b, v) => Err(EvalError::type_error(
                        "number, number",
                        &format!("{}, {}", b.type_name(), v.type_name()),
                        span,
                    )),
                }
            }

            "pow" => {
                if args.len() != 2 {
                    return Err(EvalError::arity_error("pow", 2, args.len(), span));
                }
                let base = self.eval_expr(&args[0])?;
                let val = self.eval_expr(&args[1])?;
                match (base, val) {
                    (Value::Number(b), Value::Number(n)) => Ok(Some(Value::Number(n.powf(b)))),
                    (b, v) => Err(EvalError::type_error(
                        "number, number",
                        &format!("{}, {}", b.type_name(), v.type_name()),
                        span,
                    )),
                }
            }

            _ => Ok(None), // Not a built-in function
        }
    }

    /// Evaluate an if-else expression
    fn eval_if(
        &mut self,
        condition: &Expr,
        then_branch: &Expr,
        else_branch: &Expr,
    ) -> Result<Value, EvalError> {
        let cond_val = self.eval_expr(condition)?;

        match cond_val {
            Value::Boolean(true) => self.eval_expr(then_branch),
            Value::Boolean(false) => self.eval_expr(else_branch),
            _ => Err(EvalError::type_error(
                "boolean",
                cond_val.type_name(),
                condition.span,
            )),
        }
    }

    /// Evaluate a let-in expression
    fn eval_let(&mut self, declarations: &[VarDecl], body: &Expr) -> Result<Value, EvalError> {
        // Push new scope
        self.env.push_scope();

        // Evaluate and bind variables in order
        for decl in declarations {
            let value = self.eval_expr(&decl.value)?;
            self.env.define(decl.name.clone(), value);
        }

        // Evaluate body
        let result = self.eval_expr(body)?;

        // Pop scope
        self.env.pop_scope();

        Ok(result)
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    fn eval(code: &str) -> Result<Option<Value>, EvalError> {
        let mut parser = Parser::from_source(code).unwrap();
        let program = parser.parse_program().unwrap();
        let mut evaluator = Evaluator::new();

        let mut last_value = None;
        for stmt in &program.statements {
            last_value = evaluator.eval_statement(stmt)?;
        }

        Ok(last_value)
    }

    #[test]
    fn test_eval_number() {
        let result = eval("42;").unwrap();
        assert_eq!(result, Some(Value::Number(42.0)));
    }

    #[test]
    fn test_eval_string() {
        let result = eval(r#""hello";"#).unwrap();
        assert_eq!(result, Some(Value::String("hello".to_string())));
    }

    #[test]
    fn test_eval_boolean() {
        let result = eval("true;").unwrap();
        assert_eq!(result, Some(Value::Boolean(true)));
    }

    #[test]
    fn test_eval_arithmetic() {
        assert_eq!(eval("2 + 3;").unwrap(), Some(Value::Number(5.0)));
        assert_eq!(eval("10 - 3;").unwrap(), Some(Value::Number(7.0)));
        assert_eq!(eval("4 * 5;").unwrap(), Some(Value::Number(20.0)));
        assert_eq!(eval("15 / 3;").unwrap(), Some(Value::Number(5.0)));
        assert_eq!(eval("2 ^ 3;").unwrap(), Some(Value::Number(8.0)));
    }

    #[test]
    fn test_eval_precedence() {
        assert_eq!(eval("2 + 3 * 4;").unwrap(), Some(Value::Number(14.0)));
        assert_eq!(eval("(2 + 3) * 4;").unwrap(), Some(Value::Number(20.0)));
        assert_eq!(eval("2 ^ 3 ^ 2;").unwrap(), Some(Value::Number(512.0))); // 2^(3^2) = 2^9
    }

    #[test]
    fn test_eval_comparison() {
        assert_eq!(eval("5 > 3;").unwrap(), Some(Value::Boolean(true)));
        assert_eq!(eval("5 < 3;").unwrap(), Some(Value::Boolean(false)));
        assert_eq!(eval("5 == 5;").unwrap(), Some(Value::Boolean(true)));
        assert_eq!(eval("5 != 3;").unwrap(), Some(Value::Boolean(true)));
    }

    #[test]
    fn test_eval_logical() {
        assert_eq!(eval("true && true;").unwrap(), Some(Value::Boolean(true)));
        assert_eq!(eval("true && false;").unwrap(), Some(Value::Boolean(false)));
        assert_eq!(eval("false || true;").unwrap(), Some(Value::Boolean(true)));
        assert_eq!(eval("!true;").unwrap(), Some(Value::Boolean(false)));
    }

    #[test]
    fn test_eval_string_concat() {
        assert_eq!(
            eval(r#""hello" @ " world";"#).unwrap(),
            Some(Value::String("hello world".to_string()))
        );
        assert_eq!(
            eval(r#""Answer: " @ 42;"#).unwrap(),
            Some(Value::String("Answer: 42".to_string()))
        );
    }

    #[test]
    fn test_eval_unary() {
        assert_eq!(eval("-5;").unwrap(), Some(Value::Number(-5.0)));
        assert_eq!(eval("!false;").unwrap(), Some(Value::Boolean(true)));
    }

    #[test]
    fn test_eval_let_in() {
        assert_eq!(eval("let x = 42 in x;").unwrap(), Some(Value::Number(42.0)));
        assert_eq!(
            eval("let x = 10, y = 20 in x + y;").unwrap(),
            Some(Value::Number(30.0))
        );
    }

    #[test]
    fn test_eval_let_in_scoping() {
        let result = eval("let x = 10 in let y = x in y;").unwrap();
        assert_eq!(result, Some(Value::Number(10.0)));
    }

    #[test]
    fn test_eval_if_else() {
        assert_eq!(
            eval("if (true) 1 else 2;").unwrap(),
            Some(Value::Number(1.0))
        );
        assert_eq!(
            eval("if (false) 1 else 2;").unwrap(),
            Some(Value::Number(2.0))
        );
        assert_eq!(
            eval("if (5 > 3) \"yes\" else \"no\";").unwrap(),
            Some(Value::String("yes".to_string()))
        );
    }

    #[test]
    fn test_eval_function_def() {
        let code = r#"
function square(x) => x * x;
square(5);
"#;
        assert_eq!(eval(code).unwrap(), Some(Value::Number(25.0)));
    }

    #[test]
    fn test_eval_factorial() {
        let code = r#"
function factorial(n) => if (n <= 1) 1 else n * factorial(n - 1);
factorial(5);
"#;
        assert_eq!(eval(code).unwrap(), Some(Value::Number(120.0)));
    }

    #[test]
    fn test_eval_builtin_math() {
        assert_eq!(eval("sqrt(16);").unwrap(), Some(Value::Number(4.0)));
        assert_eq!(eval("abs(-5);").unwrap(), Some(Value::Number(5.0)));
        assert_eq!(eval("floor(3.7);").unwrap(), Some(Value::Number(3.0)));
        assert_eq!(eval("ceil(3.2);").unwrap(), Some(Value::Number(4.0)));
    }

    #[test]
    fn test_eval_builtin_log() {
        let result = eval("log(2, 8);").unwrap();
        assert_eq!(result, Some(Value::Number(3.0)));
    }

    #[test]
    fn test_eval_constants() {
        let result = eval("PI;").unwrap();
        if let Some(Value::Number(n)) = result {
            assert!((n - std::f64::consts::PI).abs() < 0.0001);
        } else {
            panic!("Expected number");
        }
    }

    #[test]
    fn test_eval_error_div_by_zero() {
        let result = eval("5 / 0;");
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_error_type_mismatch() {
        let result = eval("5 + true;");
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_error_undefined_var() {
        let result = eval("x;");
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_error_undefined_func() {
        let result = eval("foo();");
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_error_arity() {
        let code = r#"
function add(a, b) => a + b;
add(1);
"#;
        let result = eval(code);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_error_redefinition() {
        let code = r#"
function test() => 1;
function test() => 2;
"#;
        let result = eval(code);
        assert!(result.is_err());
    }
}
