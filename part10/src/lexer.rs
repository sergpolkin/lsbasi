use crate::tokens::*;

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

    fn peek(&self) -> Option<char> {
        let peek_pos = self.pos + 1;
        if peek_pos > self.text.len() - 1 {
            None
        }
        else {
            let text: Vec<char> = self.text.chars().collect();
            Some(text[peek_pos])
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
        let mut res = String::new();
        while let Some(c) = self.get_char() {
            if c.is_digit(10) {
                res.push(c);
                self.pos += 1;
                continue;
            }
            else {
                break;
            }
        }
        res.parse().ok()
    }

    fn parse_id(&mut self) -> String {
        let mut res = String::new();
        while let Some(c) = self.get_char() {
            if c.is_alphanumeric() {
                res.push(c);
                self.pos += 1;
                continue;
            }
            else {
                break;
            }
        }
        res
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.get_char() {

            // println!("{:02X}", c as u8);

            if c.is_whitespace() {
                self.pos += 1;
                continue;
            }

            if c.is_alphabetic() {
                let id = self.parse_id();
                return Some(Token::get_token(&id));
            }

            if c.is_digit(10) {
                let num = self.parse_integer().unwrap();
                return Some(Token::Integer(num));
            }

            if (c == ':') & (self.peek() == Some('=')) {
                self.pos += 1;
                self.pos += 1;
                return Some(Token::ASSIGN);
            }

            if c == ';' {
                self.pos += 1;
                return Some(Token::SEMI);
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

            if c == '(' {
                self.pos += 1;
                return Some(Token::LParen);
            }
            if c == ')' {
                self.pos += 1;
                return Some(Token::RParen);
            }

            if c == '.' {
                self.pos += 1;
                return Some(Token::DOT);
            }

            break;
        }
        // Unknown token
        None
    }
}
