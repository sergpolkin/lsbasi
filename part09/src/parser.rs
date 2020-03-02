use crate::lexer::*;
use crate::ast::*;

pub struct Parser {
    lexer: Lexer,
    cur_token: Option<Token>,
}

impl Parser {
    pub fn new<S: Into<String>>(text: S) -> Parser {
        let mut lexer = Lexer::new(text);
        let tok = lexer.get_next_token();
        Parser {
            lexer,
            cur_token: tok,
        }
    }

    fn eat(&mut self, tok: Token) {
        // println!("{}", tok);
        match self.cur_token {
            Some(ref cur) if (*cur == tok) => {},
            Some(ref cur) => panic!("Expect {}, got {}", tok, cur),
            None => panic!("Expect {}, got None", tok)
        };
        self.cur_token = self.lexer.get_next_token();
    }

    fn variable(&mut self) -> AST {
        let name = match self.cur_token {
            Some(Token::ID(ref name)) => name.to_string(),
            _ => unreachable!()
        };
        self.eat(Token::ID(name.to_string()));
        AST::new(Root::Var(name))
    }

    /// program : compound_statement DOT
    fn program(&mut self) -> AST {
        let mut node = self.compound_statement();
        self.eat(Token::DOT);
        node
    }

    /// compound_statement: BEGIN statement_list END
    fn compound_statement(&mut self) -> AST {
        self.eat(Token::BEGIN);
        let node = self.statement_list();
        self.eat(Token::END);
        AST::new(Root::Compound).left(node)
    }

    /// statement_list : statement | statement SEMI statement_list
    fn statement_list(&mut self) -> AST {
        let node = self.statement();
        if self.cur_token == Some(Token::SEMI) {
            self.eat(Token::SEMI);
            AST::new(Root::Compound)
                .left(node)
                .right(self.statement_list())
        }
        else {
            AST::new(Root::Compound)
                .left(node)
        }
    }

    /// statement : compound_statement | assignment_statement | empty
    fn statement(&mut self) -> AST {
        match self.cur_token {
            Some(Token::BEGIN) => self.compound_statement(),
            Some(Token::ID(_)) => self.assignment_statement(),
            Some(_) => self.empty(),
            None => panic!("Error at statement: {}", self.lexer.pos)
        }
    }

    /// assignment_statement : variable ASSIGN expr
    fn assignment_statement(&mut self) -> AST {
        let l = self.variable();
        self.eat(Token::ASSIGN);
        let r = self.expr();
        AST::new(Root::Assign)
            .left(l)
            .right(r)
    }

    /// An empty production
    fn empty(&mut self) -> AST {
        AST::new(Root::NoOp)
    }

    /// expr : term ((PLUS | MINUS) term)*
    fn expr(&mut self) -> AST {
        let mut node = self.term();
        loop {
            node = match self.cur_token {
                Some(Token::Op(op @ ArithmeticOp::Plus)) |
                Some(Token::Op(op @ ArithmeticOp::Minus)) => {
                    self.eat(Token::Op(op));
                    AST::new(Root::BinOp(op))
                        .left(node)
                        .right(self.term())
                },
                _ => break
            };
        };
        node
    }

    /// term : factor ((MUL | DIV) factor)*
    fn term(&mut self) -> AST {
        let mut node = self.factor();
        loop {
            node = match self.cur_token {
                Some(Token::Op(op @ ArithmeticOp::Mul)) |
                Some(Token::Op(op @ ArithmeticOp::Div)) => {
                    self.eat(Token::Op(op));
                    AST::new(Root::BinOp(op))
                        .left(node)
                        .right(self.factor())
                },
                _ => break
            };
        };
        node
    }

    /// factor : PLUS factor
    ///        | MINUS factor
    ///        | INTEGER
    ///        | LPAREN expr RPAREN
    ///        | variable
    fn factor(&mut self) -> AST {
        match self.cur_token {
            Some(Token::Op(op @ ArithmeticOp::Plus)) |
            Some(Token::Op(op @ ArithmeticOp::Minus)) => {
                self.eat(Token::Op(op));
                AST::new(Root::UnaryOp(op)).right(self.factor())
            },
            Some(Token::Integer(n)) => {
                self.eat(Token::Integer(n));
                AST::new(Root::Num(n))
            },
            Some(Token::LParen) => {
                self.eat(Token::LParen);
                let node = self.expr();
                self.eat(Token::RParen);
                node
            },
            Some(Token::ID(_)) => self.variable(),
            _ => panic!()
        }
    }

    pub fn parse(&mut self) -> AST {
        self.program()
    }

    fn debug<S: Into<String>>(&self, lvl: S) {
        let lvl = lvl.into();
        if let Some(t) = self.cur_token.as_ref() {
            println!("{} [{}] {}", lvl, t, self.lexer.rest());
        }
        else {
            println!("{} [_] {}", lvl, self.lexer.rest());
        }
    }
}
