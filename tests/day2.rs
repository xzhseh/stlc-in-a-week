use stlc::{
    expr::{app::App, cond::Cond, lambda::Lambda},
    Exp,
};

#[test]
fn test_appears_free_in_basic() {
    let x = String::from("x");
    let y = String::from("y");
    let z = String::from("z");

    let exp = basic_exp();

    assert_eq!(false, exp.appears_free_in(x));
    assert_eq!(false, exp.appears_free_in(y));
    assert_eq!(true, exp.appears_free_in(z));
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

    assert_eq!(true, exp1.is_value());
    assert_eq!(true, exp2.is_value());
    assert_eq!(true, exp3.is_value());
    assert_eq!(false, exp4.is_value());
}

#[test]
fn test_substitute_basic() {
    let x = String::from("x");
    let y = String::from("y");
    let z = String::from("z");

    // The dummy expression to be substituted
    let s = Exp::Nat(114514);

    // [x := s] x => s
    let exp1 = Exp::Var(x.clone());
    assert_eq!(exp1.substitute(x.clone(), s.clone()), s);

    // [x := s] y => y
    let exp2 = Exp::Var(y.clone());
    assert_eq!(exp2.clone().substitute(x.clone(), s.clone()), exp2);

    // [x := s] (\x. x) => (\x. x)
    let exp3 = Exp::Lambda(Box::new(Lambda::new(x.clone(), Exp::Var(x.clone()))));
    assert_eq!(exp3.clone().substitute(x.clone(), s.clone()), exp3);

    // [x := s] (\z. z x) => (\z. z s)
    let exp4 = Exp::Lambda(Box::new(Lambda::new(
        z.clone(),
        Exp::App(Box::new(App::new(Exp::Var(z.clone()), Exp::Var(x.clone())))),
    )));

    let result = Exp::Lambda(Box::new(Lambda::new(
        z.clone(),
        Exp::App(Box::new(App::new(Exp::Var(z.clone()), s.clone()))),
    )));

    assert_eq!(exp4.substitute(x.clone(), s.clone()), result);
}
