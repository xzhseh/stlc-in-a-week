//! The utils part is to check your understanding of
//! our basic stlc at this moment.
//! Treat this as an exercise to get familiarize with lambda calculus
//! Feel free to add / change any test at the bottom
//! in order to fully test your implementation!
//! For reference solution, check `solution/utils.rs`.

use crate::Exp;

/// Day2-Q1: Write a function to check whether or not
/// the given variable is "free" in the provided expression.
/// To say a variable is free, basically we need to check if
/// it has been *bound* to some outer lambda abstraction(s).
/// e.g., In `\x. \y. x y z`, `x` is bound by the first lambda
/// abstraction, while `y` is bound by the second lambda abstraction
/// `z` in this case is *free*, so the following should apply, i.e.,
///  1. appears_free_in(`\x. \y. x y z`, `x`) => False
///  2. appears_free_in(`\x. \y. x y z`, `y`) => False
///  3. appears_free_in(`\x. \y. x y z`, `z`) => True
/// Note: We will assume that there are NOT any lamda abstractions in
/// its body that binds the same variable.
/// e.g., `\x. \y. x (\z. \x. z x) y` will not be included.
pub fn appears_free_in(_exp: Exp, _var: String) -> bool {
    todo!()
}

/// Day2-Q2: Write a function to check whether or not the input `Exp`
/// is a *value*, per the definition below.
///
/// v ::= \x. t         -- lambda abstraction
///       | true        -- constant true
///       | false       -- constant false
///       | n           -- natural number
pub fn is_value(_exp: Exp) -> bool {
    todo!()
}

/// Day2-Q3: Write a function that perform a *substitution* on the
/// `origin` expression. i.e., if any of the following 
/// substitution rules applies, reduce `origin` expression by
/// the corresponding substitution. `var` is the exactly variable
/// to be reduced.
/// 
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
///     [x := s] (if t1 then t2 else t3) = if [x := s] t1 then [x := s] t2 else [x := s] t3
pub fn substitute_expr(_var: String, _s: Exp, _origin: Exp) -> Exp {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{app::App, cond::Cond, lambda::Lambda};

    use super::*;

    #[test]
    fn test_appears_free_in_basic() {
        let x = String::from("x");
        let y = String::from("y");
        let z = String::from("z");

        let exp = basic_exp();

        assert_eq!(false, appears_free_in(exp.clone(), x));
        assert_eq!(false, appears_free_in(exp.clone(), y));
        assert_eq!(true, appears_free_in(exp.clone(), z));
    }

    fn basic_exp() -> Exp {
        let x = String::from("x");
        let y = String::from("y");
        let z = String::from("z");

        // The encoding of `\x. \y. x y z`
        Exp::Lambda(Box::new(Lambda::new(
            x.clone(),
            Exp::Lambda(Box::new(Lambda::new(
                y.clone(),
                Exp::App(Box::new(App::new(
                    Exp::Var(x),
                    Exp::App(Box::new(App::new(Exp::Var(y), Exp::Var(z)))),
                ))),
            ))),
        )))
    }

    #[test]
    fn test_is_value_basic() {
        let exp1 = Exp::Nat(114514);
        let exp2 = Exp::True;
        let exp3 = Exp::False;
        // Spoiler: `exp4` is ill-typed, since we haven't introduced type system yet, this is okay
        let exp4 = Exp::Cond(Box::new(Cond::new(
            exp1.clone(),
            exp2.clone(),
            exp3.clone(),
        )));

        assert_eq!(true, is_value(exp1));
        assert_eq!(true, is_value(exp2));
        assert_eq!(true, is_value(exp3));
        assert_eq!(false, is_value(exp4));
    }
}
