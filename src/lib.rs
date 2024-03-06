use app::App;
use cond::Cond;
use lambda::Lambda;

pub mod app;
pub mod cond;
pub mod lambda;
pub mod utils;
pub mod ycombinator;

/// The definition for our (currently) untyped lambda calculus
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Exp {
    /// Variable, which literally could be anything!
    Var(String),
    /// Lambda abstraction, probably the most important base for our stlc, i.e., \x. t
    Lambda(Box<Lambda>),
    /// Application, i.e., t1 t2
    App(Box<App>),
    /// Condition, i.e., if t1 then t2 else t3
    Cond(Box<Cond>),
    /// Constant True, i.e., true
    True,
    /// Constant False, i.e., false
    False,
    /// Naturual number, i.e., n
    Nat(u32),
    /// IsZero, think of this as a special application expression, i.e., IsZero n
    IsZero(Box<Exp>),
    /// Increment, i.e., inc exp
    Incr(Box<Exp>),
    /// Decrement, i.e., dec exp
    Decr(Box<Exp>),
}

impl Exp {
    /// Day3-Q1
    pub fn eval_one_step_cbv(_exp: Exp) -> Exp {
        todo!()
    }

    /// Day3-Q2
    pub fn eval_one_step_cbn(_exp: Exp) -> Exp {
        todo!()
    }

    /// Day4-Q1
    pub fn eval_multi_step_cbv(_exp: Exp) -> Exp {
        todo!()
    }

    /// Day4-Q2
    pub fn eval_multi_step_cbn(_exp: Exp) -> Exp {
        todo!()
    }

    /// Day4-Q3
    pub fn eval_omega() -> ! {
        todo!()
    }

    /// Day4-Q4
    pub fn grow_omega() -> ! {
        todo!()
    }
}