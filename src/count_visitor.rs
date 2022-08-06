use crate::{
    ast::{Expr, Stmt},
    interface::AstVisitor,
};

#[derive(Default, Debug)]
pub struct CountVisitor(i64);

impl AstVisitor for CountVisitor {
    fn visit_name(&mut self, node: &crate::ast::Name) {
        println!("{:?}", node);
        self.0 += 1;
    }

    fn visit_expr(&mut self, node: &Expr) {
        println!("{:?}", node);
        self.0 += 1;
    }

    fn visit_stmt(&mut self, node: &Stmt) {
        println!("{:?}", node);
        self.0 += 1;
    }
}
