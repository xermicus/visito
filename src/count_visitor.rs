use crate::interface::{AstBase, AstVisitor};

/// A visitor that counts all nodes in the AST
#[derive(Default, Debug)]
pub struct CountVisitor(i64);

impl AstVisitor for CountVisitor {
    fn visit(&mut self, node: &impl AstBase) {
        node.visit_children(self);
        println!("{:?}", node);
        self.0 += 1;
    }
}
