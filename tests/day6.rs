use stlc::expr::{app::App, lambda::Lambda, var::Var};

#[test]
fn test_annotate_term_basic_1() {
    // λx. x - untyped
    let mut e = Lambda::build("x", Var::build("x"));
    e.ref_annotate_term();
    let res = Lambda::build_with_type("x", Var::build("x"), "X0".into());
    assert_eq!(e, res);
}

#[test]
fn test_annotate_term_basic_2() {
    // λx. λy. x y
    let mut e = Lambda::build(
        "x",
        Lambda::build("y", App::build(Var::build("x"), Var::build("y"))),
    );
    e.ref_annotate_term();
    let res = Lambda::build_with_type(
        "x",
        Lambda::build_with_type(
            "y",
            App::build(Var::build("x"), Var::build("y")),
            "X1".into(),
        ),
        "X0".into(),
    );
    assert_eq!(e, res);
}

#[test]
fn test_annotate_term_basic_3() {
    // (λx. λy. x y) (λz. z)
    let mut e = App::build(
        Lambda::build(
            "x",
            Lambda::build("y", App::build(Var::build("x"), Var::build("y"))),
        ),
        Lambda::build("z", Var::build("z")),
    );
    e.ref_annotate_term();
    let res = App::build(
        Lambda::build_with_type(
            "x",
            Lambda::build_with_type(
                "y",
                App::build(Var::build("x"), Var::build("y")),
                "X1".into(),
            ),
            "X0".into(),
        ),
        Lambda::build_with_type("z", Var::build("z"), "X2".into()),
    );
    assert_eq!(e, res);
}
