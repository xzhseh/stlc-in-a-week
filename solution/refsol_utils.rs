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
/// the corresponding substitution. `var` is the exactly variable
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