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
        self.ty_check_inner(ty, &mut Env::new())
    }

    fn ty_check_inner(&self, ty: Type, context: &mut Env) -> bool {
        match self.clone() {
            // t-true & t-false
            Self::True | Self::False => ty.is_bool(),
            // t-if
            Self::Cond(cond) => {
                cond.r#if.ty_check_inner(Type::TBool, context)
                    && cond.r#then.ty_check_inner(ty.clone(), context)
                    && cond.r#else.ty_check_inner(ty, context)
            }
            // t-abs
            Self::Lambda(lambda) => {
                // sanity check
                assert_eq!(lambda.typed(), true, "expect `lambda` to be typed");
                let Type::TArrow(t) = ty else {
                    return false;
                };
                let first = lambda.get_type_unchecked() == t.ty1;
                // short circuit check
                if !first {
                    return false;
                }
                context.insert(lambda.arg.clone(), t.ty1);
                let second = lambda.exp.ty_check_inner(t.ty2, context);
                // subsequent type check should not be affected
                context.remove(lambda.arg.clone());
                first && second
            }
            // t-app - a.k.a. the "fancy" type inference goes here
            Self::App(_app) => todo!(),
            // t-var
            Self::Var(v) => context.lookup(v).unwrap_or(Type::TDummy) == ty,
            // t-num
            Self::Nat(_) => ty.is_int(),
            // t-add
            Self::Add(add) => {
                if !ty.is_int() {
                    return false;
                }
                add.t1.ty_check_inner(Type::TInt, context)
                    && add.t2.ty_check_inner(Type::TInt, context)
            }
            // every other will return false
            _ => false,
        }
    }
}
