//! This exercise mainly aims at checking your understanding of
//! our basic stlc at this moment.
//! Treat this as an chance to get familiarize with lambda calculus (and of course, Rust itself)
//! The functions to implement may or may not be used in the future (yes, probably just for fun)
//! For reference solution, feel free to check `src/refsols/refsol_day2.rs`.

use std::collections::HashSet;

use crate::{
    expr::{app::App, cond::Cond, decr::Decr, incr::Incr, is_zero::IsZero, lambda::Lambda},
    Exp,
};

impl Exp {
    /// TODO(Day2-Q1): Write a function to check whether or not
    /// the given variable *at least once* "free" in the provided expression.
    /// To say a variable is free, basically we need to check if
    /// it has been *bound* to some outer lambda abstraction(s).
    /// e.g., In `\x. \y. x y z`, `x` is bound by the first lambda
    /// abstraction, while `y` is bound by the second lambda abstraction
    /// `z` in this case is *free*, so the following should apply, i.e.,
    ///
    ///  1. (`\x. \y. x y z`).appears_free_in(`x`) => False
    ///  2. (`\x. \y. x y z`).appears_free_in(`y`) => False
    ///  3. (`\x. \y. x y z`).appears_free_in(`z`) => True
    ///  4. `(\x. x) (\y. x y)`.appears_free_in(`x`) => True
    ///     => (explanation: `x` is not bound in `\y. x y`)
    ///
    /// Note: We will assume that there are NOT any lamda abstractions in
    /// its body that binds the same variable.
    /// e.g., `\x. \y. x (\z. \x. z x) y` will not be included.
    pub fn appears_free_in(&self, var: &str) -> bool {
        /// A context based solution
        fn appears_free_in_inner(exp: Exp, var: &str, context: &mut HashSet<String>) -> bool {
            match exp.clone() {
                Exp::Lambda(lambda) => {
                    // update context before diving in
                    context.insert(lambda.arg.clone());
                    let result = appears_free_in_inner(lambda.exp, var, context);
                    // clear the context before exiting
                    // note that here we do not need to consider accidentially clearing
                    // the outer context, per the assumption above.
                    // i.e., case like `\x. y (\x. x) x` will not be included
                    context.remove(&lambda.arg);
                    result
                }
                Exp::Var(v) => {
                    if v == var.to_string() {
                        !context.contains(&v)
                    } else {
                        // not the variable we are interested in
                        false
                    }
                }
                Exp::App(app) => {
                    appears_free_in_inner(app.t1, var, context)
                        || appears_free_in_inner(app.t2, var, context)
                }
                Exp::Cond(cond) => {
                    appears_free_in_inner(cond.r#if, var, context)
                        || appears_free_in_inner(cond.r#then, var, context)
                        || appears_free_in_inner(cond.r#else, var, context)
                }
                Exp::IsZero(e) | Exp::Incr(e) | Exp::Decr(e) => {
                    appears_free_in_inner(*e, var, context)
                }
                Exp::Nat(_) | Exp::True | Exp::False => false,
            }
        }

        appears_free_in_inner(self.clone(), var, &mut HashSet::new())
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
                    Lambda::build(&lambda.arg, lambda.exp.substitute(var, s))
                }
            }
            // [x := s] (t1 t2)
            Exp::App(app) => App::build(
                app.t1.substitute(var.clone(), s.clone()),
                app.t2.substitute(var, s),
            ),
            // [x := s] (if t1 then t2 else t3)
            Exp::Cond(cond) => Cond::build(
                cond.r#if.substitute(var.clone(), s.clone()),
                cond.r#then.substitute(var.clone(), s.clone()),
                cond.r#else.substitute(var, s),
            ),
            Exp::IsZero(e) => IsZero::build(e.substitute(var, s)),
            // [x := s] (inc t)
            Exp::Incr(e) => Incr::build(e.substitute(var, s)),
            // [x := s] (dec t)
            Exp::Decr(e) => Decr::build(e.substitute(var, s)),
            // [x := s] true && [x := s] false && [x := s] n
            e => e,
        }
    }
}
