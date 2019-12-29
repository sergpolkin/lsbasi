use crate::lexer;
use lexer::*;

/// Tree implementation
/// [Of Boxes and Trees - Smart Pointers in Rust](https://endler.dev/2017/boxes-and-trees/)
pub enum Root {
    Num(i32),
    Op(ArithmeticOp),
}
impl Default for Root {
    fn default() -> Self {
        Root::Num(0)
    }
}

#[derive(Default)]
pub struct AST {
    root:  Root,
    left:  Option<Box<AST>>,
    right: Option<Box<AST>>,
}

impl AST {
    pub fn new(root: Root) -> AST {
        AST { root, ..Default::default() }
    }
    pub fn left(mut self, leaf: AST) -> Self {
        self.left = Some(Box::new(leaf));
        self
    }
    pub fn right(mut self, leaf: AST) -> Self {
        self.right = Some(Box::new(leaf));
        self
    }
    pub fn visit(self) -> i32 {
        match self.root {
            Root::Num(n) => n,
            Root::Op(op) => self.arithmetic(op),
        }
    }
    fn arithmetic(self, op: ArithmeticOp) -> i32 {
        let left = self.left.unwrap();
        let right = self.right.unwrap();
        match op {
            ArithmeticOp::Plus  => left.visit() + right.visit(),
            ArithmeticOp::Minus => left.visit() - right.visit(),
            ArithmeticOp::Mul   => left.visit() * right.visit(),
            ArithmeticOp::Div   => left.visit() / right.visit(),
        }
    }
}
