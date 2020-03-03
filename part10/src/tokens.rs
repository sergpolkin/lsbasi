use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Token {
    Integer(i32),
    Op(ArithmeticOp),
    LParen,
    RParen,
    ID(String),
    ASSIGN,
    SEMI,
    KW(Keyword),
    DOT,
    EOF,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ArithmeticOp {
    Plus,
    Minus,
    Mul,
    Div,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Keyword {
    BEGIN,
    END,
    RESERVED,
}

const RESERVED_KEYWORDS: &[(&str, Keyword)] = &[
    ("PROGRAM", Keyword::RESERVED),
    ("VAR",     Keyword::RESERVED),
    ("DIV",     Keyword::RESERVED),
    ("INTEGER", Keyword::RESERVED),
    ("REAL",    Keyword::RESERVED),
    ("BEGIN",   Keyword::BEGIN),
    ("END",     Keyword::END),
];

impl Token {
    pub fn get_token(id: &str) -> Token {
        let id = String::from(id).to_ascii_uppercase();
        for keyword in RESERVED_KEYWORDS {
            if id == keyword.0 {
                if keyword.1 == Keyword::RESERVED {
                    panic!("Reserved keyword: {}", keyword.0);
                }
                return Token::KW(keyword.1);
            }
        }
        Token::ID(id)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Integer(n) => write!(f, "{}", n),
            Token::Op(op) => write!(f, "{}", op),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::ID(id) => write!(f, "ID \"{}\"", id),
            Token::ASSIGN => write!(f, "ASSIGN"),
            Token::SEMI => write!(f, "SEMI"),
            Token::KW(k) => write!(f, "{:?}", k),
            Token::DOT => write!(f, "DOT"),
            Token::EOF => write!(f, "EOF"),
        }
    }
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
