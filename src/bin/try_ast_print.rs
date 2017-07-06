extern crate lox1;
use lox1::expr::*;
use lox1::scanner::*;

fn main() {
    let ast = Binary {
        left: Box::new(Unary {
            operator: Token {
                lexeme: "-".to_string(),
                line: 0,
                token_type: TokenType::Minus
            },
            right: Box::new(Literal::Number(123.0).into())
        }.into()),
        operator: Token {
            lexeme: "*".to_string(),
            line: 0,
            token_type: TokenType::Star
        },
        right: Box::new(Grouping {
            expression: Box::new(Literal::Number(45.67).into())
        }.into())
    }.into();

    let printer = AstPrinter;
    println!("{}", printer.print(&ast));
}