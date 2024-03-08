use expr::{app::App, cond::Cond, cons::Cons, lambda::Lambda};
use stlc_err::StlcError;
use utils::{is_value, substitute_expr};

pub mod expr;
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
    // TODO(Day1-Q1): Add your self-defined list encoding syntax here.
    // Feel free to play with it in `main.rs` and encoding it just
    // like any other `Exp` we've seen so far.
    Nil,
    Cons(Box<Cons>),
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

    /// TODO(Day3-Q1): Write a *helper* function that evaluate *one* step
    /// further using call-by-value evaluation strategy for the given expression
    /// ----
    /// Hint: whenever you stuck, consider review the three operational rules
    /// for call-by-value in the handout, and check if your implementation
    /// accurately follows the rules
    pub fn eval_one_step_cbv(self) -> Result<Exp> {
        self.eval(Strategy::CallByValue)
    }

    /// TODO(Day3-Q2): Same as what we have done for cbv,
    /// it's time to implement the same *helper* function for call-by-name!
    /// ----
    /// Hint: the operational rules are your best friends
    pub fn eval_one_step_cbn(self) -> Result<Exp> {
        self.eval(Strategy::CallByName)
    }

    /// TODO(Day3-Q3): Write a "driver" function to evaluate the given expression
    /// exactly the given steps, so that we don't need to manually evaluate.
    /// Of course, you need to distinguish between different evaluation strategies.
    /// This would be *especially* useful when we are dealing with yCombinator later.
    pub fn eval_multi_step(mut self, step: u32, strategy: Strategy) -> Result<Exp> {
        for _ in 0..step {
            self = match strategy {
                Strategy::CallByValue => self.eval_one_step_cbv()?,
                Strategy::CallByName => self.eval_one_step_cbn()?,
            }
        }
        Ok(self)
    }

    /// TODO(Day3-Q4): Write a function to help reduce the current expression
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

    /// TODO(Day4-Q1): Write a function to observe different behavior when we
    /// evaluating expression with omega under two evaluation strategies.
    /// The current function signature indicates it will never return,
    /// but of course, feel free to change this.
    pub fn eval_omega() -> ! {
        todo!()
    }

    /// TODO(Day4-Q2): After knowing what a omega is, could you think of any expression
    /// that will *grow* larger after each evaluation step?
    /// Write the expression down and evaluate it here to prove your answer.
    pub fn grow_omega() -> ! {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_one_step_basic() {
        // (\x. inc x) 1 -> 2
        let exp1 = Exp::App(Box::new(App::new(
            Exp::Lambda(Box::new(Lambda::new(
                "x".to_string(),
                Exp::Incr(Box::new(Exp::Var("x".to_string()))),
            ))),
            Exp::Nat(1),
        )));

        // CBV
        let first_step_cbv = exp1.clone().eval_one_step_cbv().unwrap();
        assert_eq!(first_step_cbv, Exp::Incr(Box::new(Exp::Nat(1))));

        let second_step_cbv = first_step_cbv.eval_one_step_cbv().unwrap();
        assert_eq!(second_step_cbv, Exp::Nat(2));

        // CBN should produce the exact same step
        let first_step_cbn = exp1.eval_one_step_cbn().unwrap();
        assert_eq!(first_step_cbn, Exp::Incr(Box::new(Exp::Nat(1))));

        let second_step_cbn = first_step_cbn.eval_one_step_cbn().unwrap();
        assert_eq!(second_step_cbn, Exp::Nat(2));
    }

    #[test]
    fn test_eval_multi_step_basic() {
        // (\x. inc x) 1 -> 2
        let exp1 = Exp::App(Box::new(App::new(
            Exp::Lambda(Box::new(Lambda::new(
                "x".to_string(),
                Exp::Incr(Box::new(Exp::Var("x".to_string()))),
            ))),
            Exp::Nat(1),
        )));

        // CBV
        assert_eq!(
            exp1.clone()
                .eval_multi_step(2, Strategy::CallByValue)
                .unwrap(),
            Exp::Nat(2)
        );
        // CBN
        assert_eq!(
            exp1.eval_multi_step(2, Strategy::CallByName).unwrap(),
            Exp::Nat(2)
        );

        // (\x. \y. inc y)
        let exp1 = Exp::Lambda(Box::new(Lambda::new(
            "x".to_string(),
            Exp::Lambda(Box::new(Lambda::new(
                "y".to_string(),
                Exp::Incr(Box::new(Exp::Var("y".to_string()))),
            ))),
        )));
        // ω := (\x. x x) (\x. x x)
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

        // (\x. \y. inc y) ω 1 -> 2
        let exp = Exp::App(Box::new(App::new(
            Exp::App(Box::new(App::new(exp1, omega))),
            nat1,
        )));

        // Call-By-Name
        assert_eq!(
            exp.eval_multi_step(3, Strategy::CallByName).unwrap(),
            Exp::Nat(2)
        );

        // Try to use `eval_multi_step` with Call-By-Value strategy here and observe the result.
        //                            cbv
        // i.e., (\x. \y. inc y) ω 1 -----> ???
        // ----
        // Q1: Is the result conforming to your expectation?
        // Q2: Is it different from Call-By-Name strategy?
        // If your answer to Q2 is yes, then why is it different?
        // ----
        // P.S. If you don't know the answer yet, just put a pin here.
        // Hopefully you could answer this question after day 4.
    }
}
