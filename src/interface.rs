use std::fmt::Debug;

use crate::ast::{Expr, Name, Stmt};

pub trait AstVisitor {
    fn visit(&mut self, _node: &impl AstBase) {}

    fn visit_stmt(&mut self, node: &Stmt) {
        self.visit(node)
    }
    fn visit_expr(&mut self, node: &Expr) {
        self.visit(node)
    }
    fn visit_name(&mut self, node: &Name) {
        self.visit(node)
    }
}

pub trait AstBase: Sized + Debug {
    fn accept(&self, ast_visitor: &mut impl AstVisitor);

    fn visit_children(&self, _ast_visitor: &mut impl AstVisitor) {}
}
