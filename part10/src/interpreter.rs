use crate::tokens::*;
use crate::lexer::*;
use crate::parser::*;
use crate::ast::*;

use std::collections::HashMap;

pub struct Interpreter {
    parser: Parser,
}

#[derive(Default)]
pub struct Context {
    pub variables: HashMap<String, VariableValue>,
}

impl Context {
    pub fn get_var<S: Into<String>>(&self, name: S) -> Option<VariableValue> {
        let key = name.into().to_ascii_uppercase();
        if self.variables.contains_key(&key) {
            Some(*self.variables.get(&key).unwrap())
        }
        else {
            None
        }
    }
}

impl Interpreter {
    pub fn new<S: Into<String>>(text: S) -> Interpreter {
        let text = text.into();
        //println!("{}", text);
        Interpreter {
            parser: Parser::new(text),
        }
    }

    pub fn exec(mut self) -> (Context, VariableValue) {
        let tree = self.parser.parse();
        // println!("{:#?}", tree);
        let mut ctx = Context::default();
        let res = tree.visit(&mut ctx);
        (ctx, res)
    }
}

trait NodeVisitor {
    fn visit(&self, ctx: &mut Context) -> VariableValue;
}

impl NodeVisitor for AST {
    fn visit(&self, ctx: &mut Context) -> VariableValue {
        match &self.root {
            Root::Compound => compound(self, ctx),
            Root::Num(n) => *n,
            Root::VarDecl => variable_decl(self, ctx),
            Root::VarID{name, value} => variable(self, ctx),
            Root::Assign => assign(self, ctx),
            Root::BinOp(op)   => binary(op, self, ctx),
            Root::UnaryOp(op) => unary(op, self, ctx),
            Root::NoOp => VariableValue::None,
        }
    }
}

fn compound(node: &AST, ctx: &mut Context) -> VariableValue {
    let left = node.left.as_ref().unwrap();
    let mut res = left.visit(ctx);
    if node.right.is_some() {
        let right = node.right.as_ref().unwrap();
        res = right.visit(ctx)
    }
    res
}

fn variable_decl(node: &AST, ctx: &mut Context) -> VariableValue {
    let left = node.left.as_ref().unwrap();
    match &left.root {
        Root::VarID{name, value} => {
            ctx.variables.insert(name.to_string(), *value);
        },
        _ => unreachable!()
    };
    let right = node.right.as_ref().unwrap();
    right.visit(ctx);
    VariableValue::None
}

fn assign(node: &AST, ctx: &mut Context) -> VariableValue {
    let left = node.left.as_ref().unwrap();
    let id = match &left.root {
        Root::VarID{name, value} => name,
        _ => unreachable!()
    };
    let right = node.right.as_ref().unwrap();
    let right = right.visit(ctx);
    if ctx.variables.contains_key(id) {
        let val = ctx.variables.get_mut(id).unwrap();
        val.assign(right)
    }
    else {
        ctx.variables.insert(id.to_string(), right);
        *ctx.variables.get(id).unwrap()
    }
}

fn variable(node: &AST, ctx: &mut Context) -> VariableValue {
    let id = match &node.root {
        Root::VarID{name, value} => name,
        _ => unreachable!()
    };
    if let Some(val) = ctx.variables.get(id) {
        *val
    }
    else {
        unreachable!()
    }
}

fn binary(op: &Token, node: &AST, ctx: &mut Context) -> VariableValue {
    // Unwrap and visit
    let left  = node.left.as_ref().unwrap();
    let right = node.right.as_ref().unwrap();
    let left  = left.visit(ctx);
    let right = right.visit(ctx);
    match op {
        Token::OpPlus  => left + right,
        Token::OpMinus => left - right,
        Token::OpMul   => left * right,
        Token::OpDiv   => left / right,
        _ => unreachable!()
    }
}

fn unary(op: &Token, node: &AST, ctx: &mut Context) -> VariableValue {
    assert!(node.left.is_none());
    // Unwrap and visit
    let right = node.right.as_ref().unwrap();
    let right = right.visit(ctx);
    match op {
        Token::OpPlus  =>  right,
        Token::OpMinus => -right,
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part10() {
        {
            let (ctx, res) = Interpreter::new("BEGIN END.")
                .exec();
            assert_eq!(res, VariableValue::None);
        }
        {
            let (ctx, res) = Interpreter::new("BEGIN a := 5; x := 11. END.")
                .exec();
            assert_eq!(ctx.get_var("a"), Some(VariableValue::Intereg(5)));
            assert_eq!(ctx.get_var("X"), Some(VariableValue::Real(11.0)));
            assert_eq!(ctx.get_var("z"), None);
            assert_eq!(res, VariableValue::Real(11.0));
        }
        {
            let (ctx, res) = Interpreter::new("BEGIN a := 5; x := 11; END.")
                .exec();
            assert_eq!(res, VariableValue::None);
        }
        {
            let (ctx, res) = Interpreter::new("BEGIN BEGIN a := 5 END; x := 11 END.")
                .exec();
            assert_eq!(res, VariableValue::Intereg(11));
        }
    }
}
