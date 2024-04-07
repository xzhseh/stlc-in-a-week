use crate::{
    type_::{TyConstraints, Type},
    Env, Exp,
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
    /// 1. (λx. x).annotate_term() ≡ (λx: X0. x)
    /// 2. (λx. λy. x y).annotate_term() ≡ (λx: X0. λy: X1: x y)
    /// 3. ((λx. λy. x y) (λz. z)).annotate_term() ≡ (λx: X0. λy:X1: x y) (λz: X2. z)
    pub fn annotate_term(&mut self) {
        todo!()
    }

    #[allow(dead_code)]
    fn annotate_term_inner(&mut self, _n: u32) -> u32 {
        todo!()
    }

    /// TODO(Day6-Q2): The next thing is to implement a typing algorithm
    /// that takes a typing environment and a *stlc exp*
    /// and produces a type and a set of *type constraints* that must be
    /// satisfied in order for the term to be well-typed.
    /// as with the annotation function above,
    /// this algorithm will need to conjure fresh type variables.
    /// i.e., for which the `n` should exactly be used
    pub fn infer_constraints(&self, _env: &mut Env, _n: u32) -> Option<(u32, Type, TyConstraints)> {
        todo!()
    }
}
