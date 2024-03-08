//! The utils part is to check your understanding of
//! our basic stlc at this moment.
//! Treat this as an exercise to get familiarize with lambda calculus (and of course, Rust itself)
//! Feel free to add / change any test at the bottom in order to fully test your implementation!
//! The functions to implement may or may not be used in the future
//! For reference solution, check `solution/refsol_utils.rs`.

use crate::{
    expr::{app::App, cond::Cond, lambda::Lambda},
    Exp,
};

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
pub fn appears_free_in(exp: Exp, var: String) -> bool {
    /// Helper function to check if the input *variable* appears
    /// in the given expression
    fn check_var_appear(exp: Exp, var: String) -> bool {
        match exp {
            Exp::Var(v) => var == v,
            Exp::Lambda(lambda) => check_var_appear(lambda.exp, var),
            Exp::App(app) => check_var_appear(app.t1, var.clone()) || check_var_appear(app.t2, var),
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

    !(check_var_appear(exp.clone(), var.clone()) && check_lambda_appear(exp, var))
}

/// Day2-Q2: Write a function to check whether or not the input `Exp`
/// is a *value*, per the definition below.
///
/// v ::= \x. t         -- lambda abstraction
///       | true        -- constant true
///       | false       -- constant false
///       | n           -- natural number
pub fn is_value(exp: Exp) -> bool {
    match exp {
        Exp::Lambda(_) | Exp::True | Exp::False | Exp::Nat(_) => true,
        _ => false,
    }
}

/// Day2-Q3: Write a function that perform a *substitution* on the
/// `origin` expression. i.e., if any of the following
/// substitution rules applies, reduce `origin` expression by
/// the corresponding substitution. `var` is the exact variable
/// to be reduced.
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
pub fn substitute_expr(var: String, s: Exp, origin: Exp) -> Exp {
    match origin.clone() {
        // [x := s] x && [x := s] y
        Exp::Var(v) => {
            if v == var {
                s
            } else {
                origin
            }
        }
        // [x := s] (\x. t) && [x := s] (\y. t)
        Exp::Lambda(lambda) => {
            if lambda.arg == var {
                origin
            } else {
                Exp::Lambda(Box::new(Lambda::new(
                    lambda.arg,
                    substitute_expr(var, s, lambda.exp),
                )))
            }
        }
        // [x := s] (t1 t2)
        Exp::App(app) => Exp::App(Box::new(App::new(
            substitute_expr(var.clone(), s.clone(), app.t1),
            substitute_expr(var, s, app.t2),
        ))),
        // [x := s] (if t1 then t2 else t3)
        Exp::Cond(cond) => Exp::Cond(Box::new(Cond::new(
            substitute_expr(var.clone(), s.clone(), cond.r#if),
            substitute_expr(var.clone(), s.clone(), cond.r#then),
            substitute_expr(var, s, cond.r#else),
        ))),
        // [x := s] (inc t)
        Exp::Incr(e) => Exp::Incr(Box::new(substitute_expr(var, s, *e))),
        // [x := s] (dec t)
        Exp::Decr(e) => Exp::Decr(Box::new(substitute_expr(var, s, *e))),
        // [x := s] true && [x := s] false && [x := s] n
        e => e,
    }
}

#[cfg(test)]
mod tests {
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

        // Spoiler: `exp4` is actually ill-typed, since we haven't introduced type system yet, this is okay
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

    #[test]
    fn test_substitute_expr_basic() {
        let x = String::from("x");
        let y = String::from("y");
        let z = String::from("z");

        // The dummy expression to be substituted
        let s = Exp::Nat(114514);

        // [x := s] x => s
        let exp1 = Exp::Var(x.clone());
        assert_eq!(substitute_expr(x.clone(), s.clone(), exp1), s);

        // [x := s] y => y
        let exp2 = Exp::Var(y.clone());
        assert_eq!(substitute_expr(x.clone(), s.clone(), exp2.clone()), exp2);

        // [x := s] (\x. x) => (\x. x)
        let exp3 = Exp::Lambda(Box::new(Lambda::new(x.clone(), Exp::Var(x.clone()))));
        assert_eq!(substitute_expr(x.clone(), s.clone(), exp3.clone()), exp3);

        // [x := s] (\z. z x) => (\z. z s)
        let exp4 = Exp::Lambda(Box::new(Lambda::new(
            z.clone(),
            Exp::App(Box::new(App::new(Exp::Var(z.clone()), Exp::Var(x.clone())))),
        )));

        let result = Exp::Lambda(Box::new(Lambda::new(
            z.clone(),
            Exp::App(Box::new(App::new(Exp::Var(z.clone()), s.clone()))),
        )));

        assert_eq!(substitute_expr(x.clone(), s.clone(), exp4), result);
    }
}
