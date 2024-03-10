use crate::{expr::{app::App, cond::Cond}, stlc_err::StlcError, Exp, Strategy};

type Result<T> = std::result::Result<T, StlcError>;

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
                            if app.t2.is_value() {
                                Ok(lambda.exp.substitute(lambda.arg, app.t2))
                            } else {
                                Ok(Exp::App(Box::new(App::new(app.t1, app.t2.eval(strategy)?))))
                            }
                        }
                        //    t1 -> t1'
                        // ---------------
                        // t1 t2 -> t1' t2
                        _ => {
                            if app.t1.is_value() {
                                return Err(StlcError::StuckExpressionCbv(format!("{}", self)));
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
                        Exp::Lambda(lambda) => Ok(lambda.exp.substitute(lambda.arg, app.t2)),
                        //    t1 -> t1'
                        // ---------------
                        // t1 t2 -> t1' t2
                        _ => {
                            if app.t1.is_value() {
                                return Err(StlcError::StuckExpressionCbn(format!("{}", self)));
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
                    if !cond.r#if.is_value() {
                        return Err(StlcError::InvalidExpression(
                            format!("expect if clause not to be values other than `true` or `false, actual: {}", cond.r#if)
                        ));
                    }
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
            _ => Err(StlcError::InvalidExpression(format!("{}", self))),
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
    /// exactly the given steps, so that we don't need to manually evaluate it step by step.
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

    /// TODO(Day3-Q4'): Choose your favorite upper bound number of evalutation steps
    fn upper_bound(&self) -> u32 {
        114514
    }

    /// TODO(Day3-Q4): Write a function to help reduce the current expression
    /// to its normal form under the specified strategy.
    pub fn eval_to_normal_form(mut self, strategy: Strategy) -> Result<Exp> {
        for _ in 0..self.upper_bound() {
            if self.is_value() {
                return Ok(self);
            }
            self = match strategy {
                Strategy::CallByValue => self.eval_one_step_cbv()?,
                Strategy::CallByName => self.eval_one_step_cbn()?,
            }
        }
        Err(StlcError::ExceedEvalLimit(format!(
            "exceed evaluation limit, current expr: {}",
            self
        )))
    }
}