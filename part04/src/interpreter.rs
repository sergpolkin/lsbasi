use crate::lexer;
use lexer::*;

pub struct Interpreter {
    lexer: Lexer,
    cur_token: Option<Token>,
}

impl Interpreter {
    pub fn new<S: Into<String>>(text: S) -> Interpreter {
        Interpreter {
            lexer: Lexer::new(text),
            cur_token: None,
        }
    }

    fn integer(&mut self) -> i32 {
        self.cur_token = self.lexer.get_next_token();
        match self.cur_token {
            Some(Token::Integer(n)) => n,
            _ => panic!("Expect integer at {}", self.lexer.pos)
        }
    }

    fn op(&mut self) -> Option<ArithmeticOp> {
        self.cur_token = self.lexer.get_next_token();
        match self.cur_token {
            Some(Token::Op(op)) => Some(op),
            Some(Token::EOF) | None => None,
            _ => panic!("Expect arithmetic operator at {}", self.lexer.pos)
        }
    }

    fn term(&mut self) -> i32 {
        let mut value = self.integer();
        loop {
            // MUL or DIV
            value = match self.op() {
                Some(ArithmeticOp::Mul) => value * self.integer(),
                Some(ArithmeticOp::Div) => value / self.integer(),
                _ => break,
            };
        }
        value
    }

    pub fn exec(&mut self) -> i32 {
        let mut value = self.term();
        loop {
            // ADD or SUB
            value = match self.cur_token {
                Some(Token::Op(ArithmeticOp::Plus))  => value + self.term(),
                Some(Token::Op(ArithmeticOp::Minus)) => value - self.term(),
                _ => break,
            };
        }
        value
    }
}

#[test]
fn interpreter_tests() {
    // from chapter 01
    assert_eq!(Interpreter::new("3+5").exec(), 8);
    assert_eq!(Interpreter::new("12+3").exec(), 15);
    assert_eq!(Interpreter::new(" 12+3").exec(), 15);
    assert_eq!(Interpreter::new("7 - 5").exec(), 2);
    assert_eq!(Interpreter::new("5 - 7").exec(), -2);
    // from chapter 03
    assert_eq!(Interpreter::new("3").exec(), 3);
    assert_eq!(Interpreter::new("7 - 3 + 2 - 1").exec(), 5);
    // from chapter 04
    assert_eq!(Interpreter::new("2 + 7 * 4").exec(), 30);
    assert_eq!(Interpreter::new("7 - 8 / 4").exec(), 5);
    assert_eq!(Interpreter::new("14 + 2 * 3 - 6 / 2").exec(), 17);
}
