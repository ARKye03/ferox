use ferox_core::ast::*;
use ferox_core::lexer::{Position, Span};

fn dummy_span() -> Span {
    Span::new(Position::new(), Position::new())
}

fn main() {
    println!("AST Construction Demo");
    println!("{:=<70}", "");

    // Example 1: Simple arithmetic expression
    // 2 + 3 * 4
    println!("\n1. Arithmetic Expression: 2 + 3 * 4");
    let mul = Expr::binary(
        BinaryOp::Mul,
        Expr::number(3.0, dummy_span()),
        Expr::number(4.0, dummy_span()),
        dummy_span(),
    );
    let add = Expr::binary(
        BinaryOp::Add,
        Expr::number(2.0, dummy_span()),
        mul,
        dummy_span(),
    );
    println!("AST: {}", add);

    // Example 2: Let-in expression
    // let x = 10, y = x * 2 in y + 5
    println!("\n2. Let-in Expression: let x = 10, y = x * 2 in y + 5");
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
    let let_expr = Expr::let_in(vec![decl1, decl2], body, dummy_span());
    println!("AST: {}", let_expr);

    // Example 3: If-else expression
    // if (x > 0) "positive" else "non-positive"
    println!("\n3. If-else Expression: if (x > 0) \"positive\" else \"non-positive\"");
    let condition = Expr::binary(
        BinaryOp::Greater,
        Expr::identifier("x".to_string(), dummy_span()),
        Expr::number(0.0, dummy_span()),
        dummy_span(),
    );
    let if_expr = Expr::if_else(
        condition,
        Expr::string("positive".to_string(), dummy_span()),
        Expr::string("non-positive".to_string(), dummy_span()),
        dummy_span(),
    );
    println!("AST: {}", if_expr);

    // Example 4: Function call
    // print("Hello, " @ "World!")
    println!("\n4. Function Call: print(\"Hello, \" @ \"World!\")");
    let concat = Expr::binary(
        BinaryOp::Concat,
        Expr::string("Hello, ".to_string(), dummy_span()),
        Expr::string("World!".to_string(), dummy_span()),
        dummy_span(),
    );
    let call = Expr::call("print".to_string(), vec![concat], dummy_span());
    println!("AST: {}", call);

    // Example 5: Function definition
    // function factorial(n) => if (n <= 1) 1 else n * factorial(n - 1);
    println!("\n5. Function Definition: function factorial(n) => if (n <= 1) 1 else n * factorial(n - 1);");
    let fac_condition = Expr::binary(
        BinaryOp::LessEqual,
        Expr::identifier("n".to_string(), dummy_span()),
        Expr::number(1.0, dummy_span()),
        dummy_span(),
    );
    let fac_recursive = Expr::binary(
        BinaryOp::Mul,
        Expr::identifier("n".to_string(), dummy_span()),
        Expr::call(
            "factorial".to_string(),
            vec![Expr::binary(
                BinaryOp::Sub,
                Expr::identifier("n".to_string(), dummy_span()),
                Expr::number(1.0, dummy_span()),
                dummy_span(),
            )],
            dummy_span(),
        ),
        dummy_span(),
    );
    let fac_body = Expr::if_else(
        fac_condition,
        Expr::number(1.0, dummy_span()),
        fac_recursive,
        dummy_span(),
    );
    let fac_stmt = Stmt::function_def(
        "factorial".to_string(),
        vec!["n".to_string()],
        fac_body,
        dummy_span(),
    );
    println!("AST: {}", fac_stmt);

    // Example 6: Complete program
    println!("\n6. Complete Program:");
    println!("{:-<70}", "");
    let program = Program::new(vec![
        fac_stmt,
        Stmt::expression(
            Expr::call(
                "factorial".to_string(),
                vec![Expr::number(5.0, dummy_span())],
                dummy_span(),
            ),
            dummy_span(),
        ),
    ]);
    println!("{}", program);

    // Example 7: Logical operations
    // !(x == 0) && (y > 10)
    println!("7. Logical Expression: !(x == 0) && (y > 10)");
    let eq = Expr::binary(
        BinaryOp::Equal,
        Expr::identifier("x".to_string(), dummy_span()),
        Expr::number(0.0, dummy_span()),
        dummy_span(),
    );
    let not = Expr::unary(UnaryOp::Not, eq, dummy_span());
    let gt = Expr::binary(
        BinaryOp::Greater,
        Expr::identifier("y".to_string(), dummy_span()),
        Expr::number(10.0, dummy_span()),
        dummy_span(),
    );
    let and = Expr::binary(BinaryOp::And, not, gt, dummy_span());
    println!("AST: {}", and);
}
