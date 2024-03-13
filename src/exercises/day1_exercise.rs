//! Used for Day1-Q1 encoding exercises
//! Feel free to check `src/refsols/refsol_day1.rs` when you get stuck

use crate::Exp;

pub struct Exp1;
impl Exp1 {
    /// TODO(Day1-Q1): Encode `λx. x` here
    pub fn new() -> Exp {
        todo!()
    }
}

pub struct Exp2;
impl Exp2 {
    /// TODO(Day1-Q1): Encode `(λx. incr x) 1` here
    pub fn new() -> Exp {
        todo!()
    }
}

pub struct Exp3;
impl Exp3 {
    /// TODO(Day1-Q1): Encode `(λx. x) ((λy. y) (λz. z))` here
    pub fn new() -> Exp {
        todo!()
    }
}

pub struct Exp4;
impl Exp4 {
    /// TODO(Day1-Q1): Encode `((λx. x) (λy. if y then false else true)) true` here
    pub fn new() -> Exp {
        todo!()
    }
}

pub struct Exp5;
impl Exp5 {
    /// TODO(Day1-Q1): Encode `(λx. x x) (λx. x x)` here
    pub fn new() -> Exp {
        todo!()
    }
}

pub struct OpenQuestions;
impl OpenQuestions {
    /// TODO(Day1-Q3): answer the first open question
    /// i.e., regarding constructor "syntax sugar"
    #[allow(dead_code)]
    pub fn q1() -> &'static str {
        "put your answer to the first open question here"
    }

    /// TODO(Day1-Q3): answer the second open question
    /// i.e., regarding the different choice between "var" & "nat"
    #[allow(dead_code)]
    pub fn q2() -> &'static str {
        "put your answer to the second open question here"
    }
}
