use std::collections::HashMap;

use crate::{
    type_::{tarrow::TArrow, TyConstraint, Type},
    Env, Exp, TySubst,
};

impl Type {
    /// TODO(Day7-Q1): implement the type substitution function
    /// to subsitute the current type based on the given tysubst.
    /// note: the subsitution should not be *recursive*.
    /// a simple example would be,
    /// e.g., `X0 -> X1`.apply_ty_subst([X1 ↦ Bool -> Bool, X0 ↦ Bool]) === Bool -> (Bool -> Bool)
    pub fn ref_apply_ty_subst(&mut self, sigma: &TySubst) {
        match self.clone() {
            Self::TVar(v) => {
                // lookup the sigma
                if let Some(t) = sigma.lookup(&v) {
                    *self = t;
                }
            }
            Self::TArrow(mut a) => {
                a.ty1.ref_apply_ty_subst(sigma);
                a.ty2.ref_apply_ty_subst(sigma);
                *self = TArrow::build(a.ty1, a.ty2);
            }
            // leave other intact
            _ => (),
        }
    }
}

impl TySubst {
    /// TODO(Day7-Q2): implement the type composing function
    /// to compose type substitution [γ1 ○ γ2]; which will later
    /// used by the unification algorithm.
    ///
    /// some examples are as follow.
    /// applyTySubst (composeTySubst [X0 ↦ Bool -> Bool] [X1 ↦ X0]) X0 ≡ Bool -> Bool
    /// applyTySubst (composeTySubst [X0 ↦ Bool -> Bool] [X1 ↦ X0]) X1 ≡ Bool -> Bool
    /// applyTySubst [X1 ↦ X0] X1 ≡ X0
    pub fn ref_compose(t1: TySubst, t2: TySubst) -> Self {
        let mut t2 = t2;
        let mut t1 = t1;
        let sigma = t2
            .inner_mut()
            .iter_mut()
            .map(|(k, v)| {
                v.ref_apply_ty_subst(&t1);
                (k.clone(), v.clone())
            })
            .collect::<HashMap<String, Type>>();
        for (k, v) in sigma {
            if !t1.contains(&k) {
                t1.insert(k, v);
            }
        }
        t1
    }
}

impl Type {
    /// TODO(Day7-Q3): implement the unification algorithm
    pub fn ref_unify(mut tc: Vec<TyConstraint>) -> Option<TySubst> {
        if tc.is_empty() {
            return Some(TySubst::new());
        }

        let c = tc.remove(0);
        let left = c.left();
        let right = c.right();

        if left == right {
            // keep on for the rest
            Self::ref_unify(tc)
        } else {
            match left {
                Self::TArrow(a1) => {
                    match right {
                        Self::TArrow(a2) => {
                            // S1 = T1
                            tc.push(TyConstraint::build(a1.ty1, a2.ty1));
                            // S2 = T2
                            tc.push(TyConstraint::build(a1.ty2, a2.ty2));
                            // unify(ξ' ∪ {S1 = T1, S2 = T2})
                            Self::ref_unify(tc)
                        }
                        _ => None,
                    }
                }
                // X = T
                Self::TVar(v) => {
                    if Self::in_type(v.clone(), right.clone()) {
                        // impossible to unify (why?)
                        return None;
                    } else {
                        // [X ↦ T] ξ' (i.e., tc)
                        let mut ts = TySubst::new();
                        ts.insert(v.clone(), right.clone());
                        Self::apply_ty_subst_on_tc(&mut tc, &ts);
                        // unify(tc)
                        let Some(unified) = Self::ref_unify(tc) else {
                            return None;
                        };
                        // unified ○ [X ↦ T]
                        Some(TySubst::ref_compose(unified, ts))
                    }
                }
                // S = X
                _ => {
                    let Self::TVar(v) = right else {
                        return None;
                    };

                    if Self::in_type(v.clone(), left.clone()) {
                        return None;
                    } else {
                        // [X ↦ S] ξ' (i.e., tc)
                        let mut ts = TySubst::new();
                        ts.insert(v.clone(), left.clone());
                        Self::apply_ty_subst_on_tc(&mut tc, &ts);
                        // unify(tc)
                        let Some(unified) = Self::ref_unify(tc) else {
                            return None;
                        };
                        // unified ○ [X ↦ S]
                        Some(TySubst::ref_compose(unified, ts))
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    fn in_type(v: String, t: Type) -> bool {
        match t {
            Self::TVar(v_1) => v == v_1,
            Self::TArrow(a) => Self::in_type(v.clone(), a.ty1) || Self::in_type(v, a.ty2),
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn apply_ty_subst_on_tc(tc: &mut Vec<TyConstraint>, ts: &TySubst) {
        for c in tc {
            let mut l = c.left();
            let mut r = c.right();
            l.ref_apply_ty_subst(&ts);
            r.ref_apply_ty_subst(&ts);
            *c = TyConstraint::build(l, r);
        }
    }
}

impl Exp {
    /// TODO(Day7-Q4): implement the contraint-based typing algorithm
    /// the initial exp should always be *untyped* - otherwise our
    /// bidirectional type check / inference can directly help with this.
    ///
    /// note: you need to put all the previous pieces together!
    pub fn ref_ty_infer_c(&self) -> Option<Type> {
        let mut e = self.clone();
        assert_eq!(
            e.ref_typed(),
            false,
            "expect initial expression to be untyped"
        );
        // first annotate the untyped lambda calculus expression
        let n = e.ref_annotate_term();
        // second infer the corresponding constraints
        let Some((_, mut t, tc)) = e.ref_infer_constraints(&mut Env::new(), n) else {
            return None;
        };
        // third unify the contraints - a.k.a. find the best unifier
        let Some(ts) = Type::ref_unify(tc.inner()) else {
            return None;
        };
        // fourth apply the subsitution on the type returned by `infer_constraints`
        t.ref_apply_ty_subst(&ts);
        // now we are done!
        Some(t)
    }
}
