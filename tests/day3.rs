use stlc::{
    expr::{app::App, lambda::Lambda},
    Exp, Strategy,
};

#[test]
fn test_eval_one_step_basic() {
    // (\x. inc x) 1 -> 2
    let exp1 = Exp::App(Box::new(App::new(
        Exp::Lambda(Box::new(Lambda::new(
            "x".to_string(),
            Exp::Incr(Box::new(Exp::Var("x".to_string()))),
        ))),
        Exp::Nat(1),
    )));

    // CBV
    let first_step_cbv = exp1.clone().eval_one_step_cbv().unwrap();
    assert_eq!(first_step_cbv, Exp::Incr(Box::new(Exp::Nat(1))));

    let second_step_cbv = first_step_cbv.eval_one_step_cbv().unwrap();
    assert_eq!(second_step_cbv, Exp::Nat(2));

    // CBN should produce the exact same step
    let first_step_cbn = exp1.eval_one_step_cbn().unwrap();
    assert_eq!(first_step_cbn, Exp::Incr(Box::new(Exp::Nat(1))));

    let second_step_cbn = first_step_cbn.eval_one_step_cbn().unwrap();
    assert_eq!(second_step_cbn, Exp::Nat(2));
}

#[test]
fn test_eval_multi_step_basic() {
    // (\x. inc x) 1 -> 2
    let exp1 = Exp::App(Box::new(App::new(
        Exp::Lambda(Box::new(Lambda::new(
            "x".to_string(),
            Exp::Incr(Box::new(Exp::Var("x".to_string()))),
        ))),
        Exp::Nat(1),
    )));

    // CBV
    assert_eq!(
        exp1.clone()
            .eval_multi_step(2, Strategy::CallByValue)
            .unwrap(),
        Exp::Nat(2)
    );
    // CBN
    assert_eq!(
        exp1.eval_multi_step(2, Strategy::CallByName).unwrap(),
        Exp::Nat(2)
    );

    // (\x. \y. inc y)
    let exp1 = Exp::Lambda(Box::new(Lambda::new(
        "x".to_string(),
        Exp::Lambda(Box::new(Lambda::new(
            "y".to_string(),
            Exp::Incr(Box::new(Exp::Var("y".to_string()))),
        ))),
    )));
    // ω := (\x. x x) (\x. x x)
    let omega = Exp::App(Box::new(App::new(
        Exp::Lambda(Box::new(Lambda::new(
            "x".to_string(),
            Exp::App(Box::new(App::new(
                Exp::Var("x".to_string()),
                Exp::Var("x".to_string()),
            ))),
        ))),
        Exp::Lambda(Box::new(Lambda::new(
            "x".to_string(),
            Exp::App(Box::new(App::new(
                Exp::Var("x".to_string()),
                Exp::Var("x".to_string()),
            ))),
        ))),
    )));
    let nat1 = Exp::Nat(1);

    // (\x. \y. inc y) ω 1 -> 2
    let exp = Exp::App(Box::new(App::new(
        Exp::App(Box::new(App::new(exp1, omega))),
        nat1,
    )));

    // Call-By-Name
    assert_eq!(
        exp.eval_multi_step(3, Strategy::CallByName).unwrap(),
        Exp::Nat(2)
    );

    // Try to use `eval_multi_step` with Call-By-Value strategy here and observe the result.
    //                            cbv
    // i.e., (\x. \y. inc y) ω 1 -----> ???
    // ----
    // Q1: Is the result conforming to your expectation?
    // Q2: Is it different from Call-By-Name strategy?
    // If your answer to Q2 is yes, then why is it different?
    // ----
    // P.S. If you don't know the answer yet, just put a pin here.
    // Hopefully you could answer this question after day 4.
}
