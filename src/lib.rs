use app::App;
use cond::Cond;
use lambda::Lambda;
use stlc_err::StlcError;
use utils::{is_value, substitute_expr};

pub mod app;
pub mod cond;
pub mod lambda;
pub mod refsols;
pub mod stlc_err;
pub mod utils;
pub mod ycombinator;

/// Day3-Q5: Choose your favorite upper bound number of evalutation steps
const UPPER_BOUND: u32 = 1000000;

type Result<T> = std::result::Result<T, StlcError>;

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

#[derive(Debug, Eq, PartialEq)]
pub enum Strategy {
    CallByValue,
    CallByName,
}

impl Exp {
    /// The helper function to evaluate one step further
    /// under the specified strategy
    fn eval(self, strategy: Strategy) -> Result<Exp> {
        match self.clone() {
            // The only difference of CBV vs. CBN is choosing
            // when to substitute the inner expression of lambda abstraction
            Exp::App(app) => {
                if strategy == Strategy::CallByValue {
                    // Call By Value
                    match app.t1.clone() {
                        // -----------------------
                        // (\x. t) v -> [x := v] t
                        Exp::Lambda(lambda) => {
                            if is_value(app.t2.clone()) {
                                Ok(substitute_expr(lambda.arg, app.t2, lambda.exp))
                            } else {
                                Ok(Exp::App(Box::new(App::new(app.t1, app.t2.eval(strategy)?))))
                            }
                        }
                        //    t1 -> t1'
                        // ---------------
                        // t1 t2 -> t1' t2
                        _ => {
                            if is_value(app.t1.clone()) {
                                // TODO: format this
                                return Err(StlcError::StuckExpressionCbv(format!("{:#?}", self)));
                            } else {
                                Ok(Exp::App(Box::new(App::new(app.t1.eval(strategy)?, app.t2))))
                            }
                        }
                    }
                } else {
                    // Call By Name
                    match app.t1.clone() {
                        // Note: the key difference between Call-By-Value and Call-By-Name
                        // is that in Call-By-Name strategy, we'd like to perform
                        // beta-reduction *as soon as* possible
                        // Every other rule is essentially the same
                        // -----------------------
                        // (\x. t1) t2  -> [x := t2] t1
                        Exp::Lambda(lambda) => Ok(substitute_expr(lambda.arg, app.t2, lambda.exp)),
                        //    t1 -> t1'
                        // ---------------
                        // t1 t2 -> t1' t2
                        _ => {
                            if is_value(app.t1.clone()) {
                                return Err(StlcError::StuckExpressionCbn(format!("{:#?}", self)));
                            } else {
                                Ok(Exp::App(Box::new(App::new(app.t1.eval(strategy)?, app.t2))))
                            }
                        }
                    }
                }
            }
            Exp::Cond(cond) => match cond.r#if.clone() {
                // -----------------------------
                // if true then t1 else t2 -> t1
                Exp::True => Ok(cond.r#then),
                // ------------------------------
                // if false then t1 else t2 -> t2
                Exp::False => Ok(cond.r#else),
                //                    t1 -> t1'
                // -----------------------------------------------
                // if t1 then t2 else t3 -> if t1' then t2 else t3
                _ => {
                    assert!(
                        !is_value(cond.r#if.clone()),
                        "expect if clause not to be values except `true` or `false`"
                    );
                    Ok(Exp::Cond(Box::new(Cond::new(
                        cond.r#if.eval(strategy)?,
                        cond.r#then,
                        cond.r#else,
                    ))))
                }
            },
            Exp::IsZero(e) => match *e {
                // ----------------    -----------------
                // IsZero 0 -> true && IsZero _ -> false
                Exp::Nat(num) => {
                    if num == 0 {
                        Ok(Exp::True)
                    } else {
                        Ok(Exp::False)
                    }
                }
                //        t -> t'
                // ---------------------
                // IsZero t -> IsZero t'
                _ => Ok(Exp::IsZero(Box::new(e.eval(strategy)?))),
            },
            Exp::Incr(e) => match *e {
                // ---------------
                // Incr n -> n + 1
                Exp::Nat(num) => Ok(Exp::Nat(num.saturating_add(1))),
                //      t -> t'
                // -----------------
                // Incr t -> Incr t'
                _ => Ok(Exp::Incr(Box::new(e.eval(strategy)?))),
            },
            Exp::Decr(e) => match *e {
                // ---------------    -----------
                // Decr n -> n - 1 && Decr 0 -> 0
                Exp::Nat(num) => Ok(Exp::Nat(num.saturating_sub(1))),
                //      t -> t'
                // -----------------
                // Decr t -> Decr t'
                _ => Ok(Exp::Decr(Box::new(e.eval(strategy)?))),
            },
            // TODO: add formatting
            _ => Err(StlcError::InvalidExpression(format!("{:#?}", self))),
        }
    }

    /// Day3-Q1: Write a *helper* function that evaluate *one* step
    /// further using call-by-value evaluation strategy for the given expression
    /// ----
    /// Hint: whenever you stuck, consider review the three operational rules
    /// for call-by-value in the handout, and check if your implementation
    /// accurately follows the rules
    pub fn eval_one_step_cbv(self) -> Result<Exp> {
        self.eval(Strategy::CallByValue)
    }

    /// Day3-Q2: Same as what we have done for cbv,
    /// it's time to implement the same *helper* function for call-by-name!
    /// ----
    /// Hint: the operational rules are your best friends
    pub fn eval_one_step_cbn(self) -> Result<Exp> {
        self.eval(Strategy::CallByName)
    }

    /// Day3-Q3: Write a "driver" function to evaluate the given expression
    /// exactly the given steps, so that we don't need to manually evaluate.
    /// This would be *especially* useful when we are dealing with yCombinator later.
    pub fn eval_multi_step_cbv(mut self, step: u32) -> Result<Exp> {
        for _ in 0..step {
            self = self.eval_one_step_cbv()?;
        }
        Ok(self)
    }

    /// Day3-Q4: Same as cbv, write a "driver" function also for call-by-name strategy.
    pub fn eval_multi_step_cbn(mut self, step: u32) -> Result<Exp> {
        for _ in 0..step {
            self = self.eval_one_step_cbn()?;
        }
        Ok(self)
    }

    /// Day3-Q5: Write a function to help reduce the current expression
    /// to its normal form under the specified strategy.
    pub fn eval_to_normal_form(mut self, strategy: Strategy) -> Result<Exp> {
        for _ in 0..UPPER_BOUND {
            if is_value(self.clone()) {
                return Ok(self);
            }
            self = match strategy {
                Strategy::CallByValue => self.eval_one_step_cbv()?,
                Strategy::CallByName => self.eval_one_step_cbn()?,
            }
        }
        Err(StlcError::ExceedEvalLimit(format!("{:#?}", self)))
    }

    /// Day4-Q1
    pub fn eval_omega() -> ! {
        todo!()
    }

    /// Day4-Q1
    pub fn grow_omega() -> ! {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_one_step_cbv_basic() {
        // (\x. inc x) 1 -> 2
        let exp1 = Exp::App(Box::new(App::new(
            Exp::Lambda(Box::new(Lambda::new(
                "x".to_string(),
                Exp::Incr(Box::new(Exp::Var("x".to_string()))),
            ))),
            Exp::Nat(1),
        )));

        let first_step = exp1.eval_one_step_cbv().unwrap();
        assert_eq!(first_step, Exp::Incr(Box::new(Exp::Nat(1))));

        let second_step = first_step.eval_one_step_cbv().unwrap();
        assert_eq!(second_step, Exp::Nat(2));
    }

    #[test]
    fn test_eval_multi_step_cbv_basic() {
        // (\x. inc x) 1 -> 2
        let exp1 = Exp::App(Box::new(App::new(
            Exp::Lambda(Box::new(Lambda::new(
                "x".to_string(),
                Exp::Incr(Box::new(Exp::Var("x".to_string()))),
            ))),
            Exp::Nat(1),
        )));

        assert_eq!(exp1.eval_multi_step_cbv(2).unwrap(), Exp::Nat(2));
    }

    #[test]
    fn test_eval_multi_step_cbn_basic() {
        // (\x. \y. inc y)
        let exp1 = Exp::Lambda(Box::new(Lambda::new(
            "x".to_string(),
            Exp::Lambda(Box::new(Lambda::new(
                "y".to_string(),
                Exp::Incr(Box::new(Exp::Var("y".to_string()))),
            ))),
        )));
        // omega := (\x. x x) (\x. x x)
        let omega = Exp::App(Box::new(App::new(
            Exp::Lambda(Box::new(Lambda::new(
                "x".to_string(),
                Exp::App(Box::new(App::new(
                    Exp::Var("x".to_string()),
                    Exp::Var("x".to_string()),
                ))),
            ))),
            Exp::Lambda(Box::new(Lambda::new(
                "x".to_string(),
                Exp::App(Box::new(App::new(
                    Exp::Var("x".to_string()),
                    Exp::Var("x".to_string()),
                ))),
            ))),
        )));
        let nat1 = Exp::Nat(1);

        // (\x. \y. inc y) omega 1 -> 2
        let exp = Exp::App(Box::new(App::new(
            Exp::App(Box::new(App::new(exp1, omega))),
            nat1,
        )));
        assert_eq!(exp.eval_multi_step_cbn(3).unwrap(), Exp::Nat(2));
    }
}
