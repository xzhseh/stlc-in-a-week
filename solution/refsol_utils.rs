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