use greet_visitor::GreetVisitor;

use crate::{
    ast::{Expr, Name, Stmt},
    count_visitor::CountVisitor,
    interface::AstBase,
};

mod ast;
mod count_visitor;
mod greet_visitor;
mod interface;

fn count(program: &Stmt) {
    let mut counter = CountVisitor::default();
    program.accept(&mut counter);
    println!("{:?}", counter);
}

fn greet(program: &Stmt) {
    let mut name_printer = GreetVisitor;
    program.accept(&mut name_printer);
}

fn main() {
    count(&Stmt::Expr(Expr::Add(
        Box::new(Expr::IntLit(1)),
        Box::new(Expr::IntLit(2)),
    )));

    let program = Stmt::Let(
        Name {
            value: "foo".into(),
        },
        Expr::Sub(Box::new(Expr::IntLit(2)), Box::new(Expr::IntLit(1))),
    );
    count(&program);
    greet(&program);
}
