use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum ArithmeticOp {
    Plus,
    Minus,
    Mul,
    Div,
}

impl fmt::Display for ArithmeticOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArithmeticOp::Plus  => write!(f, "+"),
            ArithmeticOp::Minus => write!(f, "-"),
            ArithmeticOp::Mul   => write!(f, "*"),
            ArithmeticOp::Div   => write!(f, "/"),
        }
    }
}


#[derive(PartialEq)]
pub enum Token {
    Integer(i32),
    Op(ArithmeticOp),
    LParen,
    RParen,
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Integer(n) => write!(f, "{}", n),
            Token::Op(op) => write!(f, "{}", op),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::EOF => write!(f, "EOF"),
        }
    }
}

pub struct Lexer {
    text: String,
    pub pos: usize,
}

impl Lexer {
    pub fn new<S: Into<String>>(text: S) -> Lexer {
        Lexer {
            text: text.into(),
            pos: 0,
        }
    }

    fn get_char(&self) -> Option<char> {
        if self.pos > self.text.len() - 1 {
            None
        }
        else {
            let text: Vec<char> = self.text.chars().collect();
            Some(text[self.pos])
        }
    }

    pub fn rest(&self) -> String {
        if self.pos > self.text.len() - 1 {
            "Empty".to_string()
        }
        else {
            self.text.as_str()[self.pos..].to_string()
        }
    }

    fn parse_integer(&mut self) -> Option<i32> {
        let mut store: Vec<char> = Vec::new();
        while let Some(c) = self.get_char() {
            if c.is_digit(10) {
                store.push(c);
                self.pos += 1;
                continue;
            }
            else {
                break;
            }
        }
        let res: String = store.into_iter().collect();
        res.parse().ok()
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.get_char() {

            // println!("{:02X}", c as u8);

            if String::from("\r\n").find(c).is_some() {
                self.pos += 1;
                return Some(Token::EOF);
            }

            if c.is_whitespace() {
                self.pos += 1;
                continue;
            }

            if c == '(' {
                self.pos += 1;
                return Some(Token::LParen);
            }
            if c == ')' {
                self.pos += 1;
                return Some(Token::RParen);
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

            break;
        }
        // Unknown token
        None
    }
}
