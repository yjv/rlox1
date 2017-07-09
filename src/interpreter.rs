use expr::{Literal, Binary, Grouping, Unary, Expr, Visitor};
use scanner::{TokenType, Token};
use std::error::Error;
use std::fmt::{Display, Result as FmtResult, Formatter};

pub struct Interpreter;

impl Interpreter {
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

impl Visitor<Result<Literal, RuntimeError>> for Interpreter {
    fn visit_binary<'a>(&self, binary: &'a Binary) -> Result<Literal, RuntimeError> {
        let left = self.evaluate(&*binary.left)?;
        let right = self.evaluate(&*binary.right)?;

        Ok(match binary.operator.token_type {
            TokenType::Minus => Literal::Number(self.cast_to_float(left, &binary.operator)? - self.cast_to_float(right, &binary.operator)?),
            TokenType::Slash => Literal::Number(self.cast_to_float(left, &binary.operator)? / self.cast_to_float(right, &binary.operator)?),
            TokenType::Star => Literal::Number(self.cast_to_float(left, &binary.operator)? * self.cast_to_float(right, &binary.operator)?),
            TokenType::Plus => {
                match left {
                    Literal::String(mut left) => match right {
                        Literal::String(right) => {
                            left.push_str(&right[..]);
                            Literal::String(left)
                        },
                        right => Literal::Number(self.cast_to_float(Literal::String(left), &binary.operator)? + self.cast_to_float(right, &binary.operator)?)
                    },
                    left => Literal::Number(self.cast_to_float(left, &binary.operator)? + self.cast_to_float(right, &binary.operator)?)
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
}

#[derive(Debug)]
pub struct RuntimeError(Token, &'static str);

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