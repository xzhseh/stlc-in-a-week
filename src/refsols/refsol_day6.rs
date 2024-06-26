use crate::{
    expr::{
        add::Add, app::App, cond::Cond, decr::Decr, incr::Incr, is_zero::IsZero, lambda::Lambda,
    },
    type_::{tarrow::TArrow, Env, TyConstraint, TyConstraints, Type},
    Exp,
};

impl Exp {
    pub fn ref_annotate_term(&mut self) -> u32 {
        assert_eq!(
            self.ref_typed(),
            false,
            "expect `annotate_term` to be called on an untyped exp"
        );
        self.ref_annotate_term_inner(0)
    }

    fn ref_annotate_term_inner(&mut self, n: u32) -> u32 {
        match self.clone() {
            Self::Lambda(mut lambda) => {
                let ret = lambda.exp.ref_annotate_term_inner(n + 1);
                *self = Lambda::build_with_type(&lambda.arg, lambda.exp, format!("X{n}").into());
                ret
            }
            Self::App(mut app) => {
                let ret = app.t1.ref_annotate_term_inner(n);
                let ret = app.t2.ref_annotate_term_inner(ret);
                *self = App::build(app.t1, app.t2);
                ret
            }
            Self::Add(mut add) => {
                let ret = add.t1.ref_annotate_term_inner(n);
                let ret = add.t2.ref_annotate_term_inner(ret);
                *self = Add::build(add.t1, add.t2);
                ret
            }
            Self::Cond(mut cond) => {
                let ret = cond.r#if.ref_annotate_term_inner(n);
                let ret = cond.r#then.ref_annotate_term_inner(ret);
                let ret = cond.r#else.ref_annotate_term_inner(ret);
                *self = Cond::build(cond.r#if, cond.r#then, cond.r#else);
                ret
            }
            Self::Incr(mut e) => {
                let ret = e.ref_annotate_term_inner(n);
                *self = Incr::build(*e);
                ret
            }
            Self::Decr(mut e) => {
                let ret = e.ref_annotate_term_inner(n);
                *self = Decr::build(*e);
                ret
            }
            Self::IsZero(mut e) => {
                let ret = e.ref_annotate_term_inner(n);
                *self = IsZero::build(*e);
                ret
            }
            // do nothing
            Self::Nat(_) | Self::True | Self::False | Self::Var(_) => n,
        }
    }

    pub fn ref_infer_constraints(
        &self,
        env: &mut Env,
        n: u32,
    ) -> Option<(u32, Type, TyConstraints)> {
        match self.clone() {
            // ct-true & ct-false
            Self::True | Self::False => Some((n, Type::TBool, TyConstraints::empty())),
            // ct-if
            Self::Cond(cond) => {
                let Some((n1, tc, c1)) = cond.r#if.ref_infer_constraints(env, n) else {
                    return None;
                };
                let Some((n2, tt, c2)) = cond.r#then.ref_infer_constraints(env, n1) else {
                    return None;
                };
                let Some((n3, te, c3)) = cond.r#else.ref_infer_constraints(env, n2) else {
                    return None;
                };
                let c = TyConstraints::build(vec![
                    TyConstraint::build(tc, Type::TBool),
                    TyConstraint::build(tt.clone(), te),
                ]);
                Some((n3, tt, TyConstraints::merge(vec![c1, c2, c3, c])))
            }
            // ct-abs
            Self::Lambda(lambda) => {
                let t1 = lambda.get_type_unchecked();
                env.insert(lambda.arg.clone(), t1.clone());
                let Some((n1, t2, c)) = lambda.exp.ref_infer_constraints(env, n) else {
                    return None;
                };
                env.remove(lambda.arg);
                Some((n1, TArrow::build(t1, t2), c))
            }
            // ct-app
            Self::App(app) => {
                let Some((n1, t1, c1)) = app.t1.ref_infer_constraints(env, n) else {
                    return None;
                };
                let Some((n2, t2, c2)) = app.t2.ref_infer_constraints(env, n1) else {
                    return None;
                };
                let x: Type = format!("X{n2}").into();
                let c = TyConstraints::build(vec![TyConstraint::build(
                    t1,
                    TArrow::build(t2, x.clone()),
                )]);
                Some((n2 + 1, x, TyConstraints::merge(vec![c1, c2, c])))
            }
            // ct-var
            Self::Var(v) => {
                let Some(t) = env.lookup(&v) else {
                    return None;
                };
                Some((n, t, TyConstraints::empty()))
            }
            // ct-num
            Self::Nat(_) => Some((n, Type::TInt, TyConstraints::empty())),
            // ct-add
            Self::Add(add) => {
                let Some((n1, t1, c1)) = add.t1.ref_infer_constraints(env, n) else {
                    return None;
                };
                let Some((n2, t2, c2)) = add.t2.ref_infer_constraints(env, n1) else {
                    return None;
                };
                let c = TyConstraints::build(vec![
                    TyConstraint::build(t1.clone(), Type::TInt),
                    TyConstraint::build(t2.clone(), Type::TInt),
                ]);
                Some((n2, Type::TInt, TyConstraints::merge(vec![c1, c2, c])))
            }
            _ => panic!("not yet supported: {}", self),
        }
    }
}
