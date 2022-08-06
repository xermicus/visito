use crate::{
    ast::{Expr, Name, Stmt},
    interface::{AstBase, AstVisitor},
};

#[derive(Default, Debug)]
pub struct CountVisitor(i64);

impl AstVisitor for CountVisitor {
    fn visit_name(&mut self, node: &Name) {
        node.visit_children(self);
        println!("{:?}", node);
        self.0 += 1;
    }

    fn visit_expr(&mut self, node: &Expr) {
        node.visit_children(self);
        println!("{:?}", node);
        self.0 += 1;
    }

    fn visit_stmt(&mut self, node: &Stmt) {
        node.visit_children(self);
        println!("{:?}", node);
        self.0 += 1;
    }
}
