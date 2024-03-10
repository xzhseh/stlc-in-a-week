//! This exercise mainly aims at checking your understanding of
//! our basic stlc at this moment.
//! Treat this as an chance to get familiarize with lambda calculus (and of course, Rust itself)
//! The functions to implement may or may not be used in the future (yes, probably just for fun)
//! For reference solution, feel free to check `src/refsols/refsol_day2.rs`.

use crate::{
    expr::{app::App, cond::Cond, lambda::Lambda},
    Exp,
};

impl Exp {
    /// TODO(Day2-Q1): Write a function to check whether or not
    /// the given variable is "free" in the provided expression.
    /// To say a variable is free, basically we need to check if
    /// it has been *bound* to some outer lambda abstraction(s).
    /// e.g., In `\x. \y. x y z`, `x` is bound by the first lambda
    /// abstraction, while `y` is bound by the second lambda abstraction
    /// `z` in this case is *free*, so the following should apply, i.e.,
    ///
    ///  1. (`\x. \y. x y z`, `x`).appears_free_in() => False
    ///  2. (`\x. \y. x y z`, `y`).appears_free_in() => False
    ///  3. (`\x. \y. x y z`, `z`).appears_free_in() => True
    ///
    /// Note: We will assume that there are NOT any lamda abstractions in
    /// its body that binds the same variable.
    /// e.g., `\x. \y. x (\z. \x. z x) y` will not be included.
    pub fn appears_free_in(&self, var: String) -> bool {
        /// Helper function to check if the input *variable* appears
        /// in the given expression
        fn check_var_appear(exp: Exp, var: String) -> bool {
            match exp {
                Exp::Var(v) => var == v,
                Exp::Lambda(lambda) => check_var_appear(lambda.exp, var),
                Exp::App(app) => {
                    check_var_appear(app.t1, var.clone()) || check_var_appear(app.t2, var)
                }
                Exp::Cond(cond) => {
                    check_var_appear(cond.r#if, var.clone())
                        || check_var_appear(cond.r#then, var.clone())
                        || check_var_appear(cond.r#else, var)
                }
                Exp::IsZero(e) => check_var_appear(*e, var),
                Exp::Incr(e) => check_var_appear(*e, var),
                Exp::Decr(e) => check_var_appear(*e, var),
                _ => false,
            }
        }

        /// Helper function to check if the *lambda abstraction* appears
        /// in the given expression
        fn check_lambda_appear(exp: Exp, var: String) -> bool {
            match exp {
                Exp::Lambda(lambda) => lambda.arg == var || check_lambda_appear(lambda.exp, var),
                Exp::App(app) => {
                    check_lambda_appear(app.t1, var.clone()) || check_lambda_appear(app.t2, var)
                }
                Exp::Cond(cond) => {
                    check_lambda_appear(cond.r#if, var.clone())
                        || check_lambda_appear(cond.r#then, var.clone())
                        || check_lambda_appear(cond.r#else, var)
                }
                Exp::IsZero(e) => check_lambda_appear(*e, var),
                Exp::Incr(e) => check_lambda_appear(*e, var),
                Exp::Decr(e) => check_lambda_appear(*e, var),
                _ => false,
            }
        }

        !(check_var_appear(self.clone(), var.clone()) && check_lambda_appear(self.clone(), var))
    }

    /// TODO(Day2-Q2): Write a function to check whether or not the input `Exp`
    /// is a *value*, per the definition below.
    ///
    /// v ::= \x. t         -- lambda abstraction
    ///       | true        -- constant true
    ///       | false       -- constant false
    ///       | n           -- natural number
    pub fn is_value(&self) -> bool {
        match self.clone() {
            Exp::Lambda(_) | Exp::True | Exp::False | Exp::Nat(_) => true,
            _ => false,
        }
    }

    /// TODO(Day2-Q3): Write a function that perform a *substitution* on the
    /// current expression. i.e., if any of the following
    /// substitution rules applies, reduce `origin` expression by
    /// the corresponding substitution. `var` is the exact variable
    /// to be reduced.
    /// Note: this function will *consume* the current expression and
    /// return a brand new substituted expression.
    ///
    /// ```ignore
    ///     [x := s] x                       = s
    ///     [x := s] y                       = y, if x != y
    ///     [x := s] (\x. t)                 = \x. t
    ///     [x := s] (\y. t)                 = \y. [x := s] t, if x != y
    ///     [x := s] (t1 t2)                 = ([x := s] t1) ([x := s] t2)
    ///     [x := s] (inc t)                 = inc ([x := s] t)
    ///     [x := s] (dec t)                 = dec ([x := s] t)
    ///     [x := s] (IsZero t)              = IsZero ([x := s] t)
    ///     [x := s] true                    = true
    ///     [x := s] false                   = false
    ///     [x := s] n                       = n
    ///     [x := s] (if t1 then t2 else t3) = if [x := s] t1 then [x := s] t2 else [x := s] t3
    /// ```
    pub fn substitute(self, var: String, s: Exp) -> Exp {
        match self.clone() {
            // [x := s] x && [x := s] y
            Exp::Var(v) => {
                if v == var {
                    s
                } else {
                    self
                }
            }
            // [x := s] (\x. t) && [x := s] (\y. t)
            Exp::Lambda(lambda) => {
                if lambda.arg == var {
                    self
                } else {
                    Exp::Lambda(Box::new(Lambda::new(
                        lambda.arg,
                        lambda.exp.substitute(var, s),
                    )))
                }
            }
            // [x := s] (t1 t2)
            Exp::App(app) => Exp::App(Box::new(App::new(
                app.t1.substitute(var.clone(), s.clone()),
                app.t2.substitute(var, s),
            ))),
            // [x := s] (if t1 then t2 else t3)
            Exp::Cond(cond) => Exp::Cond(Box::new(Cond::new(
                cond.r#if.substitute(var.clone(), s.clone()),
                cond.r#then.substitute(var.clone(), s.clone()),
                cond.r#else.substitute(var, s),
            ))),
            // [x := s] (inc t)
            Exp::Incr(e) => Exp::Incr(Box::new(e.substitute(var, s))),
            // [x := s] (dec t)
            Exp::Decr(e) => Exp::Decr(Box::new(e.substitute(var, s))),
            // [x := s] true && [x := s] false && [x := s] n
            e => e,
        }
    }
}
