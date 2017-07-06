use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Result as IoResult, stdout, stdin};
use std::process::exit;
extern crate lox1;

use lox1::*;

fn main() {
    let mut lox = Lox::new();
    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
    } else if args.len() == 2 {
        lox.run_file(&args[0]).unwrap();
    } else {
        lox.run_prompt();
    }
}
