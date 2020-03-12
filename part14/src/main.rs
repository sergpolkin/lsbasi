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
    program Part14;
    var
        x, y : integer;
        a, b : real;
    var c : integer;
        d : integer;
    procedure foo1; begin end;
    procedure foo2(ii:integer); begin ii:=42 end;
    procedure foo3(iii, jjj:integer; kkk:real);
        var x : real;
        begin
            {y := x + iii;}
        end;
    begin
        x := y;
    end.
    "#;
    let mut interpreter = Interpreter::new(text);
    let (ctx, res) = interpreter.exec();
    println!("Scope:  {:?}", ctx.variables);
    println!("Result: {:?}", res);
}
