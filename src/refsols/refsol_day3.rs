use crate::{
    expr::{app::App, cond::Cond},
    stlc_err::StlcError,
    Exp, Strategy,
};

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
                            if app.t2.ref_is_value() {
                                Ok(lambda.exp.ref_substitute(lambda.arg, app.t2))
                            } else {
                                Ok(Exp::App(Box::new(App::new(app.t1, app.t2.eval(strategy)?))))
                            }
                        }
                        //    t1 -> t1'
                        // ---------------
                        // t1 t2 -> t1' t2
                        _ => {
                            if app.t1.ref_is_value() {
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
                        Exp::Lambda(lambda) => Ok(lambda.exp.ref_substitute(lambda.arg, app.t2)),
                        //    t1 -> t1'
                        // ---------------
                        // t1 t2 -> t1' t2
                        _ => {
                            if app.t1.ref_is_value() {
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
                    if cond.r#if.ref_is_value() {
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

    pub fn ref_eval_one_step_cbv(self) -> Result<Exp> {
        self.eval(Strategy::CallByValue)
    }

    pub fn ref_eval_one_step_cbn(self) -> Result<Exp> {
        self.eval(Strategy::CallByName)
    }

    pub fn ref_eval_multi_step(mut self, step: u32, strategy: Strategy) -> Result<Exp> {
        for _ in 0..step {
            self = match strategy {
                Strategy::CallByValue => self.ref_eval_one_step_cbv()?,
                Strategy::CallByName => self.ref_eval_one_step_cbn()?,
            }
        }
        Ok(self)
    }

    fn ref_upper_bound(&self) -> u32 {
        1000000
    }

    pub fn ref_eval_to_normal_form(mut self, strategy: Strategy) -> Result<(Exp, u32)> {
        for i in 1..=self.ref_upper_bound() {
            if self.ref_is_value() {
                return Ok((self, i));
            }
            self = match strategy {
                Strategy::CallByValue => self.ref_eval_one_step_cbv()?,
                Strategy::CallByName => self.ref_eval_one_step_cbn()?,
            }
        }
        Err(StlcError::ExceedEvalLimit(format!(
            "exceed evaluation limit, current expr: {}",
            self
        )))
    }
}
