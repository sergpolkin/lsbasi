use crate::tokens::*;
use crate::lexer::*;

/// Tree implementation
/// [Of Boxes and Trees - Smart Pointers in Rust](https://endler.dev/2017/boxes-and-trees/)
#[derive(Debug)]
pub enum Root {
    Compound,
    Num(f64),
    VarDecl,
    VarID {
        name: String,
        value: VariableValue,
    },
    Assign,
    BinOp(Token),
    UnaryOp(Token),
    NoOp,
}
impl Default for Root {
    fn default() -> Self {
        Root::NoOp
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableValue {
    Intereg(i32),
    Real(f64),
    None,
}

#[derive(Debug, Default)]
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

use std::ops::{Neg, Add, Sub, Mul, Div};

impl Neg for VariableValue
{
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            Self::Real(n) => Self::Real(-n),
            Self::Intereg(n) => Self::Intereg(-n),
            _ => unimplemented!()
        }
    }
}

impl Add for VariableValue {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Intereg(a), Self::Intereg(b)) => Self::Intereg(a+b),
            (Self::Real(a), Self::Intereg(b)) => Self::Real(a+(b as f64)),
            (Self::Intereg(a), Self::Real(b)) => Self::Real((a as f64)+b),
            (Self::Real(a), Self::Real(b)) => Self::Real(a+b),
            _ => unimplemented!()
        }
    }
}

impl Sub for VariableValue {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Intereg(a), Self::Intereg(b)) => Self::Intereg(a-b),
            (Self::Real(a), Self::Intereg(b)) => Self::Real(a-(b as f64)),
            (Self::Intereg(a), Self::Real(b)) => Self::Real((a as f64)-b),
            (Self::Real(a), Self::Real(b)) => Self::Real(a-b),
            _ => unimplemented!()
        }
    }
}

impl Mul for VariableValue {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Intereg(a), Self::Intereg(b)) => Self::Intereg(a*b),
            (Self::Real(a), Self::Intereg(b)) => Self::Real(a*(b as f64)),
            (Self::Intereg(a), Self::Real(b)) => Self::Real((a as f64)*b),
            (Self::Real(a), Self::Real(b)) => Self::Real(a*b),
            _ => unimplemented!()
        }
    }
}

impl Div for VariableValue {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Intereg(a), Self::Intereg(b)) => Self::Intereg(a/b),
            (Self::Real(a), Self::Intereg(b)) => Self::Real(a/(b as f64)),
            (Self::Intereg(a), Self::Real(b)) => Self::Real((a as f64)/b),
            (Self::Real(a), Self::Real(b)) => Self::Real(a/b),
            _ => unimplemented!()
        }
    }
}
