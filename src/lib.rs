use std::fs::File;
use std::io::prelude::*;
use std::io::{Result as IoResult, stdout, stdin};
use std::process::exit;

pub mod scanner;
pub mod expr;
pub mod parser;
pub mod interpreter;

pub struct Lox {
    pub had_error: bool
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn run_file(&mut self, path: &String) -> IoResult<()> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.run(&contents);

        if self.had_error {
            exit(65);
        }

        Ok(())
    }

    pub fn run_prompt(&mut self) {
        let mut input = String::new();
        let stdin = stdin();

        loop {
            print!("> ");
            stdout().flush().unwrap();
            input.clear();
            match stdin.read_line(&mut input) {
                Ok(_) => {
                    self.run(&input);
                }
                Err(error) => println!("error: {}", error),
            }
            self.had_error = false;
        }
    }

    pub fn run(&mut self, source: &String) {
        let mut scanner = scanner::Scanner::new(source.clone());
        let tokens = scanner.scan_tokens(self);

        let mut parser = parser::Parser::new(tokens.clone());
        let expr = parser.parse(self);

        if self.had_error {
            return;
        }

        println!("{}", expr::AstPrinter.print(&expr.unwrap()));
    }

    pub fn report(&mut self, line: i32, location: String, message: String) {
        println!("[line {} ] Error {} : {}", line, location, message);
        self.had_error = true;
    }
}
