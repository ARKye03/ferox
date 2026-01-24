//! This example shows the complete AST structure of FEROX

fn main() {
    println!("FEROX AST Structure");
    println!("{:=<70}", "");

    println!("\n## Operators");
    println!("{:-<70}", "");
    println!("BinaryOp:");
    println!("  Arithmetic:  + - * / ^");
    println!("  Comparison:  == != < > <= >=");
    println!("  Logical:     && ||");
    println!("  String:      @ (concatenation)");
    println!("\nUnaryOp:");
    println!("  Logical:     ! (not)");
    println!("  Arithmetic:  - (negation)");

    println!("\n## Expression Types (ExprKind)");
    println!("{:-<70}", "");
    println!("Literals:");
    println!("  - Number(f64)           e.g., 42, 3.14");
    println!("  - String(String)        e.g., \"hello\"");
    println!("  - Boolean(bool)         e.g., true, false");
    println!("  - Identifier(String)    e.g., x, foo");

    println!("\nCompound Expressions:");
    println!("  - Binary {{ op, left, right }}");
    println!("      e.g., 2 + 3, x == y, \"hello\" @ \"world\"");
    println!("  - Unary {{ op, operand }}");
    println!("      e.g., !x, -5");
    println!("  - Call {{ name, args }}");
    println!("      e.g., print(x), sin(3.14), max(a, b)");
    println!("  - If {{ condition, then_branch, else_branch }}");
    println!("      e.g., if (x > 0) \"pos\" else \"neg\"");
    println!("  - Let {{ declarations, body }}");
    println!("      e.g., let x = 1, y = 2 in x + y");

    println!("\n## Statement Types (StmtKind)");
    println!("{:-<70}", "");
    println!("  - Expression(Expr)");
    println!("      Any expression followed by semicolon");
    println!("      e.g., print(42); or 2 + 3;");
    println!("  - FunctionDef {{ name, params, body }}");
    println!("      e.g., function square(x) => x * x;");

    println!("\n## Program Structure");
    println!("{:-<70}", "");
    println!("  Program {{ statements: Vec<Stmt> }}");
    println!("      A sequence of statements");

    println!("\n## Position Tracking");
    println!("{:-<70}", "");
    println!("Every Expr, Stmt, and VarDecl includes a Span:");
    println!("  Span {{ start: Position, end: Position }}");
    println!("  Position {{ line, column, offset }}");
    println!("  → Used for error reporting with exact locations");

    println!("\n## Helper Functions");
    println!("{:-<70}", "");
    println!("Expr constructors:");
    println!("  - Expr::number(value, span)");
    println!("  - Expr::string(value, span)");
    println!("  - Expr::boolean(value, span)");
    println!("  - Expr::identifier(name, span)");
    println!("  - Expr::binary(op, left, right, span)");
    println!("  - Expr::unary(op, operand, span)");
    println!("  - Expr::call(name, args, span)");
    println!("  - Expr::if_else(cond, then, else, span)");
    println!("  - Expr::let_in(decls, body, span)");

    println!("\nStmt constructors:");
    println!("  - Stmt::expression(expr, span)");
    println!("  - Stmt::function_def(name, params, body, span)");

    println!("\n## Display Trait");
    println!("{:-<70}", "");
    println!("All AST nodes implement Display for pretty printing:");
    println!("  - Expressions shown in Lisp-like notation");
    println!("  - Binary ops: (left op right)");
    println!("  - Unary ops: (op operand)");
    println!("  - If: (if cond then then else else)");
    println!("  - Let: (let x = val, y = val in body)");

    println!("\n{:=<70}", "");
}
