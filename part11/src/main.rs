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
    PROGRAM Part11;
    VAR
        number : INTEGER;
        a, b   : INTEGER;
        y      : REAL;

    BEGIN {Part11}
        number := 2;
        a := number ;
        b := 10 * a + 10 * number DIV 4;
        y := 20 / 7 + 3.14
    END.  {Part11}
    "#;
    let mut interpreter = Interpreter::new(text);
    let (ctx, res) = interpreter.exec();
    println!("Scope:  {:?}", ctx.variables);
    println!("Result: {:?}", res);
}
