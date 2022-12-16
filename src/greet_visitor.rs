use crate::{ast::Name, interface::AstBase, interface::AstVisitor};

/// A visitor that prints all names in the AST
pub struct GreetVisitor;

impl AstVisitor for GreetVisitor {
    fn visit_name(&mut self, node: &Name) {
        node.visit_children(self);
        println!("Node is called: {}", node.value);
    }
}
