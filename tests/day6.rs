use stlc::{
    expr::{app::App, lambda::Lambda, var::Var},
    type_::{tarrow::TArrow, Env, TyConstraint, TyConstraints},
};

#[test]
fn test_annotate_term_basic_1() {
    // λx. x - untyped
    let mut e = Lambda::build("x", Var::build("x"));
    let n = e.annotate_term();
    assert_eq!(n, 1);
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
    let n = e.annotate_term();
    assert_eq!(n, 2);
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
    let n = e.annotate_term();
    assert_eq!(n, 3);
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

#[test]
fn test_infer_constraints_basic_1() {
    // λx: X0. x x
    let e = Lambda::build_with_type(
        "x",
        App::build(Var::build("x"), Var::build("x")),
        "X0".into(),
    );
    let Some((n, t, c)) = e.infer_constraints(&mut Env::new(), 3) else {
        panic!("expect infer constraints to generate the result for {}", e);
    };
    assert_eq!(n, 4);
    assert_eq!(t, TArrow::build("X0".into(), "X3".into()));
    assert_eq!(
        c,
        TyConstraints::build(vec![TyConstraint::build(
            "X0".into(),
            TArrow::build("X0".into(), "X3".into())
        )])
    );
}

#[test]
fn test_infer_constraints_basic_2() {
    // λx: X0. λy: X1. λz: X2. (x z) (y z)
    let e = Lambda::build_with_type(
        "x",
        Lambda::build_with_type(
            "y",
            Lambda::build_with_type(
                "z",
                App::build(
                    App::build(Var::build("x"), Var::build("z")),
                    App::build(Var::build("y"), Var::build("z")),
                ),
                "X2".into(),
            ),
            "X1".into(),
        ),
        "X0".into(),
    );
    let Some((n, t, c)) = e.infer_constraints(&mut Env::new(), 3) else {
        panic!("expect infer constraints to generate the result for {}", e);
    };
    assert_eq!(n, 6);
    // X0 -> (X1 -> (X2 -> X5))
    let res = TArrow::build(
        "X0".into(),
        TArrow::build("X1".into(), TArrow::build("X2".into(), "X5".into())),
    );
    assert_eq!(t, res);
    // [X3 = (X4 -> X5), X0 = (X2 -> X3), X1 = (X2 -> X4)]
    let res = TyConstraints::build(vec![
        TyConstraint::build("X3".into(), TArrow::build("X4".into(), "X5".into())),
        TyConstraint::build("X0".into(), TArrow::build("X2".into(), "X3".into())),
        TyConstraint::build("X1".into(), TArrow::build("X2".into(), "X4".into())),
    ]);
    assert_eq!(res.len(), c.len(), "expect length to be the same");
    // do the bidirectional sanity check
    for t in res.inner_ref() {
        assert!(
            c.contains(t),
            "{:#?} is not in the result constraint vector `c`",
            t
        );
    }
    for t in c.inner_ref() {
        assert!(
            res.contains(t),
            "{:#?} is not in the provided constraint vector `res`",
            t
        );
    }
}
