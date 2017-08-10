use scanner;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: scanner::Token,
    pub right: Box<Expr>
}

#[derive(Clone, Debug)]
pub struct Call {
    pub callee: Box<Expr>,
    pub paren: scanner::Token,
    pub arguments: Vec<Expr>
}

#[derive(Clone, Debug)]
pub struct Grouping {
    pub expression: Box<Expr>
}

#[derive(Debug, Clone)]
pub enum Literal {
    Callable(Rc<Callable>),
    String(String),
    Number(f64),
    Bool(bool),
    Nil
}

impl PartialEq for Literal {
    fn eq(&self, other: &Literal) -> bool {
        match (self, other) {
            (&Literal::String(ref string1), &Literal::String(ref string2)) => string1 == string2,
            (&Literal::Number(ref number1), &Literal::Number(ref number2)) => number1 == number2,
            (&Literal::Bool(ref bool1), &Literal::Bool(ref bool2)) => bool1 == bool2,
            (&Literal::Nil, &Literal::Nil) => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: scanner::Token,
    pub right: Box<Expr>
}

#[derive(Clone, Debug)]
pub struct Unary {
    pub operator: scanner::Token,
    pub right: Box<Expr>
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: scanner::Token
}

#[derive(Clone, Debug)]
pub struct Assign {
    pub name: scanner::Token,
    pub value: Box<Expr>
}

pub trait ExprVisitor<T> {
    fn visit_binary<'a>(&mut self, _: &'a Binary) -> T;

    fn visit_call<'a>(&mut self, _: &'a Call) -> T;

    fn visit_grouping<'a>(&mut self, _: &'a Grouping) -> T;

    fn visit_literal<'a>(&mut self, _: &'a Literal) -> T;

    fn visit_unary<'a>(&mut self, _: &'a Unary) -> T;

    fn visit_variable<'a>(&mut self, _: &'a Variable) -> T;

    fn visit_assign<'a>(&mut self, _: &'a Assign) -> T;

    fn visit_logical<'a>(&mut self, _: &'a Logical) -> T;
}

#[derive(Clone, Debug)]
pub enum Expr {
    Binary(Binary),
    Call(Call),
    Grouping(Grouping),
    Literal(Literal),
    Logical(Logical),
    Unary(Unary),
    Variable(Variable),
    Assign(Assign)
}

impl Expr {
    pub fn accept<'a, T: ExprVisitor<U> + 'a, U>(&self, visitor: &'a mut T) -> U {
        match *self {
            Expr::Binary(ref v) => visitor.visit_binary(v),
            Expr::Call(ref v) => visitor.visit_call(v),
            Expr::Grouping(ref v) => visitor.visit_grouping(v),
            Expr::Literal(ref v) => visitor.visit_literal(v),
            Expr::Unary(ref v) => visitor.visit_unary(v),
            Expr::Variable(ref v) => visitor.visit_variable(v),
            Expr::Assign(ref v) => visitor.visit_assign(v),
            Expr::Logical(ref v) => visitor.visit_logical(v)
        }
    }
}
//
//impl From<Binary> for Expr {
//    fn from(v: Binary) -> Self {
//        Expr::Binary(v)
//    }
//}
//
//impl From<Grouping> for Expr {
//    fn from(v: Grouping) -> Self {
//        Expr::Grouping(v)
//    }
//}
//
//impl From<Literal> for Expr {
//    fn from(v: Literal) -> Self {
//        Expr::Literal(v)
//    }
//}
//
//impl From<Unary> for Expr {
//    fn from(v: Unary) -> Self {
//        Expr::Unary(v)
//    }
//}

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression(Expr),
    If(If),
    Print(Expr),
    Var(Var),
    While(While),
    Block(Block)
}

#[derive(Clone, Debug)]
pub struct If {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>
}

#[derive(Clone, Debug)]
pub struct Var {
    pub name: scanner::Token,
    pub initializer: Option<Expr>
}

#[derive(Clone, Debug)]
pub struct While {
    pub condition: Expr,
    pub body: Box<Stmt>
}

#[derive(Clone, Debug)]
pub struct Block {
    pub statements: Vec<Stmt>
}

impl Stmt {
    pub fn accept<'a, T: StmtVisitor<U> + 'a, U>(&self, visitor: &'a mut T) -> U {
        match *self {
            Stmt::Expression(ref v) => visitor.visit_expr(v),
            Stmt::Print(ref v) => visitor.visit_print(v),
            Stmt::Var(ref v) => visitor.visit_var(v),
            Stmt::Block(ref v) => visitor.visit_block(v),
            Stmt::If(ref v) => visitor.visit_if(v),
            Stmt::While(ref v) => visitor.visit_while(v)
        }
    }
}

pub trait StmtVisitor<T> {
    fn visit_expr<'a>(&mut self, _: &'a Expr) -> T;
    fn visit_print<'a>(&mut self, _: &'a Expr) -> T;
    fn visit_var<'a>(&mut self, _: &'a Var) -> T;
    fn visit_block<'a>(&mut self, _: &'a Block) -> T;
    fn visit_if<'a>(&mut self, _: &'a If) -> T;
    fn visit_while<'a>(&mut self, _: &'a While) -> T;
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize<'a>(&mut self, name: &'a str, exprs: Vec<&Expr>) -> String {
        let mut string = String::new();

        string.push('(');
        string.push_str(&name);
        for expr in exprs {
            string.push(' ');
            string.push_str(&expr.accept::<AstPrinter, String>(self));
        }

        string.push(')');

        string
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary<'a>(&mut self, expr: &'a Binary) -> String {
        self.parenthesize(&format!("{}", expr.operator.lexeme), vec![&*expr.left, &*expr.right])
    }

    fn visit_call<'a>(&mut self, _: &'a Call) -> String {
        unimplemented!()
    }

    fn visit_grouping<'a>(&mut self, expr: &'a Grouping) -> String {
        self.parenthesize("group", vec![&*expr.expression])
    }

    fn visit_literal<'a>(&mut self, expr: &'a Literal) -> String {
        format!("{:?}", expr)
    }

    fn visit_unary<'a>(&mut self, expr: &'a Unary) -> String {
        self.parenthesize(&expr.operator.lexeme, vec![&*expr.right])
    }

    fn visit_variable<'a>(&mut self, expr: &'a Variable) -> String {
        format!("{}", expr.name.lexeme)
    }

    fn visit_assign<'a>(&mut self, expr: &'a Assign) -> String {
        self.parenthesize(&expr.name.lexeme, vec![&*expr.value])
    }

    fn visit_logical<'a>(&mut self, expr: &'a Logical) -> String {
        self.parenthesize(&format!("{}", expr.operator.lexeme), vec![&*expr.left, &*expr.right])
    }
}

pub trait Callable: ::std::fmt::Debug {
    fn call(&self, interpreter: &mut ::interpreter::Interpreter, arguments: Vec<Literal>) -> Result<Literal, ::interpreter::RuntimeError>;
    fn arity(&self) -> usize;
}