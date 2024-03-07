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
    /// Day3-Q1: Write a *helper* function that evaluate *one* step
    /// further using call-by-value evaluation strategy for the given expression
    /// ----
    /// Hint: whenever you stuck, consider review the three operational rules
    /// for call-by-value in the handout, and check if your implementation
    /// accurately follows the rules
    pub fn eval_one_step_cbv(&self, exp: Exp) -> Exp {
        match exp {

            _ => panic!(),
        }
    }

    /// Day3-Q2: Same as what we have done for cbv,
    /// it's time to implement the same *helper* function for call-by-name!
    /// ----
    /// Hint: the operational rules are your best friends
    pub fn eval_one_step_cbn(&self, _exp: Exp) -> Exp {
        todo!()
    }

    /// Day4-Q1
    pub fn eval_multi_step_cbv(&self, _exp: Exp, _step: u32) -> Exp {
        todo!()
    }

    /// Day4-Q2
    pub fn eval_multi_step_cbn(&self, _exp: Exp, _step: u32) -> Exp {
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
