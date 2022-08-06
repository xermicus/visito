use crate::ast::{Expr, Name, Stmt};

pub trait AstVisitor {
    fn visit_stmt(&mut self, _node: &Stmt) {}
    fn visit_expr(&mut self, _node: &Expr) {}
    fn visit_name(&mut self, _node: &Name) {}
}

pub trait AstBase: Sized {
    fn accept(&self, ast_visitor: &mut impl AstVisitor);

    fn visit_children(&self, _ast_visitor: &mut impl AstVisitor) {}
}
