use stlc::{
    expr::{add::Add, app::App, lambda::Lambda, var::Var},
    type_::{tarrow::TArrow, Type},
};

#[test]
fn test_typed_basic() {
    // λx. λy. x + y
    let e = Lambda::build(
        "x",
        Lambda::build("y", Add::build(Var::build("x"), Var::build("y"))),
    );
    assert_eq!(e.typed(), false);

    // λx: TInt. λy: TInt. x + y
    let e = Lambda::build_with_type(
        "x",
        Lambda::build_with_type(
            "y",
            Add::build(Var::build("x"), Var::build("y")),
            Type::TInt,
        ),
        Type::TInt,
    );
    assert_eq!(e.typed(), true);
}

#[test]
fn test_ty_check_basic() {
    // λx: TInt. λy: TInt. x + y
    let e = Lambda::build_with_type(
        "x",
        Lambda::build_with_type(
            "y",
            Add::build(Var::build("x"), Var::build("y")),
            Type::TInt,
        ),
        Type::TInt,
    );

    // Γ ⊢ λx: TInt. λy. TInt. x + y : TInt -> (TInt -> TInt)
    let t = TArrow::build(Type::TInt, TArrow::build(Type::TInt, Type::TInt));

    assert_eq!(e.ty_check(t), true);
}

/// this test requires you've correctly [1] implemented `ty_infer`
///
/// [1] for some definition of correctly.
#[test]
fn test_ty_check_hard() {
    // (TInt -> (TInt -> TInt))
    let tx = TArrow::build(Type::TInt, TArrow::build(Type::TInt, Type::TInt));

    // (TInt -> TInt)
    let ty = TArrow::build(Type::TInt, Type::TInt);

    // TInt
    let tz = Type::TInt;

    // λx: (TInt -> (TInt -> TInt)). λy: (TInt -> TInt). λz: TInt. (x z) (y z)
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
                tz.clone(),
            ),
            ty.clone(),
        ),
        tx.clone(),
    );

    // (TInt -> (TInt -> TInt)) -> ((TInt -> TInt) -> (TInt -> TInt))
    let t = TArrow::build(tx, TArrow::build(ty, TArrow::build(tz, Type::TInt)));
    assert_eq!(e.ty_check(t), true);
}
