use crate::{type_::Type, Env, Exp};

impl Exp {
    /// TODO(Day5-Q1): check if the current expression
    /// is typed or not; if so return true, vice versa.
    pub fn typed(&self) -> bool {
        fn validate(lhs: bool, rhs: bool) -> bool {
            assert_eq!(
                lhs, rhs,
                "expect type to be consistent across the entire term"
            );
            lhs && rhs
        }

        match self.clone() {
            // typically we only need to care about lambda abstraction
            // - which is the only "typable" term in the context of stlc.
            Self::Lambda(lambda) => lambda.typed(),
            Self::Add(add) => validate(add.t1.typed(), add.t2.typed()),
            Self::App(app) => validate(app.t1.typed(), app.t2.typed()),
            Self::Cond(cond) => validate(
                cond.r#if.typed(),
                validate(cond.r#then.typed(), cond.r#else.typed()),
            ),
            Self::Incr(e) | Self::Decr(e) | Self::IsZero(e) => e.typed(),
            // for the constant / variable terms, simply return true.
            Self::Var(_) | Self::Nat(_) | Self::True | Self::False => true,
        }
    }

    /// the potentially useful [1] type inference function,
    /// you could use this when implementing `ty_check`.
    /// [1] for some definition of useful.
    #[allow(dead_code)]
    fn ty_infer_(&self, _context: &Env) -> Type {
        todo!()
    }

    /// TODO(Day5-Q2): implement the type check function
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
    ///
    /// hint: a `ty_check_inner` helper function may be of help
    /// - since we need to start with an empty context (i.e., Env).
    pub fn ty_check(&self, ty: Type) -> bool {
        self.ty_check_inner(ty, Env::new())
    }

    fn ty_check_inner(&self, ty: Type, context: Env) -> bool {
        todo!()
    }
}
