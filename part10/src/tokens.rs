use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Numbers
    Integer(i32),
    Real(f64),
    // Operators
    OpPlus,   // '+'
    OpMinus,  // '-'
    OpMul,    // '*'
    OpDiv,    // '/'
    // Lexems
    ID(String),
    KW(Keyword),
    // Delim
    LParen,
    RParen,
    ASSIGN,   // ':='
    SEMI,     // ';'
    COLON,    // ':'
    COMMA,    // ','
    DOT,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    PROGRAM,
    VAR,
    INTEREG,
    REAL,
    BEGIN,
    END,
    RESERVED,
}

const RESERVED_KEYWORDS: &[(&str, Keyword)] = &[
    ("PROGRAM", Keyword::PROGRAM),
    ("VAR",     Keyword::VAR),
    ("DIV",     Keyword::RESERVED),
    ("INTEGER", Keyword::INTEREG),
    ("REAL",    Keyword::REAL),
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
                return Token::KW(keyword.1.clone());
            }
        }
        Token::ID(id)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Numbers
            Token::Integer(n) => write!(f, "{}", n),
            Token::Real(n) => write!(f, "{}", n),
            // Operators
            Token::OpPlus => write!(f, "+"),
            Token::OpMinus => write!(f, "-"),
            Token::OpMul  => write!(f, "*"),
            Token::OpDiv  => write!(f, "/"),
            // Lexems
            Token::ID(id) => write!(f, "ID \"{}\"", id),
            Token::KW(k)  => write!(f, "{:?}", k),
            // Delim
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::ASSIGN => write!(f, "ASSIGN"),
            Token::SEMI   => write!(f, "SEMI"),
            Token::COLON  => write!(f, "COLON"),
            Token::COMMA  => write!(f, "COMMA"),
            Token::DOT => write!(f, "DOT"),
            Token::EOF => write!(f, "EOF"),
        }
    }
}

