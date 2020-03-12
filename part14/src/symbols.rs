use crate::ast::*;

use std::collections::HashMap;

pub type VariableTable = HashMap<String, VariableValue>;

#[derive(Default)]
pub struct SymbolTable {
    pub variables: VariableTable,
}


impl SymbolTable {
    pub fn define(&mut self, var: &Root) {
        let (id, value) = match var {
            Root::VarID{name, value} => (name, value),
            _ => unreachable!()
        };
        if self.variables.contains_key(id) {
            panic!("Variable \"{}\" already defined", id);
        }
        self.variables.insert(id.to_string(), *value);
        println!("Define: {:?}", var);
    }

    pub fn lookup(&mut self, var: &Root) {
        let id = match var {
            Root::VarID{name, value} => name,
            _ => unreachable!()
        };
        if !self.variables.contains_key(id) {
            panic!("Variable \"{}\" not defined", id);
        }
        println!("Lookup: {}", id);
    }
}

#[derive(Default)]
pub struct SemanticAnalyzer {
    pub symtab: SymbolTable,
}

impl NodeVisitor for SemanticAnalyzer {
    type Result = ();
    fn visit(&mut self, node: &AST) {
        match &node.root {
            Root::Compound => self.compound(node),
            Root::Num(_) => (),
            Root::VarDecl => self.variable_decl(node),
            Root::VarID{name, value} => self.variable(node),
            Root::ProcedureDecl{name} => self.procedure_decl(node),
            Root::Param => self.param(node),
            Root::Assign => self.assign(node),
            Root::BinOp(_)   => self.binary(node),
            Root::UnaryOp(_) => self.unary(node),
            Root::NoOp => (),
        }
    }
}

impl SemanticAnalyzer {
    fn compound(&mut self, node: &AST) {
        let left = node.left.as_ref().unwrap();
        self.visit(left);
        if node.right.is_some() {
            let right = node.right.as_ref().unwrap();
            self.visit(right);
        }
    }

    fn procedure_decl(&mut self, node: &AST) {
        println!("{:?}", node.root);
        let left = node.left.as_ref().unwrap();
        self.visit(left);
    }

    fn param(&mut self, node: &AST) {
        let left = node.left.as_ref().unwrap();
        println!(" {:?}", left.root);
        let right = node.right.as_ref().unwrap();
        self.visit(right);
    }

    fn variable_decl(&mut self, node: &AST) {
        let left = node.left.as_ref().unwrap();
        self.symtab.define(&left.root);
        let right = node.right.as_ref().unwrap();
        self.visit(right);
    }

    fn assign(&mut self, node: &AST) {
        // right-hand side
        let right = node.right.as_ref().unwrap();
        self.visit(right);
        // left-hand side
        let left = node.left.as_ref().unwrap();
        self.visit(left);
    }

    fn variable(&mut self, node: &AST) {
        self.symtab.lookup(&node.root);
    }

    fn binary(&mut self, node: &AST) {
        // Unwrap and visit
        let left  = node.left.as_ref().unwrap();
        let right = node.right.as_ref().unwrap();
        self.visit(left);
        self.visit(right);
    }

    fn unary(&mut self, node: &AST) {
        assert!(node.left.is_none());
        // Unwrap and visit
        let right = node.right.as_ref().unwrap();
        self.visit(right);
    }
}
