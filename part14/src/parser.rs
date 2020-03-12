use crate::tokens::*;
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
        self.cur_token = match &self.cur_token {
            Some(ref cur) if (cur == &tok) => self.lexer.get_next_token(),
            Some(ref cur) => panic!("Expect {}, got {}", tok, cur),
            None => panic!("Expect {}, got None", tok)
        };
    }

    fn eat_type(&mut self) -> Keyword {
        let kw = match &self.cur_token {
            Some(Token::KW(kw))
                if *kw == Keyword::INTEREG || *kw == Keyword::REAL => *kw,
            Some(tok) => panic!("Expect 'type', got {}", tok),
            None => panic!("Expect 'type', got None")
        };
        self.cur_token = self.lexer.get_next_token();
        kw
    }

    fn eat_any(&mut self) {
        // println!("${}", self.cur_token.as_ref().unwrap());
        if self.cur_token.is_some() {
            self.cur_token = self.lexer.get_next_token();
        }
        else {
            panic!("Expect 'any' token, got None");
        }
    }

    fn variable(&mut self) -> AST {
        let name = match self.cur_token {
            Some(Token::ID(ref name)) => name.to_string(),
            _ => unreachable!()
        };
        self.eat(Token::ID(name.to_string()));
        let var = Root::VarID { name, value: VariableValue::None };
        AST::new(var)
    }

    /// program : PROGRAM variable SEMI block DOT
    fn program(&mut self) -> AST {
        if self.cur_token == Some(Token::KW(Keyword::PROGRAM)) {
            self.eat(Token::KW(Keyword::PROGRAM));
            let _prog_name = self.variable();
            self.eat(Token::SEMI);
        }
        let block_node = self.block();
        self.eat(Token::DOT);
        block_node
    }

    /// block : declarations compound_statement
    fn block(&mut self) -> AST {
        let decl_node = self.declarations();
        let comp_node = self.compound_statement();
        AST::new(Root::Compound)
            .left(decl_node)
            .right(comp_node)
    }

    /// declarations : variable_declarations
    ///              | procedure_declarations
    ///              | empty
    fn declarations(&mut self) -> AST {
        let var  = self.variable_declarations(AST::default());
        let proc = self.procedure_declarations(AST::default());
        AST::new(Root::Compound)
            .left(var)
            .right(proc)
    }

    /// variable_declarations : VAR ID (COMMA ID)* COLON type_spec
    fn variable_declarations(&mut self, comp: AST) -> AST {
        if self.cur_token == Some(Token::KW(Keyword::VAR)) {
            self.eat(Token::KW(Keyword::VAR));
            let decl = self.variable_declaration_helper(AST::default());
            let new_comp = AST::new(Root::Compound)
                .left(decl)
                .right(comp);
            self.variable_declarations(new_comp)
        }
        else {
            comp
        }
    }

    fn variable_declaration_helper(&mut self, decl: AST) -> AST {
        // eat variable ID, wrap in VarDecl
        let mut node = AST::new(Root::VarDecl)
            .left(self.variable())
            .right(decl);
        match self.cur_token {
            Some(Token::COMMA) => {
                self.eat(Token::COMMA);
                self.variable_declaration_helper(node)
            },
            Some(Token::COLON) => {
                self.eat(Token::COLON);
                let typ = self.eat_type();
                set_type(&mut node, typ);
                self.eat(Token::SEMI);
                if let Some(Token::ID(ref _id)) = self.cur_token {
                    let new_comp = AST::new(Root::Compound).left(node);
                    self.variable_declaration_helper(new_comp)
                }
                else {
                    node
                }
            },
            _ => panic!("Error at variable_declaration: {}", self.lexer.pos)
        }
    }

    /// procedure_declarations : (PROCEDURE ID SEMI block SEMI)*
    fn procedure_declarations(&mut self, comp: AST) -> AST {
        if self.cur_token == Some(Token::KW(Keyword::PROCEDURE)) {
            self.eat(Token::KW(Keyword::PROCEDURE));
            let proc_name = get_name(&self.variable());
            let params = self.formal_parameter_list();
            self.eat(Token::SEMI);
            let block = self.block();
            self.eat(Token::SEMI);
            let proc_decl = AST::new(Root::ProcedureDecl{name: proc_name})
                .left(params)
                .right(block);
            let new_comp = AST::new(Root::Compound)
                .left(proc_decl)
                .right(comp);
            self.procedure_declarations(new_comp)
        }
        else {
            comp
        }
    }

    fn formal_parameter_list(&mut self) -> AST {
        if self.cur_token == Some(Token::LParen) {
            self.eat(Token::LParen);
            let params = self.formal_parameters(AST::default());
            self.eat(Token::RParen);
            params
        }
        else {
            AST::default()
        }
    }

    fn formal_parameters(&mut self, decl: AST) -> AST {
        // eat variable ID, wrap in Param
        let mut node = AST::new(Root::Param)
            .left(self.variable())
            .right(decl);
        match self.cur_token {
            Some(Token::COMMA) => {
                self.eat(Token::COMMA);
                self.formal_parameters(node)
            },
            Some(Token::COLON) => {
                self.eat(Token::COLON);
                let typ = self.eat_type();
                set_type(&mut node, typ);
                if self.cur_token == Some(Token::SEMI) {
                    self.eat(Token::SEMI);
                    let new_comp = AST::new(Root::Compound).left(node);
                    self.formal_parameters(new_comp)
                }
                else {
                    node
                }
            },
            _ => panic!("Error at formal_parameters: {}", self.lexer.pos)
        }
    }

    /// compound_statement : BEGIN statement_list END
    fn compound_statement(&mut self) -> AST {
        self.eat(Token::KW(Keyword::BEGIN));
        let node = self.statement_list();
        self.eat(Token::KW(Keyword::END));
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
            Some(Token::KW(Keyword::BEGIN)) => self.compound_statement(),
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
            node = match self.cur_token.clone() {
                Some(op @ Token::OpPlus) |
                Some(op @ Token::OpMinus) => {
                    self.eat(op.clone());
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
            node = match self.cur_token.clone() {
                Some(op @ Token::OpMul) |
                Some(op @ Token::OpDiv) => {
                    self.eat(op.clone());
                    AST::new(Root::BinOp(op))
                        .left(node)
                        .right(self.factor())
                },
                Some(kw @ Token::KW(Keyword::DIV)) => {
                    self.eat(kw);
                    AST::new(Root::BinOp(Token::OpIntegerDiv))
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
        match self.cur_token.clone() {
            Some(op @ Token::OpPlus) |
            Some(op @ Token::OpMinus) => {
                self.eat(op.clone());
                AST::new(Root::UnaryOp(op))
                    .right(self.factor())
            },
            Some(Token::Integer(n)) => {
                self.eat(Token::Integer(n));
                let val = VariableValue::Intereg(n);
                AST::new(Root::Num(val))
            },
            Some(Token::Real(n)) => {
                self.eat(Token::Real(n));
                let val = VariableValue::Real(n);
                AST::new(Root::Num(val))
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

// Some helper functions

/// Set VarID type in VarDecl chain
fn set_type(decl: &mut AST, typ: Keyword)
{
    match decl.root {
        Root::VarDecl | Root::Param => {
            let id = decl.left.as_deref_mut().unwrap();
            if let Root::VarID{ref name, ref mut value} = id.root {
                assert_eq!(*value, VariableValue::None);
                *value = VariableValue::from(typ);
            }
            else {
                unreachable!();
            }
            let decl = decl.right.as_deref_mut().unwrap();
            set_type(decl, typ);
        },
        Root::Compound => {},
        Root::NoOp => {},
        _ => unreachable!()
    }
}

fn get_name(decl: &AST) -> String {
    match &decl.root {
        Root::VarID{name,value} => { name.to_string() },
        _ => unreachable!()
    }
}
