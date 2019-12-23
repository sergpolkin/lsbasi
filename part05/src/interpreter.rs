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

    fn op(&mut self) -> Option<ArithmeticOp> {
        match self.cur_token {
            Some(Token::Op(op)) => Some(op),
            Some(Token::EOF) | Some(Token::RParen) | None => None,
            _ => panic!("Expect arithmetic operator at {}", self.lexer.pos)
        }
    }

    fn factor(&mut self) -> i32 {
        self.cur_token = self.lexer.get_next_token();
        let value = match self.cur_token {
            Some(Token::Integer(n)) => n,
            Some(Token::LParen)     => self.exec(),
            _ => panic!("Expect integer at {}", self.lexer.pos)
        };
        self.cur_token = self.lexer.get_next_token();
        value
    }

    fn term(&mut self) -> i32 {
        let mut value = self.factor();
        loop {
            // MUL or DIV
            value = match self.op() {
                Some(ArithmeticOp::Mul) => value * self.factor(),
                Some(ArithmeticOp::Div) => value / self.factor(),
                _ => break,
            };
        }
        value
    }

    pub fn exec(&mut self) -> i32 {
        let mut value = self.term();
        loop {
            // ADD or SUB
            value = match self.op() {
                Some(ArithmeticOp::Plus)  => value + self.term(),
                Some(ArithmeticOp::Minus) => value - self.term(),
                _ => break,
            };
        }
        value
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
}
