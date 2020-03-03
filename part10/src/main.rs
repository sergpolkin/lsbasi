#![allow(unused)]
mod tokens;
mod lexer;
mod ast;
mod parser;

mod interpreter;
use interpreter::*;

fn main() {
    let text = r#"
    BEGIN

        BEGIN
            number := 2;
            a := NumBer;
            b := 10 * a + 10 * NUMBER / 4;
            c := a - - b
        end;

        x := 11;
    END.
    "#;
    let mut interpreter = Interpreter::new(text);
    let (ctx, res) = interpreter.exec();
    println!("Scope:  {:?}", ctx.variables);
    println!("Result: {:?}", res);
}
