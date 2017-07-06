use scanner::{TokenType, Token};
use expr::*;
use super::report;

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

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token_types(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right)
            });
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_token_types(vec![TokenType::Greater,TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right)
            });
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_token_types(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right)
            });
        }

        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token_types(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right)
            });
        }

        return expr;
    }

    fn unary(&mut self) -> Expr {
        if self.match_token_types(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary(Unary {
                operator: operator,
                right: Box::new(right)
            });
        }

        return self.primary();
    }
    fn primary(&mut self) -> Expr {
        if self.match_token_types(vec![TokenType::False]) {
            return Expr::Literal(Literal::Bool(false));
        }
        if self.match_token_types(vec![TokenType::True]) {
            return Expr::Literal(Literal::Bool(true));
        }
        if self.match_token_types(vec![TokenType::Nil]) {
            return Expr::Literal(Literal::Nil);
        }

        if self.match_token_types(vec![TokenType::Number(12.0), TokenType::String("".to_string())]) {
            return Expr::Literal(match self.previous().token_type {
                TokenType::Number(number) => Literal::Number(number),
                TokenType::String(string) => Literal::String(string),
                _ => panic!()
            });
        }

        if self.match_token_types(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.".to_string());
            return Expr::Grouping(Grouping {
                expression: Box::new(expr)
            });
        }

        self.error(self.peek(), "Expect expression".to_string())
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

        match self.peek().token_type {
            TokenType::Number(_) => match token_type {
                TokenType::Number(_) => true,
                _ => false
            },
            TokenType::String(_) => match token_type {
                TokenType::String(_) => true,
                _ => false
            },
            peeked_type => peeked_type == token_type
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

    fn consume(&mut self, token_type: TokenType, message: String) -> Token {
        if self.check(token_type) {
            return self.advance();
        }

        self.error(self.peek(), message)
    }

    fn error(&self, token: Token, message: String) -> ! {
        if token.token_type == TokenType::Eof {
            report(token.line, " at end".to_string(), message);
        } else {
            report(token.line, format!(" at '{}'", token.lexeme), message);
        }
        panic!();
    }
}