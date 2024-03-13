//! Day-Q1 encoding exercises reference solution

use crate::{
    expr::{app::App, cond::Cond, incr::Incr, lambda::Lambda, var::Var},
    Exp,
};

pub struct Exp1;
impl Exp1 {
    /// `λx. x`
    pub fn new() -> Exp {
        Lambda::build("x", Var::build("x"))
    }
}

pub struct Exp2;
impl Exp2 {
    /// `(λx. incr x) 1`
    pub fn new() -> Exp {
        App::build(Lambda::build("x", Incr::build(Var::build("x"))), 1.into())
    }
}

pub struct Exp3;
impl Exp3 {
    /// `(λx. x) ((λy. y) (λz. z))`
    pub fn new() -> Exp {
        App::build(
            Lambda::build("x", Var::build("x")),
            App::build(
                Lambda::build("y", Var::build("y")),
                Lambda::build("z", Var::build("z")),
            ),
        )
    }
}

pub struct Exp4;
impl Exp4 {
    /// `((λx. x) (λy. if y then false else true)) true`
    pub fn new() -> Exp {
        App::build(
            App::build(
                Lambda::build("x", Var::build("x")),
                Lambda::build("y", Cond::build(Var::build("y"), Exp::False, Exp::True)),
            ),
            Exp::True,
        )
    }
}

pub struct Exp5;
impl Exp5 {
    /// `(λx. x x) (λx. x x)`
    pub fn new() -> Exp {
        let e = Lambda::build("x", App::build(Var::build("x"), Var::build("x")));
        App::build(e.clone(), e)
    }
}

pub struct OpenQuestions;
impl OpenQuestions {
    #[allow(dead_code)]
    pub fn q1() -> &'static str {
        "put your answer to the first open question here"
    }

    #[allow(dead_code)]
    pub fn q2() -> &'static str {
        "put your answer to the second open question here"
    }
}
