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
    fn get_next_token(&mut self) -> Option<Token> {
        let text: Vec<char> = self.text.chars().collect();

        if self.pos > self.text.len() - 1 {
            return Some(Token::EOF);
        }

        let c = text[self.pos];
        self.pos += 1;

        if c.is_digit(10) {
            let n: i32 = c.to_digit(10).unwrap() as i32;
            return Some(Token::Integer(n));
        }

        if c == '+' {
            return Some(Token::Plus);
        }

        if let Some(_) = String::from("\r\n").find(c) {
            return Some(Token::EOF);
        }

        None
    }
    fn exec(&mut self) -> i32 {
        self.cur_token = self.get_next_token();
        let left = match self.cur_token {
            Some(Token::Integer(n)) => n,
            _ => panic!("Expect single-digit integer")
        };

        self.cur_token = self.get_next_token();
        let _op = match self.cur_token {
            Some(Token::Plus) => Token::Plus,
            _ => panic!("Expect `+` token")
        };

        self.cur_token = self.get_next_token();
        let right = match self.cur_token {
            Some(Token::Integer(n)) => n,
            _ => panic!("Expect single-digit integer")
        };

        self.cur_token = self.get_next_token();
        match self.cur_token {
            Some(Token::EOF) => {}
            _ => panic!("Expect EOF token")
        };

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
