
#[derive(Copy, Clone, PartialEq)]
enum ArithmeticOp {
    Plus,
    Minus,
    Mul,
    Div,
}

#[derive(PartialEq)]
enum Token {
    Integer(i32),
    Op(ArithmeticOp),
    EOF,
}

pub struct Interpreter {
    text: String,
    pos: usize,
    cur_token: Option<Token>,
}

impl Interpreter {
    pub fn new<S: Into<String>>(text: S) -> Interpreter {
        Interpreter {
            text: text.into(),
            pos: 0,
            cur_token: None,
        }
    }

    // Lexer
    fn parse_integer(&mut self) -> Option<i32> {
        let text: Vec<char> = self.text.chars().collect();
        let mut store: Vec<char> = Vec::new();
        loop {
            if self.pos > self.text.len() - 1 {
                let res: String = store.into_iter().collect();
                return res.parse().ok();
            }

            let c = text[self.pos];
            if c.is_digit(10) {
                store.push(c);
                self.pos += 1;
                continue;
            }
            else {
                let res: String = store.into_iter().collect();
                return res.parse().ok();
            }
        }
    }

    fn get_next_token(&mut self) -> Option<Token> {
        let text: Vec<char> = self.text.chars().collect();

        loop {
            if self.pos > self.text.len() - 1 {
                return Some(Token::EOF);
            }

            let c = text[self.pos];

            if c.is_whitespace() {
                self.pos += 1;
                continue;
            }

            if c.is_digit(10) {
                let num = self.parse_integer().unwrap();
                return Some(Token::Integer(num));
            }

            if c == '+' {
                self.pos += 1;
                return Some(Token::Op(ArithmeticOp::Plus));
            }
            if c == '-' {
                self.pos += 1;
                return Some(Token::Op(ArithmeticOp::Minus));
            }
            if c == '*' {
                self.pos += 1;
                return Some(Token::Op(ArithmeticOp::Mul));
            }
            if c == '/' {
                self.pos += 1;
                return Some(Token::Op(ArithmeticOp::Div));
            }

            if String::from("\r\n").find(c).is_some() {
                self.pos += 1;
                return Some(Token::EOF);
            }

            break;
        }
        // Unknown token
        None
    }

    // Parser
    fn unwrap_integer(&self) -> i32 {
        match self.cur_token {
            Some(Token::Integer(n)) => n,
            _ => panic!("Expect integer at {}", self.pos)
        }
    }

    fn unwrap_op(&self) -> ArithmeticOp {
        match self.cur_token {
            Some(Token::Op(op)) => op,
            _ => panic!("Expect arithmetic operator at {}", self.pos)
        }
    }

    fn term(&mut self) -> i32 {
        let value = self.unwrap_integer();
        self.cur_token = self.get_next_token();
        value
    }

    pub fn exec(&mut self) -> i32 {
        self.cur_token = self.get_next_token();
        let mut result = self.term();

        loop {
            if self.cur_token == Some(Token::EOF) {
                break;
            }
            let op = self.unwrap_op();
            self.cur_token = self.get_next_token();
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
