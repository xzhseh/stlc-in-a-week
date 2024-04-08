use stlc::{
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
    let composed = TySubst::ref_compose(ts1, ts2);
    let mut t: Type = "X0".into();
    let res = TArrow::build(Type::TBool, Type::TBool);
    t.ref_apply_ty_subst(&composed);
    assert_eq!(t, res.clone());
    let mut t: Type = "X1".into();
    t.ref_apply_ty_subst(&composed);
    assert_eq!(t, res);
}