use scanner;

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: scanner::Token,
    pub right: Box<Expr>
}

pub struct Grouping {
    pub expression: Box<Expr>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
    Nil
}

pub struct Unary {
    pub operator: scanner::Token,
    pub right: Box<Expr>
}

pub trait Visitor<T> {
    fn visit_binary<'a>(&self, _: &'a Binary) -> T;

    fn visit_grouping<'a>(&self, _: &'a Grouping) -> T;

    fn visit_literal<'a>(&self, _: &'a Literal) -> T;

    fn visit_unary<'a>(&self, _: &'a Unary) -> T;
}

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary)
}

impl Expr {
    pub fn accept<'a, T: Visitor<U> + 'a, U>(&self, visitor: &'a T) -> U {
        match *self {
            Expr::Binary(ref v) => visitor.visit_binary(v),
            Expr::Grouping(ref v) => visitor.visit_grouping(v),
            Expr::Literal(ref v) => visitor.visit_literal(v),
            Expr::Unary(ref v) => visitor.visit_unary(v)
        }
    }
}

impl From<Binary> for Expr {
    fn from(v: Binary) -> Self {
        Expr::Binary(v)
    }
}

impl From<Grouping> for Expr {
    fn from(v: Grouping) -> Self {
        Expr::Grouping(v)
    }
}

impl From<Literal> for Expr {
    fn from(v: Literal) -> Self {
        Expr::Literal(v)
    }
}

impl From<Unary> for Expr {
    fn from(v: Unary) -> Self {
        Expr::Unary(v)
    }
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize<'a>(&self, name: &'a str, exprs: Vec<&Expr>) -> String {
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

impl Visitor<String> for AstPrinter {
    fn visit_binary<'a>(&self, expr: &'a Binary) -> String {
        self.parenthesize(&format!("{}", expr.operator.lexeme), vec![&*expr.left, &*expr.right])
    }

    fn visit_grouping<'a>(&self, expr: &'a Grouping) -> String {
        self.parenthesize("group", vec![&*expr.expression])
    }

    fn visit_literal<'a>(&self, expr: &'a Literal) -> String {
        format!("{:?}", expr)
    }

    fn visit_unary<'a>(&self, expr: &'a Unary) -> String {
        self.parenthesize(&expr.operator.lexeme, vec![&*expr.right])
    }
}