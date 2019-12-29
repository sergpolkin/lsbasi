use crate::lexer;
use lexer::*;

/// Tree implementation
/// [Of Boxes and Trees - Smart Pointers in Rust](https://endler.dev/2017/boxes-and-trees/)
enum Root {
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
    fn new(root: Root) -> AST {
        AST { root, ..Default::default() }
    }
    fn left(mut self, leaf: AST) -> Self {
        self.left = Some(Box::new(leaf));
        self
    }
    fn right(mut self, leaf: AST) -> Self {
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


pub struct Parser {
    lexer: Lexer,
    cur_token: Option<Token>,
}

impl Parser {
    pub fn new<S: Into<String>>(text: S) -> Parser {
        Parser {
            lexer: Lexer::new(text),
            cur_token: None,
        }
    }

    fn op(&mut self) -> Option<ArithmeticOp> {
        match self.cur_token {
            Some(Token::Op(op)) => Some(op),
            Some(Token::EOF) | Some(Token::RParen) | None => None,
            _ => panic!("Expect arithmetic operator at {}", self.lexer.pos)
        }
    }

    fn factor(&mut self) -> AST {
        self.cur_token = self.lexer.get_next_token();
        let value = match self.cur_token {
            Some(Token::Integer(n)) => AST::new(Root::Num(n)),
            Some(Token::LParen)     => self.parse(),
            _ => panic!("Expect integer at {}", self.lexer.pos)
        };
        self.cur_token = self.lexer.get_next_token();
        value
    }

    fn term(&mut self) -> AST {
        let mut node = self.factor();
        loop {
            // MUL or DIV
            node = match self.op() {
                Some(op @ ArithmeticOp::Mul) |
                Some(op @ ArithmeticOp::Div) =>
                    AST::new(Root::Op(op)).left(node).right(self.factor()),
                _ => break,
            };
        }
        node
    }

    pub fn parse(&mut self) -> AST {
        let mut node = self.term();
        loop {
            // ADD or SUB
            node = match self.op() {
                Some(op @ ArithmeticOp::Plus) |
                Some(op @ ArithmeticOp::Minus) =>
                    AST::new(Root::Op(op)).left(node).right(self.term()),
                _ => break,
            };
        }
        node
    }
}
