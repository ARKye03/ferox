//! FEROX Core - Functional Expression Runtime for Operations and eXecution
//!
//! This crate contains the core interpreter logic:
//! - Lexer: Tokenization
//! - Parser: AST construction
//! - AST: Abstract Syntax Tree definitions
//! - Evaluator: Expression evaluation
//! - Environment: Scope and variable management
//! - Error: Error types and handling
//!
//! This is a library crate with NO external dependencies - only Rust std.

// Module declarations (uncomment as you implement them)
// pub mod lexer;
// pub mod parser;
// pub mod ast;
// pub mod evaluator;
// pub mod environment;
// pub mod error;

// Re-exports for convenient access
// pub use lexer::*;
// pub use parser::*;
// pub use ast::*;
// pub use evaluator::*;
// pub use environment::*;
// pub use error::*;

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder_test() {
        assert_eq!(2 + 2, 4);
    }
}
