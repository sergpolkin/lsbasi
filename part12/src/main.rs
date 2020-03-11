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
    PROGRAM Part12;
    VAR
        a : INTEGER;

    PROCEDURE P1;
    VAR
        a : REAL;
        k : INTEGER;

        PROCEDURE P2;
        VAR
            a, z : INTEGER;
        BEGIN {P2}
            z := 777;
        END;  {P2}

    BEGIN {P1}

    END;  {P1}

    BEGIN {Part12}
    a := 10;
    END.  {Part12}
    "#;
    let mut interpreter = Interpreter::new(text);
    let (ctx, res) = interpreter.exec();
    println!("Scope:  {:?}", ctx.variables);
    println!("Result: {:?}", res);
}
