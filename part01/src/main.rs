use std::io::{self, Write};

#[derive(Debug)]
enum Token {
    Integer(i32),
    Plus,
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

        if self.pos > self.text.len() - 1 {
            return Some(Token::EOF);
        }

        let c = text[self.pos];

        if c.is_digit(10) {
            let num = self.parse_integer().unwrap();
            return Some(Token::Integer(num));
        }

        if c == '+' {
            self.pos += 1;
            return Some(Token::Plus);
        }

        if String::from("\r\n").find(c).is_some() {
            self.pos += 1;
            return Some(Token::EOF);
        }

        None
    }

    fn unwrap_integer(&self) -> i32 {
        match self.cur_token {
            Some(Token::Integer(n)) => n,
            _ => panic!("Expect integer at {}", self.pos)
        }
    }

    fn unwrap_plus(&self) {
        match self.cur_token {
            Some(Token::Plus) => (),
            _ => panic!("Expect `+` token at {}", self.pos)
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
        let _op = self.unwrap_plus();

        self.cur_token = self.get_next_token();
        let right = self.unwrap_integer();

        self.cur_token = self.get_next_token();
        self.unwrap_eof();

        left + right
    }
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
