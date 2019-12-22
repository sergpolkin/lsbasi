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

    fn unwrap_integer(&self) -> i32 {
        match self.cur_token {
            Some(Token::Integer(n)) => n,
            _ => panic!("Expect integer at {}", self.lexer.pos)
        }
    }

    fn unwrap_op(&self) -> ArithmeticOp {
        match self.cur_token {
            Some(Token::Op(op)) => op,
            _ => panic!("Expect arithmetic operator at {}", self.lexer.pos)
        }
    }

    fn term(&mut self) -> i32 {
        let value = self.unwrap_integer();
        self.cur_token = self.lexer.get_next_token();
        value
    }

    pub fn exec(&mut self) -> i32 {
        self.cur_token = self.lexer.get_next_token();
        let mut result = self.term();

        loop {
            if self.cur_token == Some(Token::EOF) || self.cur_token == None {
                break;
            }
            let op = self.unwrap_op();
            self.cur_token = self.lexer.get_next_token();
            result = match op {
                ArithmeticOp::Plus  => result + self.term(),
                ArithmeticOp::Minus => result - self.term(),
                _ => panic!("Only `+` and `-` operation available")
            };
        }
        result
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
}
