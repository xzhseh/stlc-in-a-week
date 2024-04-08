use stlc::{
    expr::{add::Add, cond::Cond, lambda::Lambda, var::Var},
    type_::{tarrow::TArrow, TyConstraint, Type},
    TySubst,
};

#[test]
fn test_apply_ty_subst_basic() {
    let mut t = TArrow::build("X0".into(), "X1".into());
    let mut ts = TySubst::new();
    ts.insert("X1".into(), TArrow::build(Type::TBool, Type::TBool));
    ts.insert("X0".into(), Type::TBool);
    t.apply_ty_subst(&ts);
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
    let composed = TySubst::compose(ts1, ts2);
    let mut t: Type = "X0".into();
    let res = TArrow::build(Type::TBool, Type::TBool);
    t.apply_ty_subst(&composed);
    assert_eq!(t, res.clone());
    let mut t: Type = "X1".into();
    t.apply_ty_subst(&composed);
    assert_eq!(t, res);
}

#[test]
fn test_unify_basic_1() {
    // { X = Nat, Y = X -> X }
    let c = vec![
        TyConstraint::build("X".into(), Type::TInt),
        TyConstraint::build("Y".into(), TArrow::build("X".into(), "X".into())),
    ];
    let Some(ts) = Type::unify(c) else {
        panic!("expect `c` to be successfully unified");
    };
    // { X ↦ Nat, Y ↦ Nat -> Nat }
    let mut res = TySubst::new();
    res.insert("X".into(), Type::TInt);
    res.insert("Y".into(), TArrow::build(Type::TInt, Type::TInt));
    assert_eq!(res.len(), ts.len());
    for (k, v) in ts.inner() {
        assert!(res.contains(&k), "{k} must exist in the result tysubst");
        assert_eq!(v, res.lookup(&k).unwrap());
    }
}

#[test]
fn test_unify_basic_2() {
    // { X -> Y = Y -> Z, Z = U -> W }
    let c = vec![
        TyConstraint::build(
            TArrow::build("X".into(), "Y".into()),
            TArrow::build("Y".into(), "Z".into()),
        ),
        TyConstraint::build("Z".into(), TArrow::build("U".into(), "W".into())),
    ];
    let Some(ts) = Type::unify(c) else {
        panic!("expect `c` to be successfully unified");
    };
    // { X ↦ U -> W, Y ↦ U -> W, Z ↦ U -> W }
    let mut res = TySubst::new();
    let uw = TArrow::build("U".into(), "W".into());
    res.insert("X".into(), uw.clone());
    res.insert("Y".into(), uw.clone());
    res.insert("Z".into(), uw);
    for (k, v) in ts.inner() {
        assert!(res.contains(&k), "{k} must exist in the result tysubst");
        assert_eq!(v, res.lookup(&k).unwrap());
    }
}

#[test]
fn test_unify_basic_3() {
    // { Y ↦ Nat -> Y }
    let c = vec![TyConstraint::build(
        "Y".into(),
        TArrow::build(Type::TInt, "Y".into()),
    )];
    let res = Type::unify(c.clone());
    assert_eq!(
        res.is_none(),
        true,
        "{:#?} can not be unified due to its recursive nature",
        c
    );
}

#[test]
fn test_unify_basic_4() {
    // { Nat -> Nat = X -> Y }
    let c = vec![TyConstraint::build(
        TArrow::build(Type::TInt, Type::TInt),
        TArrow::build("X".into(), "Y".into()),
    )];
    let Some(ts) = Type::unify(c) else {
        panic!("expect `c` to be successfully unified");
    };
    // { X ↦ Nat, Y ↦ Nat }
    let mut res = TySubst::new();
    res.insert("X".into(), Type::TInt);
    res.insert("Y".into(), Type::TInt);
    assert_eq!(res.len(), ts.len());
    for (k, v) in ts.inner() {
        assert!(res.contains(&k), "{k} must exist in the result tysubst");
        assert_eq!(v, res.lookup(&k).unwrap());
    }
}

#[test]
fn test_ty_infer_c_basic_1() {
    // λx. if x then 1 + 3 else 5
    let e = Lambda::build(
        "x",
        Cond::build(Var::build("x"), Add::build(1.into(), 3.into()), 5.into()),
    );
    // try to infer the type of the untyped lambda calculus exp
    let Some(t) = e.ty_infer_c() else {
        panic!("expect `e` to be successfully inferred");
    };
    // (λx. if x then 1 + 3 else 5) : (Bool -> Int)
    let res = TArrow::build(Type::TBool, Type::TInt);
    assert_eq!(t, res);
}

#[test]
fn test_ty_infer_c_basic_2() {
    // λx. λy. x + y
    let e = Lambda::build(
        "x",
        Lambda::build("y", Add::build(Var::build("x"), Var::build("y"))),
    );
    let Some(t) = e.ty_infer_c() else {
        panic!("expect `e` to be successfully inferred");
    };
    // (λx. λy. x + y) : (Int -> (Int -> Int))
    let res = TArrow::build(Type::TInt, TArrow::build(Type::TInt, Type::TInt));
    assert_eq!(t, res);
}
