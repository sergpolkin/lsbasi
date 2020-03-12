use crate::tokens::*;
use crate::lexer::*;
use crate::parser::*;
use crate::ast::*;
use crate::symbols::*;

pub struct Interpreter {
    parser: Parser,
    context: Context,
}

#[derive(Default)]
pub struct Context {
    pub variables: VariableTable,
}

impl Context {
    pub fn get_var<S: Into<String>>(&self, name: S) -> Option<VariableValue> {
        let key = name.into().to_ascii_uppercase();
        if self.variables.contains_key(&key) {
            Some(*self.variables.get(&key).unwrap())
        }
        else {
            None
        }
    }
}

impl Interpreter {
    pub fn new<S: Into<String>>(text: S) -> Interpreter {
        let text = text.into();
        //println!("{}", text);
        Interpreter {
            parser: Parser::new(text),
            context: Context::default(),
        }
    }

    pub fn exec(mut self) -> (Context, VariableValue) {
        let tree = self.parser.parse();
        // println!("{:#?}", tree);
        let mut semantic_analyzer = SemanticAnalyzer::default();
        semantic_analyzer.visit(&tree);
        let res = self.visit(&tree);
        (self.context, res)
    }
}

impl NodeVisitor for Interpreter {
    type Result = VariableValue;
    fn visit(&mut self, node: &AST) -> VariableValue {
        match &node.root {
            Root::Compound => self.compound(node),
            Root::Num(n) => *n,
            Root::VarDecl => VariableValue::None,
            Root::VarID{name, value} => self.variable(node),
            Root::ProcedureDecl{name} => VariableValue::None,
            Root::Param => VariableValue::None,
            Root::Assign => self.assign(node),
            Root::BinOp(op)   => self.binary(op, node),
            Root::UnaryOp(op) => self.unary(op, node),
            Root::NoOp => VariableValue::None,
        }
    }
}

impl Interpreter {
    fn compound(&mut self, node: &AST) -> VariableValue {
        let left = node.left.as_ref().unwrap();
        let mut res = self.visit(left);
        if node.right.is_some() {
            let right = node.right.as_ref().unwrap();
            res = self.visit(right)
        }
        res
    }

    fn assign(&mut self, node: &AST) -> VariableValue {
        // right-hand side
        let right = node.right.as_ref().unwrap();
        let right = self.visit(right);
        // left-hand side
        let left = node.left.as_ref().unwrap();
        let id = match &left.root {
            Root::VarID{name, value} => name,
            _ => unreachable!()
        };
        // assign
        if self.context.variables.contains_key(id) {
            let val = self.context.variables.get_mut(id).unwrap();
            val.assign(right)
        }
        else {
            self.context.variables.insert(id.to_string(), right);
            *self.context.variables.get(id).unwrap()
        }
    }

    fn variable(&mut self, node: &AST) -> VariableValue {
        let id = match &node.root {
            Root::VarID{name, value} => name,
            _ => unreachable!()
        };
        if let Some(val) = self.context.variables.get(id) {
            *val
        }
        else {
            unreachable!()
        }
    }

    fn binary(&mut self, op: &Token, node: &AST) -> VariableValue {
        // Unwrap and visit
        let left  = node.left.as_ref().unwrap();
        let right = node.right.as_ref().unwrap();
        let left  = self.visit(left);
        let right = self.visit(right);
        match op {
            Token::OpPlus  => left + right,
            Token::OpMinus => left - right,
            Token::OpMul   => left * right,
            Token::OpDiv   => left.as_real() / right.as_real(),
            Token::OpIntegerDiv => left.as_integer() / right.as_integer(),
            _ => unreachable!()
        }
    }

    fn unary(&mut self, op: &Token, node: &AST) -> VariableValue {
        assert!(node.left.is_none());
        // Unwrap and visit
        let right = node.right.as_ref().unwrap();
        let right = self.visit(right);
        match op {
            Token::OpPlus  =>  right,
            Token::OpMinus => -right,
            _ => unreachable!()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part14() {
        {
            let (ctx, res) = Interpreter::new("BEGIN END.")
                .exec();
            assert_eq!(res, VariableValue::None);
        }
    }
}
