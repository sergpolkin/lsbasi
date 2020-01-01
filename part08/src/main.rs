#![allow(unused)]
mod lexer;
mod ast;
mod parser;

mod interpreter;
use interpreter::*;

use std::io::{self, Write};

fn main() {
    let mut text = String::new();
    loop {
        text.clear();
        print!("calc> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut text) {
            Ok(n) if n > 0 => {}
            _ => break
        }
        let mut interpreter = Interpreter::new(&text);
        let result = interpreter.exec();
        println!("{}", result);
    }
}
