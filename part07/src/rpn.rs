use crate::parser;
use parser::*;

use crate::lexer;
use crate::ast;
use lexer::*;
use ast::*;

use std::fmt;

/// translate to postfix notation, Reverse Polish Notation

struct Translator {
    parser: Parser,
}

impl Translator {
    fn new<S: Into<String>>(text: S) -> Translator {
        Translator {
            parser: Parser::new(text),
        }
    }

    fn exec(&mut self) -> String {
        let tree = self.parser.parse();
        tree.visit()
    }
}

trait NodeVisitor {
    fn visit(&self) -> String;
}

impl NodeVisitor for AST {
    fn visit(&self) -> String {
        match self.root {
            Root::Num(n) => n.to_string(),
            Root::Op(op) => {
                let left = self.left.as_ref().unwrap();
                let right = self.right.as_ref().unwrap();
                format!("{} {} {}", left.visit(), right.visit(), op)
            }
        }
    }
}

impl fmt::Display for ArithmeticOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArithmeticOp::Plus  => write!(f, "+"),
            ArithmeticOp::Minus => write!(f, "-"),
            ArithmeticOp::Mul   => write!(f, "*"),
            ArithmeticOp::Div   => write!(f, "/"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rpn() {
        assert_eq!(Translator::new("2 + 3").exec(), "2 3 +");
        assert_eq!(Translator::new("2 + 3 * 5").exec(), "2 3 5 * +");
        assert_eq!(Translator::new("5 + ((1 + 2) * 4) - 3").exec(), "5 1 2 + 4 * + 3 -");
        assert_eq!(Translator::new("(5 + 3) * 12 / 3").exec(), "5 3 + 12 * 3 /");
    }
}
