use crate::{
    expr::{
        add::Add, app::App, cond::Cond, decr::Decr, incr::Incr, is_zero::IsZero, lambda::Lambda,
    },
    Exp,
};

impl Exp {
    /// TODO(Day6-Q1): write a helper function that could
    /// automatically makes a *untyped* expression *typed*,
    /// by inserting type variables to the lambda abstraction.
    /// note: to make everyone's life easier, this function should
    /// only be called if the current exp is untyped.
    /// plus, the type variables should be named as something like,
    /// e.g., X0, X1, X2, ..., XN - check `tests/day6.rs` for more info.
    ///
    /// ps. a inner helper method that contains the on-the-fly number
    /// of your type variable may be of help.
    ///
    /// some examples are as follow.
    /// 1. annotateTerm' (λx. x) ≡ (λx: X0. x)
    /// 2. annotateTerm' (λx. λy. x y) ≡ (λx: X0. λy: X1: x y)
    /// 3. annotateTerm' ((λx. λy. x y) (λz. z)) ≡ (λx: X0. λy:X1: x y) (λz: X2. z)
    pub fn ref_annotate_term(&mut self) {
        assert_eq!(
            self.ref_typed(),
            false,
            "expect `annotate_term` to be called on an untyped exp"
        );
        self.ref_annotate_term_inner(0);
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
}
