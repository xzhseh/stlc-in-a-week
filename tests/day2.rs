use stlc::{expr::{app::App, cond::Cond, lambda::Lambda}, utils::{appears_free_in, is_value, substitute_expr}, Exp};

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