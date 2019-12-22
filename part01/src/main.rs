use std::io::{self, Write};

#[derive(Debug)]
enum Token {
    Integer(i32),
    Plus,
    Minus,
    EOF,
}

struct Interpreter {
    text: String,
    pos: usize,
    cur_token: Option<Token>,
}
impl Interpreter {
    fn new<S: Into<String>>(text: S) -> Interpreter {
        Interpreter {
            text: text.into(),
            pos: 0,
            cur_token: None,
        }
    }

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
                return Some(Token::Plus);
            }
            if c == '-' {
                self.pos += 1;
                return Some(Token::Minus);
            }

            if String::from("\r\n").find(c).is_some() {
                self.pos += 1;
                return Some(Token::EOF);
            }

        }
    }

    fn unwrap_integer(&self) -> i32 {
        match self.cur_token {
            Some(Token::Integer(n)) => n,
            _ => panic!("Expect integer at {}", self.pos)
        }
    }

    fn unwrap_op(&self) -> Option<Token> {
        match self.cur_token {
            Some(Token::Plus)  => Some(Token::Plus),
            Some(Token::Minus) => Some(Token::Minus),
            _ => panic!("Expect `+` or `-` token at {}", self.pos)
        }
    }

    fn unwrap_eof(&self) {
        match self.cur_token {
            Some(Token::EOF) => (),
            _ => panic!("Expect EOF token at {}", self.pos)
        }
    }

    fn exec(&mut self) -> i32 {
        self.cur_token = self.get_next_token();
        let left = self.unwrap_integer();

        self.cur_token = self.get_next_token();
        let op = self.unwrap_op();

        self.cur_token = self.get_next_token();
        let right = self.unwrap_integer();

        self.cur_token = self.get_next_token();
        self.unwrap_eof();

        match op {
            Some(Token::Plus)  => left + right,
            Some(Token::Minus) => left - right,
            _ => unreachable!()
        }
    }
}

#[test]
fn interpreter_tests() {
    assert_eq!(Interpreter::new("3+5").exec(), 8);
    assert_eq!(Interpreter::new("12+3").exec(), 15);
    assert_eq!(Interpreter::new(" 12+3").exec(), 15);
    assert_eq!(Interpreter::new("7 - 5").exec(), 2);
    assert_eq!(Interpreter::new("5 - 7").exec(), -2);
}

fn main() {
    let mut text = String::new();
    loop {
        text.clear();
        print!("calc> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut text) {
            Ok(n) if n > 0 => {}
            _ => break
        }
        let mut interpreter = Interpreter::new(&text);
        let result = interpreter.exec();
        println!("{}", result);
    }
}
