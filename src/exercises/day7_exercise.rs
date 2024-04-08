use std::collections::HashMap;

use crate::{
    type_::{tarrow::TArrow, Type},
    TySubst,
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
