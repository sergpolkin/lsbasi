use crate::lexer;
use lexer::*;

use crate::ast;
use ast::*;

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
