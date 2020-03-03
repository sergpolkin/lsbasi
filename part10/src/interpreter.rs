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
    pub variables: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new<S: Into<String>>(text: S) -> Interpreter {
        let text = text.into();
        //println!("{}", text);
        Interpreter {
            parser: Parser::new(text),
        }
    }

    pub fn exec(mut self) -> (Context, Option<i32>) {
        let tree = self.parser.parse();
        //println!("{:#?}", tree);
        let mut ctx = Context::default();
        let res = tree.visit(&mut ctx);
        (ctx, res)
    }
}

trait NodeVisitor {
    fn visit(&self, ctx: &mut Context) -> Option<i32>;
}

impl NodeVisitor for AST {
    fn visit(&self, ctx: &mut Context) -> Option<i32> {
        match self.root {
            Root::Compound => compound(self, ctx),
            Root::Assign   => assign(self, ctx),
            Root::Var(ref id) => variable(id, self, ctx),
            Root::BinOp(op)   => binary(op, self, ctx),
            Root::UnaryOp(op) => unary(op, self, ctx),
            Root::Num(n) => Some(n),
            Root::NoOp => None,
        }
    }
}

fn compound(node: &AST, ctx: &mut Context) -> Option<i32> {
    let left = node.left.as_ref().unwrap();
    let mut res = left.visit(ctx);
    if node.right.is_some() {
        let right = node.right.as_ref().unwrap();
        res = right.visit(ctx)
    }
    res
}

fn assign(node: &AST, ctx: &mut Context) -> Option<i32> {
    let left = node.left.as_ref().unwrap();
    let id = match left.root {
        Root::Var(ref id) => id.to_string(),
        _ => unreachable!()
    };
    let right = node.right.as_ref().unwrap();
    let right = right.visit(ctx).unwrap();
    let val = ctx.variables.entry(id).or_insert(0);
    *val = right;
    Some(right)
}

fn variable<S>(id: S, node: &AST, ctx: &mut Context) -> Option<i32>
where S: Into<String>
{
    let val = ctx.variables.get(&id.into()).unwrap();
    Some(*val)
}

fn binary(op: ArithmeticOp, node: &AST, ctx: &mut Context) -> Option<i32> {
    // Unwrap and visit
    let left = node.left.as_ref().unwrap();
    let right = node.right.as_ref().unwrap();
    let left = left.visit(ctx).unwrap();
    let right = right.visit(ctx).unwrap();
    let res = match op {
        ArithmeticOp::Plus  => left + right,
        ArithmeticOp::Minus => left - right,
        ArithmeticOp::Mul   => left * right,
        ArithmeticOp::Div   => left / right,
    };
    Some(res)
}

fn unary(op: ArithmeticOp, node: &AST, ctx: &mut Context) -> Option<i32> {
    assert!(node.left.is_none());
    // Unwrap and visit
    let right = node.right.as_ref().unwrap();
    let right = right.visit(ctx).unwrap();
    let res = match op {
        ArithmeticOp::Plus  =>  right,
        ArithmeticOp::Minus => -right,
        _ => unreachable!()
    };
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part09() {
        let i = Interpreter::new("BEGIN END.");
        assert_eq!(i.exec().1, None);
        let i = Interpreter::new("BEGIN a := 5; x := 11 END.");
        assert_eq!(i.exec().1, Some(11));
        let i = Interpreter::new("BEGIN a := 5; x := 11; END.");
        assert_eq!(i.exec().1, None);
        let i = Interpreter::new("BEGIN BEGIN a := 5 END; x := 11 END.");
        assert_eq!(i.exec().1, Some(11));
    }
}
