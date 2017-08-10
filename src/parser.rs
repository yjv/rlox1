use scanner::{TokenType, Token};
use ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            current: 0
        }
    }

    pub fn parse(&mut self, lox: &mut super::Lox) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if let Some(statement) = self.declaration(lox) {
                statements.push(statement);
            }
        }

        statements
    }

    fn declaration(&mut self, lox: &mut super::Lox) -> Option<Stmt> {
        match if self.match_token_types(vec![TokenType::Var]) {
            self.var_declaration(lox)
        } else {
            self.statement(lox)
        } {
            Ok(stmt) => Some(stmt),
            Err(_) => {
                self.synchronize();
                None
            }
        }
    }

    fn var_declaration(&mut self, lox: &mut super::Lox) -> Result<Stmt, ()> {
        let name = self.consume(lox, TokenType::Identifier, "Expect variable name.".to_string())?;

        let mut initializer = None;
        if self.match_token_types(vec![TokenType::Equal]) {
            initializer = Some(self.expression(lox)?);
        }

        self.consume(lox, TokenType::Semicolon, "Expect ';' after variable declaration.".to_string())?;
        Ok(Stmt::Var(Var {
            name: name,
            initializer: initializer
        }))
    }

    fn statement(&mut self, lox: &mut super::Lox) -> Result<Stmt, ()> {
        if self.match_token_types(vec![TokenType::For]) {
            self.for_statement(lox)
        } else if self.match_token_types(vec![TokenType::If]) {
            self.if_statement(lox)
        } else if self.match_token_types(vec![TokenType::Print]) {
            self.print_statement(lox)
        } else if self.match_token_types(vec![TokenType::While]) {
            self.while_statement(lox)
        } else if self.match_token_types(vec![TokenType::LeftBrace]) {
            Ok(Stmt::Block(Block { statements: self.block(lox)? }))
        } else {
            self.expression_statement(lox)
        }
    }

    fn for_statement(&mut self, lox: &mut super::Lox) -> Result<Stmt, ()> {
        self.consume(lox, TokenType::LeftParen, "Expect '(' after 'for'.".to_string())?;

        let initializer = if self.match_token_types(vec![TokenType::Semicolon]) {
            None
        } else if self.match_token_types(vec![TokenType::Var]) {
            Some(self.var_declaration(lox)?)
        } else {
            Some(self.expression_statement(lox)?)
        };

        let condition = if self.check(TokenType::Semicolon) {
            Expr::Literal(Literal::Bool(true))
        } else {
            self.expression(lox)?
        };

        self.consume(lox, TokenType::Semicolon, "Expect ';' after loop condition.".to_string())?;

        let increment = if self.check(TokenType::RightParen) {
            None
        } else {
            Some(self.expression(lox)?)
        };

        self.consume(lox, TokenType::RightParen, "Expect ')' after for clauses.".to_string())?;

        let mut body = self.statement(lox)?;

        if let Some(increment) = increment {
            body = Stmt::Block(Block {
                statements: vec![
                    body,
                    Stmt::Expression(increment)
                ]
            });
        }

        body = Stmt::While(While {
            condition: condition,
            body: Box::new(body)
        });

        if let Some(initializer) = initializer {
            body = Stmt::Block(Block {
                statements: vec![
                    initializer,
                    body
                ]
            })
        }

        Ok(body)
    }

    fn if_statement(&mut self, lox: &mut super::Lox) -> Result<Stmt, ()> {
        self.consume(lox, TokenType::LeftParen, "Expect '(' after 'if'.".to_string())?;
        let condition = self.expression(lox)?;
        self.consume(lox, TokenType::RightParen, "Expect ')' after if condition.".to_string())?;

        let then_branch = Box::new(self.statement(lox)?);
        let else_branch = if self.match_token_types(vec![TokenType::Else]) {
            Some(Box::new(self.statement(lox)?))
        } else {
            None
        };

        Ok(Stmt::If(If {
            condition: condition,
            then_branch: then_branch,
            else_branch: else_branch
        }))
    }

    fn print_statement(&mut self, lox: &mut super::Lox) -> Result<Stmt, ()> {
        let value = self.expression(lox)?;
        self.consume(lox, TokenType::Semicolon, "Expect ';' after value.".to_string())?;
        Ok(Stmt::Print(value))
    }

    fn while_statement(&mut self, lox: &mut super::Lox) -> Result<Stmt, ()> {
        self.consume(lox, TokenType::LeftParen, "Expect '(' after 'while'.".to_string())?;
        let condition = self.expression(lox)?;
        self.consume(lox, TokenType::RightParen, "Expect ')' after condition.".to_string())?;
        let body = self.statement(lox)?;

        Ok(Stmt::While(While {
            condition: condition,
            body: Box::new(body)
        }))
    }

    fn block(&mut self, lox: &mut super::Lox) -> Result<Vec<Stmt>, ()> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Some(statement) = self.declaration(lox) {
                statements.push(statement);
            }
        }

        self.consume(lox, TokenType::RightBrace, "Expect '}' after block.".to_string())?;
        Ok(statements)
    }

    fn expression_statement(&mut self, lox: &mut super::Lox) -> Result<Stmt, ()> {
        let expr = self.expression(lox)?;

        if !self.is_at_end() {
            self.consume(lox, TokenType::Semicolon, "Expect ';' after expression.".to_string())?;
        }

        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self, lox: &mut super::Lox) -> Result<Expr, ()> {
        self.assignment(lox)
    }

    fn assignment(&mut self, lox: &mut super::Lox) -> Result<Expr, ()> {
        let expr = self.or(lox)?;

        if self.match_token_types(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment(lox)?;

            match expr {
                Expr::Variable(v) => Ok(Expr::Assign(Assign {
                    name: v.name,
                    value: Box::new(value)
                })),
                _ => self.error(lox, equals, "Invalid assignment target.".to_string())
            }
        } else {
            Ok(expr)
        }
    }

    fn or(&mut self, lox: &mut super::Lox) -> Result<Expr, ()> {
        let mut expr = self.and(lox)?;

        while self.match_token_types(vec![TokenType::Or]) {
            let operator = self.previous();
            let right = self.and(lox)?;
            expr = Expr::Logical(Logical {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right)
            });
        }

        Ok(expr)
    }

    fn and(&mut self, lox: &mut super::Lox) -> Result<Expr, ()> {
        let mut expr = self.equality(lox)?;

        while self.match_token_types(vec![TokenType::And]) {
            let operator = self.previous();
            let right = self.equality(lox)?;
            expr = Expr::Logical(Logical {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right)
            });
        }

        Ok(expr)
    }

    fn equality(&mut self, lox: &mut super::Lox) -> Result<Expr, ()> {
        let mut expr = self.comparison(lox)?;

        while self.match_token_types(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison(lox)?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right)
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self, lox: &mut super::Lox) -> Result<Expr, ()> {
        let mut expr = self.term(lox)?;

        while self.match_token_types(vec![TokenType::Greater,TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term(lox)?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right)
            });
        }

        Ok(expr)
    }

    fn term(&mut self, lox: &mut super::Lox) -> Result<Expr, ()> {
        let mut expr = self.factor(lox)?;

        while self.match_token_types(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor(lox)?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right)
            });
        }

        return Ok(expr);
    }

    fn factor(&mut self, lox: &mut super::Lox) -> Result<Expr, ()> {
        let mut expr = self.unary(lox)?;

        while self.match_token_types(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary(lox)?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right)
            });
        }

        return Ok(expr);
    }

    fn unary(&mut self, lox: &mut super::Lox) -> Result<Expr, ()> {
        if self.match_token_types(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary(lox)?;
            return Ok(Expr::Unary(Unary {
                operator: operator,
                right: Box::new(right)
            }));
        }

        Ok(self.call(lox)?)
    }

    fn call(&mut self, lox: &mut super::Lox) -> Result<Expr, ()> {
        let mut expr = self.primary(lox)?;

        loop {
            if self.match_token_types(vec![TokenType::LeftParen]) {
                expr = self.finish_call(lox, expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, lox: &mut super::Lox, callee: Expr) -> Result<Expr, ()> {
        let mut arguments = Vec::new();
        if !self.check(TokenType::RightParen) {
            while {
                if arguments.len() >= 8 {
                    self.error::<Expr>(lox, self.peek(), "Cannot have more than 8 arguments.".to_string()).unwrap_err();
                }
                arguments.push(self.expression(lox)?);
                self.match_token_types(vec![TokenType::Comma])
            } {};
        }

        let paren = self.consume(lox, TokenType::RightParen, "Expect ')' after arguments.".to_string())?;

        return Ok(Expr::Call(Call {
            callee: Box::new(callee),
            paren: paren,
            arguments: arguments
        }));
    }

    fn primary(&mut self, lox: &mut super::Lox) -> Result<Expr, ()> {
        if self.match_token_types(vec![TokenType::False]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }
        if self.match_token_types(vec![TokenType::True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }
        if self.match_token_types(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if self.match_token_types(vec![TokenType::Number(12.0), TokenType::String("".to_string())]) {
            return Ok(Expr::Literal(match self.previous().token_type {
                TokenType::Number(number) => Literal::Number(number),
                TokenType::String(string) => Literal::String(string),
                _ => panic!()
            }));
        }

        if self.match_token_types(vec![TokenType::Identifier]) {
            return Ok(Expr::Variable(Variable { name: self.previous() }));
        }

        if self.match_token_types(vec![TokenType::LeftParen]) {
            let expr = self.expression(lox)?;
            self.consume(lox, TokenType::RightParen, "Expect ')' after expression.".to_string())?;
            return Ok(Expr::Grouping(Grouping {
                expression: Box::new(expr)
            }));
        }

        self.error(lox, self.peek(), "Expect expression".to_string())
    }

    fn match_token_types(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        match (self.peek().token_type, token_type) {
            (TokenType::Number(_), TokenType::Number(_)) => true,
            (TokenType::String(_), TokenType::String(_)) => true,
            (peeked_type, token_type) => peeked_type == token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current+=1;
        }

        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        match self.peek().token_type {
            TokenType::Eof => true,
            _ => false
        }
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            match self.previous().token_type {
                TokenType::Semicolon => return,
                _ => match self.peek().token_type {
                    TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For | TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => return,
                    _ => self.advance()
                }
            };
        }
    }

    fn consume(&mut self, lox: &mut super::Lox, token_type: TokenType, message: String) -> Result<Token, ()> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        self.error(lox, self.peek(), message)
    }

    fn error<T>(&self, lox: &mut super::Lox, token: Token, message: String) -> Result<T, ()> {
        if token.token_type == TokenType::Eof {
            lox.report(token.line, " at end".to_string(), message);
        } else {
            lox.report(token.line, format!(" at '{}'", token.lexeme), message);
        }
        Err(())
    }
}