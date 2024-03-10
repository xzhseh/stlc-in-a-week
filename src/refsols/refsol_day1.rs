//! Day-Q1 encoding exercises reference solution

use crate::{
    expr::{app::App, cond::Cond, incr::Incr, lambda::Lambda, var::Var},
    Exp,
};

pub struct Exp1;
impl Exp1 {
    /// TODO(Day1-Q1): Encode `\x. x` here
    pub fn new() -> Exp {
        Lambda::build("x", Var::build("x"))
    }
}

pub struct Exp2;
impl Exp2 {
    /// TODO(Day1-Q1): Encode `(\x. inc x) 1` here
    pub fn new() -> Exp {
        App::build(Lambda::build("x", Incr::build(Var::build("x"))), 1.into())
    }
}

pub struct Exp3;
impl Exp3 {
    /// TODO(Day1-Q1): Encode `(\x. x) ((\y. y) (\z. z))` here
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
    /// TODO(Day1-Q1): Encode `((\x. x) (\y. if y then false else true)) true` here
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
    /// TODO(Day1-Q1): Encode `(\x. x x) (\x. x x)` here
    pub fn new() -> Exp {
        let e = Lambda::build("x", App::build(Var::build("x"), Var::build("x")));
        App::build(e.clone(), e)
    }
}
