use ast::{Literal, Binary, Grouping, Unary, Expr, ExprVisitor, StmtVisitor, Stmt, Variable, Var};
use scanner::{TokenType, Token};
use std::error::Error;
use std::fmt::{Display, Result as FmtResult, Formatter};
use super::Lox;

pub struct Interpreter;

impl Interpreter {
    pub fn interpret<'a>(&self, lox: &mut Lox, statements: &'a Vec<Stmt>) {
        for statement in statements {
            match self.execute(statement) {
                Ok(_) => (),
                Err(error) => {
                    lox.runtime_error(error);
                    return;
                }
            }
        }
    }

    fn stringify(&self, value: Literal) -> String {
        match value {
            Literal::Nil => "nil".to_string(),
            Literal::Number(number) => {
                let value = number.to_string();
                value
            },
            Literal::String(value) => value,
            Literal::Bool(value) => value.to_string()
        }
    }

    fn execute<'a>(&self, stmt: &'a Stmt) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    fn evaluate<'a>(&self, expr: &'a Expr) -> Result<Literal, RuntimeError> {
        expr.accept(self)
    }

    fn is_truthy(&self, literal: Literal) -> bool {
        match literal {
            Literal::Bool(bool) if !bool => false,
            Literal::Nil => false,
            _ => true
        }
    }

    fn cast_to_float<'a>(&self, literal: Literal, operator: &'a Token) -> Result<f64, RuntimeError> {
        match literal {
            Literal::Number(number) => Ok(number),
            _ => Err(RuntimeError(operator.clone(), "Operand must be a numbers"))
        }
    }

    fn is_equal(&self, left: Literal, right: Literal) -> bool {
        left == right
    }
}

impl ExprVisitor<Result<Literal, RuntimeError>> for Interpreter {
    fn visit_binary<'a>(&self, binary: &'a Binary) -> Result<Literal, RuntimeError> {
        let left = self.evaluate(&*binary.left)?;
        let right = self.evaluate(&*binary.right)?;

        Ok(match binary.operator.token_type {
            TokenType::Minus => Literal::Number(self.cast_to_float(left, &binary.operator)? - self.cast_to_float(right, &binary.operator)?),
            TokenType::Slash => Literal::Number(self.cast_to_float(left, &binary.operator)? / self.cast_to_float(right, &binary.operator)?),
            TokenType::Star => Literal::Number(self.cast_to_float(left, &binary.operator)? * self.cast_to_float(right, &binary.operator)?),
            TokenType::Plus => {
                match (left, right) {
                    (Literal::String(mut left), Literal::String(right)) => {
                        left.push_str(&right[..]);
                        Literal::String(left)
                    },
                    (left, right) => Literal::Number(self.cast_to_float(left, &binary.operator)? + self.cast_to_float(right, &binary.operator)?)
                }
            },
            TokenType::Greater => Literal::Bool(self.cast_to_float(left, &binary.operator)? > self.cast_to_float(right, &binary.operator)?),
            TokenType::GreaterEqual => Literal::Bool(self.cast_to_float(left, &binary.operator)? >= self.cast_to_float(right, &binary.operator)?),
            TokenType::Less => Literal::Bool(self.cast_to_float(left, &binary.operator)? < self.cast_to_float(right, &binary.operator)?),
            TokenType::LessEqual => Literal::Bool(self.cast_to_float(left, &binary.operator)? <= self.cast_to_float(right, &binary.operator)?),
            TokenType::BangEqual => Literal::Bool(!self.is_equal(left, right)),
            TokenType::EqualEqual => Literal::Bool(self.is_equal(left, right)),
            _ => unreachable!()
        })
    }

    fn visit_grouping<'a>(&self, grouping: &'a Grouping) -> Result<Literal, RuntimeError> {
        self.evaluate(&*grouping.expression)
    }

    fn visit_literal<'a>(&self, literal: &'a Literal) -> Result<Literal, RuntimeError> {
        Ok(literal.clone())
    }

    fn visit_unary<'a>(&self, unary: &'a Unary) -> Result<Literal, RuntimeError> {
        let right = self.evaluate(&*unary.right)?;

        Ok(match unary.operator.token_type {
            TokenType::Minus => Literal::Number(-self.cast_to_float(right, &unary.operator)?),
            TokenType::Bang => Literal::Bool(self.is_truthy(right)),
            _ => unreachable!()
        })
    }

    fn visit_variable<'a>(&self, _: &'a Variable) -> Result<Literal, RuntimeError> {
        unimplemented!()
    }
}

impl StmtVisitor<Result<(), RuntimeError>> for Interpreter {
    fn visit_expr<'a>(&self, expr: &'a Expr) -> Result<(), RuntimeError> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_print<'a>(&self, print: &'a Expr) -> Result<(), RuntimeError> {
        let result = self.evaluate(print)?;
        println!("{}", self.stringify(result));
        Ok(())
    }

    fn visit_var<'a>(&self, _: &'a Var) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct RuntimeError(pub Token, pub &'static str);

impl Error for RuntimeError {
    fn description(&self) -> &str {
        "A runtime error occurred"
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "A runtime error occurred: {}", self.1)
    }
}