use stlc::{
    exercises::day2_exercise::ValidExpressions, expr::{app::App, cond::Cond, decr::Decr, incr::Incr, lambda::Lambda, var::Var}, Exp
};

#[test]
fn test_appears_free_in_basic() {
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

    let exp = basic_exp();

    assert_eq!(false, exp.appears_free_in("x"));
    assert_eq!(false, exp.appears_free_in("y"));
    assert_eq!(true, exp.appears_free_in("z"));
}

#[test]
fn test_appears_free_in_hard() {
    // The encoding of `((\x. x) (\y. if ((\z. inc z) (inc h)) then (inc r) else (dec w))) 1`
    let exp = App::build(
        App::build(
            Lambda::build("x", Var::build("x")),
            Lambda::build(
                "y",
                Cond::build(
                    App::build(
                        Lambda::build("z", Incr::build(Var::build("z"))),
                        Incr::build(Var::build("h")),
                    ),
                    Incr::build(Var::build("r")),
                    Decr::build(Var::build("w")),
                ),
            ),
        ),
        1.into(),
    );

    assert_eq!(false, exp.appears_free_in("x"));
    // Note: though `y` is not "used", but it has been bound to
    // the lambda abstraction, i.e., `\y. if (...) then (...) else (...)`
    // so this will not be considered as "free variable"
    assert_eq!(false, exp.appears_free_in("y"));
    assert_eq!(false, exp.appears_free_in("z"));
    assert_eq!(true, exp.appears_free_in("h"));
    assert_eq!(true, exp.appears_free_in("r"));
    assert_eq!(true, exp.appears_free_in("w"));
}

#[test]
fn test_appears_free_in_corner() {
    // The encoding of `(\x. x) (\y. x y)`
    let exp = App::build(
        Lambda::build("x", Var::build("x")),
        Lambda::build("y", App::build(Var::build("x"), Var::build("y"))),
    );

    assert_eq!(true, exp.appears_free_in("x"));
    assert_eq!(false, exp.appears_free_in("y"));

    // The encoding of `(\x. x y) (\y. \z. if x then inc y else z)`
    let exp = App::build(
        Lambda::build("x", App::build(Var::build("x"), Var::build("y"))),
        Lambda::build(
            "y",
            Lambda::build(
                "z",
                Cond::build(
                    Var::build("x"),
                    Incr::build(Var::build("y")),
                    Var::build("z"),
                ),
            ),
        ),
    );

    assert_eq!(true, exp.appears_free_in("x"));
    assert_eq!(true, exp.appears_free_in("y"));
    assert_eq!(false, exp.appears_free_in("z"));
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

#[test]
fn test_valid_expression_1() {
    assert_eq!(ValidExpressions::q1(), (false, ""));
}

#[test]
fn test_valid_expression_2() {
    assert_eq!(ValidExpressions::q2(), (false, ""));
}

#[test]
fn test_valid_expression_3() {
    assert_eq!(ValidExpressions::q3(), (false, ""));
}
