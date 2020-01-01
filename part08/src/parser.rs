use crate::lexer::*;
use crate::ast::*;

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
        // self.debug("factor");
        let value = match self.cur_token {
            Some(Token::Op(op @ ArithmeticOp::Plus)) |
            Some(Token::Op(op @ ArithmeticOp::Minus)) =>
                AST::new(Root::UnaryOp(op)).right(self.parse()),
            Some(Token::Integer(n)) => AST::new(Root::Num(n)),
            Some(Token::LParen)     => self.parse(),
            _ => panic!("Expect integer at {}", self.lexer.pos)
        };
        self.cur_token = self.lexer.get_next_token();
        value
    }

    fn term(&mut self) -> AST {
        let mut node = self.factor();
        // self.debug("term  ");
        loop {
            // MUL or DIV
            node = match self.op() {
                Some(op @ ArithmeticOp::Mul) |
                Some(op @ ArithmeticOp::Div) =>
                    AST::new(Root::BinOp(op)).left(node).right(self.factor()),
                _ => break,
            };
        }
        node
    }

    pub fn parse(&mut self) -> AST {
        let mut node = self.term();
        // self.debug("parse ");
        loop {
            // ADD or SUB
            node = match self.op() {
                Some(op @ ArithmeticOp::Plus) |
                Some(op @ ArithmeticOp::Minus) =>
                    AST::new(Root::BinOp(op)).left(node).right(self.term()),
                _ => break,
            };
        }
        node
    }

    fn debug<S: Into<String>>(&self, lvl: S) {
        let lvl = lvl.into();
        if let Some(t) = self.cur_token.as_ref() {
            println!("{} [{}] {}", lvl, t, self.lexer.rest());
        }
        else {
            println!("{} [_] {}", lvl, self.lexer.rest());
        }
    }
}
