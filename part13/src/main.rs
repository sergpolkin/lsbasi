#![allow(unused)]
mod tokens;
mod lexer;
mod ast;
mod parser;

mod symbols;
mod interpreter;
use interpreter::*;

fn main() {
    let text = r#"
    program Part13;
    var
        x, y : integer;
        a, b : real;
    var c : integer;
        d : integer;
    begin
        x := y;
    end.
    "#;
    let mut interpreter = Interpreter::new(text);
    let (ctx, res) = interpreter.exec();
    println!("Scope:  {:?}", ctx.variables);
    println!("Result: {:?}", res);
}
