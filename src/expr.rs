use scanner;

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: scanner::Token,
    pub right: Box<Expr>
}

pub struct Grouping {
    pub expression: Box<Expr>
}

#[derive(Debug)]
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
    fn visit_binary<'a>(&self, expr: &'a Binary) -> Option<T> {
        None
    }

    fn visit_grouping<'a>(&self, expr: &'a Grouping) -> Option<T> {
        None
    }

    fn visit_literal<'a>(&self, expr: &'a Literal) -> Option<T> {
        None
    }

    fn visit_unary<'a>(&self, expr: &'a Unary) -> Option<T> {
        None
    }
}

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary)
}

impl Expr {
    fn accept<'a, T: Visitor<U> + 'a, U>(&self, visitor: &'a T) -> Option<U> {
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
        expr.accept(self).unwrap()
    }

    fn parenthesize<'a>(&self, name: &'a str, exprs: Vec<&Expr>) -> String {
        let mut string = String::new();

        string.push('(');
        string.push_str(&name);
        for expr in exprs {
            string.push(' ');
            string.push_str(&expr.accept::<AstPrinter, String>(self).unwrap());
        }

        string.push(')');

        string
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary<'a>(&self, expr: &'a Binary) -> Option<String> {
        Some(self.parenthesize(&format!("{}", expr.operator.lexeme), vec![&*expr.left, &*expr.right]))
    }

    fn visit_grouping<'a>(&self, expr: &'a Grouping) -> Option<String> {
        Some(self.parenthesize("group", vec![&*expr.expression]))
    }

    fn visit_literal<'a>(&self, expr: &'a Literal) -> Option<String> {
        Some(format!("{:?}", expr))
    }

    fn visit_unary<'a>(&self, expr: &'a Unary) -> Option<String> {
        Some(self.parenthesize(&expr.operator.lexeme, vec![&*expr.right]))
    }
}