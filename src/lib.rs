use core::fmt;

use expr::{app::App, cond::Cond, lambda::Lambda};

pub mod exercises;
pub mod expr;
pub mod refsols;
pub mod stlc_err;
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
    /// Non-negative number, i.e., n
    Nat(u32),
    /// IsZero, think of this as a special application expression, i.e., IsZero t
    IsZero(Box<Exp>),
    /// Increment, i.e., inc exp
    Incr(Box<Exp>),
    /// Decrement, i.e., dec exp
    Decr(Box<Exp>),
    // TODO(Day1-Q2): Add your self-defined syntax here.
    // Feel free to play with it in `main.rs` and encoding it just
    // like any other `Exp` we've seen so far.
}

#[derive(Debug, Eq, PartialEq)]
pub enum Strategy {
    CallByValue,
    CallByName,
    // TODO(General): adding other evaluation strategy, PR(s) welcome!
    // e.g., call by reference, call by need, etc.
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Exp::Var(v) => write!(f, "{}", v),
            Exp::Lambda(lambda) => write!(f, "{}", *lambda),
            Exp::App(app) => write!(f, "{}", *app),
            Exp::Cond(cond) => write!(f, "{}", *cond),
            Exp::True => write!(f, "true"),
            Exp::False => write!(f, "false"),
            Exp::Nat(n) => write!(f, "{}", n),
            Exp::IsZero(e) => write!(f, "is_zero ({})", *e),
            Exp::Incr(e) => write!(f, "incr ({})", *e),
            Exp::Decr(e) => write!(f, "decr ({})", *e),
        }
    }
}
