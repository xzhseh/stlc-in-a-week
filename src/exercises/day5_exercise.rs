use crate::{
    type_::Type,
    Env, Exp,
};

impl Exp {
    /// TODO(Day5-Q1): check if the current expression
    /// is typed or not; if so return true, vice versa.
    pub fn typed(&self) -> bool {
        todo!()
    }

    /// the potentially useful [1] type inference function,
    /// you could use this when implementing `ty_check`.
    ///
    /// [1] of course, for some definition of useful.
    #[allow(dead_code)]
    fn ty_infer(&self, _context: &Env) -> Option<Type> {
        todo!()
    }

    /// TODO(Day5-Q2): implement the *bidirectional* type check function
    /// typically what it does is: for any given expression
    /// and the corresponding type, return true if
    /// the type checks, otherwise return false.
    ///
    /// note: you should also return false when the existing
    /// information / context is not enough to infer
    /// the corresponding types.
    ///
    /// an simple example would be, e.g.,
    /// ----
    /// (λx. 114514 + x).ty_check(int -> int) === true
    /// ----
    /// refer to the test cases for more detailed use cases.
    ///
    /// hint: a `ty_check_inner` helper function may be of help;
    /// since we need to start with an empty (mutable) context (i.e., Env).
    pub fn ty_check(&self, _ty: Type) -> bool {
        todo!()
    }
}
