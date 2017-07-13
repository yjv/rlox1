use scanner;

#[derive(Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: scanner::Token,
    pub right: Box<Expr>
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Unary {
    pub operator: scanner::Token,
    pub right: Box<Expr>
}

#[derive(Clone)]
pub struct Variable {
    pub name: scanner::Token
}

pub trait ExprVisitor<T> {
    fn visit_binary<'a>(&self, _: &'a Binary) -> T;

    fn visit_grouping<'a>(&self, _: &'a Grouping) -> T;

    fn visit_literal<'a>(&self, _: &'a Literal) -> T;

    fn visit_unary<'a>(&self, _: &'a Unary) -> T;

    fn visit_variable<'a>(&self, _: &'a Variable) -> T;
}

#[derive(Clone)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Variable(Variable)
}

impl Expr {
    pub fn accept<'a, T: ExprVisitor<U> + 'a, U>(&self, visitor: &'a T) -> U {
        match *self {
            Expr::Binary(ref v) => visitor.visit_binary(v),
            Expr::Grouping(ref v) => visitor.visit_grouping(v),
            Expr::Literal(ref v) => visitor.visit_literal(v),
            Expr::Unary(ref v) => visitor.visit_unary(v),
            Expr::Variable(ref v) => visitor.visit_variable(v)
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

pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Var)
}

pub struct Var {
    pub name: scanner::Token,
    pub initializer: Option<Expr>
}

impl Stmt {
    pub fn accept<'a, T: StmtVisitor<U> + 'a, U>(&self, visitor: &'a mut T) -> U {
        match *self {
            Stmt::Expression(ref v) => visitor.visit_expr(v),
            Stmt::Print(ref v) => visitor.visit_print(v),
            Stmt::Var(ref v) => visitor.visit_var(v)
        }
    }
}

pub trait StmtVisitor<T> {
    fn visit_expr<'a>(&self, _: &'a Expr) -> T;
    fn visit_print<'a>(&self, _: &'a Expr) -> T;
    fn visit_var<'a>(&mut self, _: &'a Var) -> T;
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

impl ExprVisitor<String> for AstPrinter {
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

    fn visit_variable<'a>(&self, expr: &'a Variable) -> String {
        format!("{}", expr.name.lexeme)
    }
}