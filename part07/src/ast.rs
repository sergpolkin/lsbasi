use crate::lexer::*;

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
    pub root:  Root,
    pub left:  Option<Box<AST>>,
    pub right: Option<Box<AST>>,
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
}
