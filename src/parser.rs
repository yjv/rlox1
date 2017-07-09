use scanner::{TokenType, Token};
use expr::*;

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

    pub fn parse(&mut self, lox: &mut super::Lox) -> Option<Expr> {
        self.expression(lox).ok()
    }

    fn expression(&mut self, lox: &mut super::Lox) -> Result<Expr, ()> {
        self.equality(lox)
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

        return Ok(self.primary(lox)?);
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