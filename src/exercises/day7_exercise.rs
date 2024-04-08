use crate::{
    type_::{TyConstraint, TySubst, Type},
    Exp,
};

impl Type {
    /// TODO(Day7-Q1): implement the type substitution function
    /// to subsitute the current type based on the given tysubst.
    /// note: the subsitution should not be *recursive*.
    /// a simple example would be,
    /// e.g., `X0 -> X1`.apply_ty_subst([X1 ↦ Bool -> Bool, X0 ↦ Bool]) === Bool -> (Bool -> Bool)
    pub fn apply_ty_subst(&mut self, _sigma: &TySubst) {
        todo!()
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
    pub fn compose(_t1: TySubst, _t2: TySubst) -> Self {
        todo!()
    }
}

impl Type {
    /// TODO(Day7-Q3): implement the unification algorithm
    /// according to the algorithm specified in handout.
    pub fn unify(_tc: Vec<TyConstraint>) -> Option<TySubst> {
        todo!()
    }
}

impl Exp {
    /// TODO(Day7-Q4): implement the contraint-based typing algorithm
    /// the initial exp should always be *untyped* - otherwise our
    /// bidirectional type check / inference can directly help with this.
    ///
    /// note: you need to put all the previous pieces together!
    pub fn ty_infer_c(&self) -> Option<Type> {
        todo!()
    }
}
