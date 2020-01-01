use crate::parser::*;
use crate::lexer::*;
use crate::ast::*;

pub struct Interpreter {
    parser: Parser,
}

impl Interpreter {
    pub fn new<S: Into<String>>(text: S) -> Interpreter {
        let text = text.into();
        println!("{}", text);
        Interpreter {
            parser: Parser::new(text),
        }
    }

    pub fn exec(&mut self) -> i32 {
        let tree = self.parser.parse();
        tree.visit()
    }
}

trait NodeVisitor {
    fn visit(&self) -> i32;
}

impl NodeVisitor for AST {
    fn visit(&self) -> i32 {
        match self.root {
            Root::Num(n) => n,
            Root::BinOp(op) => binary(self, op),
            Root::UnaryOp(op) => unary(self, op),
        }
    }
}

fn binary(node: &AST, op: ArithmeticOp) -> i32 {
    let left = node.left.as_ref().unwrap();
    let right = node.right.as_ref().unwrap();
    match op {
        ArithmeticOp::Plus  => left.visit() + right.visit(),
        ArithmeticOp::Minus => left.visit() - right.visit(),
        ArithmeticOp::Mul   => left.visit() * right.visit(),
        ArithmeticOp::Div   => left.visit() / right.visit(),
    }
}

fn unary(node: &AST, op: ArithmeticOp) -> i32 {
    assert!(node.left.is_none());
    let right = node.right.as_ref().unwrap();
    match op {
        ArithmeticOp::Plus  =>  right.visit(),
        ArithmeticOp::Minus => -right.visit(),
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part01() {
        assert_eq!(Interpreter::new("3+5").exec(), 8);
        assert_eq!(Interpreter::new("12+3").exec(), 15);
        assert_eq!(Interpreter::new(" 12+3").exec(), 15);
        assert_eq!(Interpreter::new("7 - 5").exec(), 2);
        assert_eq!(Interpreter::new("5 - 7").exec(), -2);
    }
    #[test]
    fn part03() {
        assert_eq!(Interpreter::new("3").exec(), 3);
        assert_eq!(Interpreter::new("7 - 3 + 2 - 1").exec(), 5);
    }
    #[test]
    fn part04() {
        assert_eq!(Interpreter::new("2 + 7 * 4").exec(), 30);
        assert_eq!(Interpreter::new("7 - 8 / 4").exec(), 5);
        assert_eq!(Interpreter::new("14 + 2 * 3 - 6 / 2").exec(), 17);
    }
    #[test]
    fn part05() {
        assert_eq!(Interpreter::new("7 + 3 * (10 / (12 / (3 + 1) - 1))").exec(), 22);
        assert_eq!(Interpreter::new("7 + 3 * (10 / (12 / (3 + 1) - 1)) / (2 + 3) - 5 - 3 + (8)").exec(), 10);
        assert_eq!(Interpreter::new("7 + (((3 + 2)))").exec(), 12);
    }
    #[test]
    fn part08() {
        assert_eq!(Interpreter::new("- 3").exec(), -3);
        assert_eq!(Interpreter::new("+ 3").exec(), 3);
        assert_eq!(Interpreter::new("5 - - - + - 3").exec(), 8);
        assert_eq!(Interpreter::new("5 - - - + - (3 + 4) - +2").exec(), 10);
    }
}
