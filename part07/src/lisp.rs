use crate::parser::*;
use crate::ast::*;

/// LISP style notation

struct Translator {
    parser: Parser,
}

impl Translator {
    fn new<S: Into<String>>(text: S) -> Translator {
        Translator {
            parser: Parser::new(text),
        }
    }

    fn exec(&mut self) -> String {
        let tree = self.parser.parse();
        tree.visit()
    }
}

trait NodeVisitor {
    fn visit(&self) -> String;
}

impl NodeVisitor for AST {
    fn visit(&self) -> String {
        match self.root {
            Root::Num(n) => n.to_string(),
            Root::Op(op) => {
                let left = self.left.as_ref().unwrap();
                let right = self.right.as_ref().unwrap();
                format!("({} {} {})", op, left.visit(), right.visit())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lisp() {
        assert_eq!(Translator::new("2 + 3").exec(), "(+ 2 3)");
        assert_eq!(Translator::new("2 + 3 * 5").exec(), "(+ 2 (* 3 5))");
        assert_eq!(Translator::new("7 + 5 * 2 - 3").exec(), "(- (+ 7 (* 5 2)) 3)");
        assert_eq!(Translator::new("1 + 2 + 3 + 4 + 5").exec(), "(+ (+ (+ (+ 1 2) 3) 4) 5)");
    }
}
