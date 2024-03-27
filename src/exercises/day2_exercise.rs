//! This exercise mainly aims at checking your understanding of
//! our basic stlc at this moment.
//! Treat this as an chance to get familiarize with lambda calculus (and of course, Rust itself)
//! The functions to implement may or may not be used in the future (yes, probably just for fun)
//! For reference solution, feel free to check `src/refsols/refsol_day2.rs`.

use crate::Exp;

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
    pub fn appears_free_in(&self, _var: &str) -> bool {
        todo!()
    }

    /// TODO(Day2-Q2): Write a function to check whether or not the input `Exp`
    /// is a *value*, per the definition below.
    ///
    /// v ::= \x. t         -- lambda abstraction
    ///       | true        -- constant true
    ///       | false       -- constant false
    ///       | n           -- natural number
    pub fn is_value(&self) -> bool {
        todo!()
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
    pub fn substitute(self, _var: String, _s: Exp) -> Exp {
        todo!()
    }
}

/// Please check whether the given expression is valid or not
/// under the context of our currently untyped lambda calculus
/// after we have learned about the *semantics*.
/// Don't forget to include your explanation.
pub struct ValidExpressions;
impl ValidExpressions {
    /// TODO(Day2-Q4): `incr (λx. x)`
    pub fn q1() -> (bool, &'static str) {
        todo!()
    }

    /// TODO(Day2-Q4): `app true false`
    pub fn q2() -> (bool, &'static str) {
        todo!()
    }

    /// TODO(Day2-Q4): `if 1 then true else false`
    /// hint: we do *not* have type system yet
    pub fn q3() -> (bool, &'static str) {
        todo!()
    }
}
