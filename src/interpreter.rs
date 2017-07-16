use ast::{Literal, Binary, Grouping, Unary, Expr, ExprVisitor, StmtVisitor, Stmt, Variable, Var, Assign};
use scanner::{TokenType, Token};
use std::error::Error;
use std::fmt::{Display, Result as FmtResult, Formatter};
use std::collections::HashMap;
use super::Lox;

pub struct Interpreter {
    environment: Environment
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new()
        }
    }

    pub fn interpret<'a>(&mut self, lox: &mut Lox, statements: &'a Vec<Stmt>) {
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

    fn execute<'a>(&mut self, stmt: &'a Stmt) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    fn evaluate<'a>(&mut self, expr: &'a Expr) -> Result<Literal, RuntimeError> {
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
            _ => Err(RuntimeError(operator.clone(), "Operand must be a numbers".to_string()))
        }
    }

    fn is_equal(&self, left: Literal, right: Literal) -> bool {
        left == right
    }
}

impl ExprVisitor<Result<Literal, RuntimeError>> for Interpreter {
    fn visit_binary<'a>(&mut self, binary: &'a Binary) -> Result<Literal, RuntimeError> {
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

    fn visit_grouping<'a>(&mut self, grouping: &'a Grouping) -> Result<Literal, RuntimeError> {
        self.evaluate(&*grouping.expression)
    }

    fn visit_literal<'a>(&mut self, literal: &'a Literal) -> Result<Literal, RuntimeError> {
        Ok(literal.clone())
    }

    fn visit_unary<'a>(&mut self, unary: &'a Unary) -> Result<Literal, RuntimeError> {
        let right = self.evaluate(&*unary.right)?;

        Ok(match unary.operator.token_type {
            TokenType::Minus => Literal::Number(-self.cast_to_float(right, &unary.operator)?),
            TokenType::Bang => Literal::Bool(self.is_truthy(right)),
            _ => unreachable!()
        })
    }

    fn visit_variable<'a>(&mut self, variable: &'a Variable) -> Result<Literal, RuntimeError> {
        self.environment.get(&variable.name)
    }

    fn visit_assign<'a>(&mut self, assign: &'a Assign) -> Result<Literal, RuntimeError> {
        let value = self.evaluate(&*assign.value)?;

        self.environment.assign(&assign.name, value.clone())?;
        Ok(value)
    }
}

impl StmtVisitor<Result<(), RuntimeError>> for Interpreter {
    fn visit_expr<'a>(&mut self, expr: &'a Expr) -> Result<(), RuntimeError> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_print<'a>(&mut self, print: &'a Expr) -> Result<(), RuntimeError> {
        let result = self.evaluate(print)?;
        println!("{}", self.stringify(result));
        Ok(())
    }

    fn visit_var<'a>(&mut self, stmt: &'a Var) -> Result<(), RuntimeError> {
        let value = if stmt.initializer.is_some() {
            self.evaluate(&stmt.initializer.clone().unwrap())?
        } else {
            Literal::Nil
        };

        self.environment.define(stmt.name.lexeme.clone(), value);
        Ok(())
    }
}

#[derive(Debug)]
pub struct RuntimeError(pub Token, pub String);

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

pub struct Environment {
    pub values: HashMap<String, Literal>
}

impl Environment {
    fn new() -> Self {
        Environment {
            values: HashMap::new()
        }
    }
    fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    fn get<'a>(&self, name: &'a Token) -> Result<Literal, RuntimeError> {
        self.values.get(&name.lexeme).cloned().ok_or(RuntimeError(name.clone(), format!("Undefined variable '{}'.", name.lexeme)))
    }

    fn assign<'a>(&mut self, name: &'a Token, value: Literal) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else {
            Err(RuntimeError(name.clone(), format!("Undefined variable '{}'.", name.lexeme)))
        }

    }
}