//! Environment and scope management for FEROX
//!
//! Manages variable scopes and function definitions during evaluation.

use crate::ast::Expr;
use crate::lexer::Span;
use std::collections::HashMap;

// ============================================================================
// Value Types
// ============================================================================

/// Runtime values in FEROX
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Unit, // For print() and other side-effect operations
}

impl Value {
    /// Get the type name for error messages
    pub fn type_name(&self) -> &str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::Unit => "unit",
        }
    }

    /// Convert value to string for display
    pub fn to_display_string(&self) -> String {
        match self {
            Value::Number(n) => {
                // Format numbers nicely (no trailing .0 for integers)
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::String(s) => s.clone(),
            Value::Boolean(b) => format!("{}", b),
            Value::Unit => String::new(),
        }
    }

    /// Convert value to string (with quotes for strings, used by @ operator)
    pub fn to_string_coerced(&self) -> String {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::String(s) => s.clone(),
            Value::Boolean(b) => format!("{}", b),
            Value::Unit => String::new(),
        }
    }
}

// ============================================================================
// User-Defined Functions
// ============================================================================

/// User-defined function
#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Expr,
}

// ============================================================================
// Environment (Scope Management)
// ============================================================================

/// Environment for variable and function storage
pub struct Environment {
    /// Variable scopes (stack of maps)
    scopes: Vec<HashMap<String, Value>>,
    /// Global function definitions (persistent across statements)
    functions: HashMap<String, Function>,
}

impl Environment {
    /// Create a new environment
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()], // Start with global scope
            functions: HashMap::new(),
        }
    }

    // ------------------------------------------------------------------------
    // Variable management
    // ------------------------------------------------------------------------

    /// Define a variable in the current scope
    pub fn define(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    /// Get a variable value (searches from innermost to outermost scope)
    pub fn get(&self, name: &str) -> Option<&Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value);
            }
        }
        None
    }

    /// Push a new scope
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pop the current scope
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    // ------------------------------------------------------------------------
    // Function management
    // ------------------------------------------------------------------------

    /// Define a function (global, persistent)
    pub fn define_function(&mut self, name: String, func: Function) -> Result<(), String> {
        // Check if function already exists (no redefinition allowed)
        if self.functions.contains_key(&name) {
            return Err(format!("Function '{}' is already defined", name));
        }
        self.functions.insert(name, func);
        Ok(())
    }

    /// Get a function definition
    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.get(name)
    }

    /// Check if a name is a function
    pub fn is_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Runtime Error
// ============================================================================

/// Runtime or semantic error
#[derive(Debug, Clone)]
pub struct EvalError {
    pub message: String,
    pub span: Span,
}

impl EvalError {
    pub fn new(message: String, span: Span) -> Self {
        Self { message, span }
    }

    /// Type mismatch error
    pub fn type_error(expected: &str, got: &str, span: Span) -> Self {
        Self::new(
            format!("Type error: expected {}, got {}", expected, got),
            span,
        )
    }

    /// Undefined variable error
    pub fn undefined_var(name: &str, span: Span) -> Self {
        Self::new(format!("Undefined variable '{}'", name), span)
    }

    /// Undefined function error
    pub fn undefined_func(name: &str, span: Span) -> Self {
        Self::new(format!("Undefined function '{}'", name), span)
    }

    /// Arity mismatch error
    pub fn arity_error(name: &str, expected: usize, got: usize, span: Span) -> Self {
        Self::new(
            format!(
                "Function '{}' expects {} argument(s), got {}",
                name, expected, got
            ),
            span,
        )
    }

    /// Division by zero error
    pub fn div_by_zero(span: Span) -> Self {
        Self::new("Division by zero".to_string(), span)
    }
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Runtime error at {}: {}", self.span, self.message)
    }
}

impl std::error::Error for EvalError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Position;

    fn dummy_span() -> Span {
        Span::new(Position::new(), Position::new())
    }

    #[test]
    fn test_value_type_names() {
        assert_eq!(Value::Number(42.0).type_name(), "number");
        assert_eq!(Value::String("hello".to_string()).type_name(), "string");
        assert_eq!(Value::Boolean(true).type_name(), "boolean");
        assert_eq!(Value::Unit.type_name(), "unit");
    }

    #[test]
    fn test_value_display() {
        assert_eq!(Value::Number(42.0).to_display_string(), "42");
        assert_eq!(Value::Number(3.14).to_display_string(), "3.14");
        assert_eq!(
            Value::String("hello".to_string()).to_display_string(),
            "hello"
        );
        assert_eq!(Value::Boolean(true).to_display_string(), "true");
    }

    #[test]
    fn test_environment_variables() {
        let mut env = Environment::new();

        env.define("x".to_string(), Value::Number(42.0));
        assert_eq!(env.get("x"), Some(&Value::Number(42.0)));
        assert_eq!(env.get("y"), None);
    }

    #[test]
    fn test_environment_scopes() {
        let mut env = Environment::new();

        env.define("x".to_string(), Value::Number(1.0));

        env.push_scope();
        env.define("x".to_string(), Value::Number(2.0));
        env.define("y".to_string(), Value::Number(3.0));

        // Inner scope shadows outer scope
        assert_eq!(env.get("x"), Some(&Value::Number(2.0)));
        assert_eq!(env.get("y"), Some(&Value::Number(3.0)));

        env.pop_scope();

        // Back to outer scope
        assert_eq!(env.get("x"), Some(&Value::Number(1.0)));
        assert_eq!(env.get("y"), None);
    }

    #[test]
    fn test_function_definition() {
        let mut env = Environment::new();

        let func = Function {
            params: vec!["x".to_string()],
            body: Expr::identifier("x".to_string(), dummy_span()),
        };

        env.define_function("identity".to_string(), func.clone())
            .unwrap();

        assert!(env.is_function("identity"));
        assert_eq!(env.get_function("identity").unwrap().params, func.params);
    }

    #[test]
    fn test_function_no_redefinition() {
        let mut env = Environment::new();

        let func = Function {
            params: vec![],
            body: Expr::number(42.0, dummy_span()),
        };

        env.define_function("test".to_string(), func.clone())
            .unwrap();

        let result = env.define_function("test".to_string(), func);
        assert!(result.is_err());
    }
}
