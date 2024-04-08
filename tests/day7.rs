use stlc::{
    expr::{add::Add, cond::Cond, lambda::Lambda, var::Var},
    type_::{tarrow::TArrow, Type},
    TySubst,
};

#[test]
fn test_apply_ty_subst_basic() {
    let mut t = TArrow::build("X0".into(), "X1".into());
    let mut ts = TySubst::new();
    ts.insert("X1".into(), TArrow::build(Type::TBool, Type::TBool));
    ts.insert("X0".into(), Type::TBool);
    t.ref_apply_ty_subst(&ts);
    let res = TArrow::build(Type::TBool, TArrow::build(Type::TBool, Type::TBool));
    assert_eq!(t, res);
}

#[test]
fn test_ty_compose_basic() {
    // [X0 ↦ Bool -> Bool]
    let mut ts1 = TySubst::new();
    ts1.insert("X0".into(), TArrow::build(Type::TBool, Type::TBool));
    // [X1 ↦ X0]
    let mut ts2 = TySubst::new();
    ts2.insert("X1".into(), "X0".into());
    // should be: [X0 ↦ Bool -> Bool, X1 ↦ Bool -> Bool]
    let composed = TySubst::ref_compose(ts1, ts2);
    let mut t: Type = "X0".into();
    let res = TArrow::build(Type::TBool, Type::TBool);
    t.ref_apply_ty_subst(&composed);
    assert_eq!(t, res.clone());
    let mut t: Type = "X1".into();
    t.ref_apply_ty_subst(&composed);
    assert_eq!(t, res);
}

#[test]
fn test_ty_infer_c_basic_1() {
    // λx. if x then 1 + 3 else 5
    let e = Lambda::build(
        "x",
        Cond::build(Var::build("x"), Add::build(1.into(), 3.into()), 5.into()),
    );
    // try to infer the type of the untyped lambda calculus exp
    let Some(t) = e.ref_ty_infer_c() else {
        panic!("expect `e` to be successfully inferred");
    };
    // (λx. if x then 1 + 3 else 5) : (Bool -> Int)
    let res = TArrow::build(Type::TBool, Type::TInt);
    assert_eq!(t, res);
}
