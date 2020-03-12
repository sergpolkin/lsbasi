use crate::ast::*;

use std::collections::HashMap;

pub type VariableTable = HashMap<String, VariableValue>;

#[derive(Debug)]
pub struct SymbolTable {
    pub scope_name: String,
    pub scope_level: u32,
    pub enclosing_scope: Option<Box<SymbolTable>>,
    pub variables: VariableTable,
}

impl SymbolTable {
    pub fn new(name: impl Into<String>, lvl: u32) -> Self {
        SymbolTable {
            scope_name: name.into(),
            scope_level: lvl,
            enclosing_scope: None,
            variables: VariableTable::default(),
        }
    }

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
    pub scope: Option<Box<SymbolTable>>,
}

impl SemanticAnalyzer {
    pub fn current_scope(&mut self) -> &mut SymbolTable {
        self.scope.as_mut().unwrap()
    }
    pub fn push_scope(&mut self, scope: SymbolTable) {
        self.scope = Some(Box::new(SymbolTable{
            enclosing_scope: self.scope.take(),
            ..scope
        }));
    }
    pub fn pop_scope(&mut self) -> SymbolTable {
        match self.scope.take() {
            Some(mut scope) => {
                self.scope = scope.enclosing_scope.take();
                *scope
            },
            _ => unreachable!()
        }
    }
}

impl NodeVisitor for SemanticAnalyzer {
    type Result = ();
    fn visit(&mut self, node: &AST) {
        match &node.root {
            Root::Program{name} => self.program(node),
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
    fn program(&mut self, node: &AST) {
        let global_scope = SymbolTable::new("global", 1);
        self.push_scope(global_scope);
        println!("ENTER scope: global");
        // visit subtree
        let left = node.left.as_ref().unwrap();
        self.visit(left);

        let global_scope = self.pop_scope();
        println!("{:?}", global_scope);
        println!("LEAVE scope: global");
        assert!(self.scope.is_none());
    }

    fn compound(&mut self, node: &AST) {
        let left = node.left.as_ref().unwrap();
        self.visit(left);
        if node.right.is_some() {
            let right = node.right.as_ref().unwrap();
            self.visit(right);
        }
    }

    fn procedure_decl(&mut self, node: &AST) {
        println!("ENTER scope: {}", node.get_name());
        let proc_scope = SymbolTable::new(
            node.get_name(),
            self.current_scope().scope_level + 1);
        self.push_scope(proc_scope);
        let left = node.left.as_ref().unwrap();
        self.visit(left);
        let right = node.right.as_ref().unwrap();
        self.visit(right);
        let proc_scope = self.pop_scope();
        println!("{:?}", proc_scope);
        println!("LEAVE scope: {}", node.get_name());
    }

    fn param(&mut self, node: &AST) {
        let left = node.left.as_ref().unwrap();
        self.current_scope().define(&left.root);
        let right = node.right.as_ref().unwrap();
        self.visit(right);
    }

    fn variable_decl(&mut self, node: &AST) {
        let left = node.left.as_ref().unwrap();
        self.current_scope().define(&left.root);
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
        self.current_scope().lookup(&node.root);
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
