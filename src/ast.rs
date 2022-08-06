use crate::interface::AstBase;

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Let(Name, Expr),
}

impl AstBase for Stmt {
    fn accept(&self, ast_visitor: &mut impl crate::interface::AstVisitor) {
        self.visit_children(ast_visitor);
        ast_visitor.visit_stmt(self);
    }

    fn visit_children(&self, ast_visitor: &mut impl crate::interface::AstVisitor) {
        match self {
            Stmt::Expr(expr) => {
                expr.visit_children(ast_visitor);
            }
            Stmt::Let(name, expr) => {
                name.accept(ast_visitor);
                expr.visit_children(ast_visitor);
            }
        }
    }
}

#[derive(Debug)]
pub struct Name {
    pub value: String,
}

impl AstBase for Name {
    fn accept(&self, ast_visitor: &mut impl crate::interface::AstVisitor) {
        ast_visitor.visit_name(self);
    }
}

#[derive(Debug)]
pub enum Expr {
    IntLit(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}

impl AstBase for Expr {
    fn accept(&self, ast_visitor: &mut impl crate::interface::AstVisitor) {
        self.visit_children(ast_visitor);
        ast_visitor.visit_expr(self);
    }

    fn visit_children(&self, ast_visitor: &mut impl crate::interface::AstVisitor) {
        match self {
            Expr::IntLit(_) => ast_visitor.visit_expr(self),
            Expr::Add(left, right) => {
                left.visit_children(ast_visitor);
                right.visit_children(ast_visitor);
            }
            Expr::Sub(left, right) => {
                left.visit_children(ast_visitor);
                right.visit_children(ast_visitor);
            }
        }
    }
}
