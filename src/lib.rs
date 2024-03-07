use app::App;
use cond::Cond;
use lambda::Lambda;
use utils::{is_value, substitute_expr};

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
        match exp.clone() {
            Exp::App(app) => match app.t1.clone() {
                // -----------------------
                // (\x. t) v -> [x := v] t
                Exp::Lambda(lambda) => {
                    if is_value(app.t2.clone()) {
                        substitute_expr(lambda.arg, app.t2, lambda.exp)
                    } else {
                        Exp::App(Box::new(App::new(app.t1, self.eval_one_step_cbv(app.t2))))
                    }
                }
                //    t1 -> t1'
                // ---------------
                // t1 t2 -> t1' t2
                _ => {
                    if is_value(app.t1.clone()) {
                        panic!("invalid expression: {:#?}", exp);
                    } else {
                        Exp::App(Box::new(App::new(self.eval_one_step_cbv(app.t1), app.t2)))
                    }
                }
            },
            Exp::Cond(cond) => match cond.r#if.clone() {
                // -----------------------------
                // if true then t1 else t2 -> t1
                Exp::True => cond.r#then,
                // ------------------------------
                // if false then t1 else t2 -> t2
                Exp::False => cond.r#else,
                //                    t1 -> t1'
                // -----------------------------------------------
                // if t1 then t2 else t3 -> if t1' then t2 else t3
                _ => {
                    assert!(
                        !is_value(cond.r#if.clone()),
                        "expect if clause not to be values except `true` or `false`"
                    );
                    Exp::Cond(Box::new(Cond::new(
                        self.eval_one_step_cbv(cond.r#if),
                        cond.r#then,
                        cond.r#else,
                    )))
                }
            },
            Exp::IsZero(e) => match *e {
                // ----------------    -----------------
                // IsZero 0 -> true && IsZero _ -> false
                Exp::Nat(num) => {
                    if num == 0 {
                        Exp::True
                    } else {
                        Exp::False
                    }
                }
                //        t -> t'
                // ---------------------
                // IsZero t -> IsZero t'
                _ => Exp::IsZero(Box::new(self.eval_one_step_cbv(*e))),
            },
            Exp::Incr(e) => match *e {
                // ---------------
                // Incr n -> n + 1
                Exp::Nat(num) => Exp::Nat(num.saturating_add(1)),
                //      t -> t'
                // -----------------
                // Incr t -> Incr t'
                _ => Exp::Incr(Box::new(self.eval_one_step_cbv(*e))),
            },
            Exp::Decr(e) => match *e {
                // ---------------    -----------
                // Decr n -> n - 1 && Decr 0 -> 0
                Exp::Nat(num) => Exp::Nat(num.saturating_sub(1)),
                //      t -> t'
                // -----------------
                // Decr t -> Decr t'
                _ => Exp::Decr(Box::new(self.eval_one_step_cbv(*e))),
            },
            _ => panic!("invalid expression: {:#?}", exp),
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
