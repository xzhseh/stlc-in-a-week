use stlc::{
    day1_exercise::{Exp1, Exp2, Exp3, Exp4, Exp5},
    expr::{app::App, cond::Cond, lambda::Lambda},
    Exp,
};

#[test]
fn exp1_test() {
    let x = String::from("x");

    let exp1 = Exp::Lambda(Lambda::new_with_box(x.clone(), Exp::Var(x)));

    assert_eq!(Exp1::new(), exp1);
}

#[test]
fn exp2_test() {
    let x = String::from("x");

    let exp2 = Exp::Lambda(Lambda::new_with_box(
        x.clone(),
        Exp::Incr(Box::new(Exp::Var(x))),
    ));

    assert_eq!(Exp2::new(), exp2);
}

#[test]
fn exp3_test() {
    let x = String::from("x");
    let y = String::from("y");
    let z = String::from("z");

    let exp3 = Exp::App(App::new_with_box(
        // (\x. x)
        Exp::Lambda(Lambda::new_with_box(x.clone(), Exp::Var(x))),
        // ((\y. y) (\z. z))
        Exp::App(App::new_with_box(
            Exp::Lambda(Lambda::new_with_box(y.clone(), Exp::Var(y))),
            Exp::Lambda(Lambda::new_with_box(z.clone(), Exp::Var(z))),
        )),
    ));

    assert_eq!(Exp3::new(), exp3);
}

#[test]
fn exp4_test() {
    let x = String::from("x");
    let y = String::from("y");

    let exp4 = Exp::App(App::new_with_box(
        Exp::App(App::new_with_box(
            // (\x. x)
            Exp::Lambda(Lambda::new_with_box(x.clone(), Exp::Var(x))),
            // (\y. if y then false else true)
            Exp::Lambda(Lambda::new_with_box(
                y.clone(),
                Exp::Cond(Cond::new_with_box(Exp::Var(y), Exp::False, Exp::True)),
            )),
        )),
        Exp::True,
    ));

    assert_eq!(Exp4::new(), exp4);
}

#[test]
fn exp5_test() {
    let x = String::from("x");

    // (\x. x x)
    let exp = Exp::Lambda(Lambda::new_with_box(
        x.clone(),
        Exp::App(App::new_with_box(Exp::Var(x.clone()), Exp::Var(x.clone()))),
    ));

    let exp5 = Exp::App(App::new_with_box(exp.clone(), exp));

    assert_eq!(Exp5::new(), exp5);
}
