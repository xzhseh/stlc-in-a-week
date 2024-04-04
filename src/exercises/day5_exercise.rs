use crate::{
    type_::{tarrow::TArrow, Type},
    Env, Exp,
};

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
    ///
    /// [1] of course, for some definition of useful.
    #[allow(dead_code)]
    fn ty_infer(&self, context: &Env) -> Option<Type> {
        // before actually beginning the inference, try think two questions first:
        // 1. what kind of exp could be presumably inferred *based on the context*?
        // 2. and in what case will `ty_infer` be invoked?
        match self.clone() {
            Self::True | Self::False | Self::IsZero(_) => Some(Type::TBool),
            Self::Incr(_) | Self::Decr(_) | Self::Add(_) | Self::Nat(_) => Some(Type::TInt),
            Self::Cond(cond) => {
                let mut context_1 = context.clone();
                // if clause should be boolean type
                if !cond.r#if.ty_check_inner(Type::TBool, &mut context_1) {
                    return None;
                }
                let Some(t1) = cond.r#then.ty_infer(context) else {
                    return None;
                };
                let Some(t2) = cond.r#else.ty_infer(context) else {
                    return None;
                };
                if t1 != t2 {
                    return None;
                }
                Some(t1)
            }
            Self::Var(v) => context.lookup(&v),
            Self::Lambda(lambda) => {
                // note: we assume `lambda` is typed here -
                // to make everyone's life easier...
                assert_eq!(
                    lambda.typed(),
                    true,
                    "expect `lambda` to be typed in `ty_infer`"
                );
                let Some(t) = lambda.exp.ty_infer(context) else {
                    return None;
                };
                // t1 -> t2
                Some(TArrow::build(lambda.get_type_unchecked(), t))
            }
            // likely to be the most common case to infer
            Self::App(app) => {
                let Some(t1) = app.t1.ty_infer(context) else {
                    return None;
                };

                // type of e1 should be arrow type - otherwise it
                // does not make any sense for an application
                let Type::TArrow(t) = t1 else {
                    return None;
                };

                // do the type check for e2
                let mut context = context.clone();
                if !app.t2.ty_check_inner(t.ty1, &mut context) {
                    return None;
                }

                // good, now we are done!
                Some(t.ty2)
            }
        }
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
    /// refer to the test cases for more detailed use cases.
    ///
    /// hint: a `ty_check_inner` helper function may be of help;
    /// since we need to start with an empty (mutable) context (i.e., Env).
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
                // subsequent type check should *not* be affected
                // e.g., Γ ⊢ (λx: TInt. x + 1) + (λy: TInt. y + 1) : TInt
                // when type check the second term (i.e., λy),
                // the context with [x -> TInt] should not be visible.
                context.remove(lambda.arg.clone());
                first && second
            }
            // t-app - a.k.a. the "fancy" type inference goes here
            Self::App(app) => {
                // here is where things get excited
                // the information available: Γ ⊢ e1 e2: T

                match app.t1.clone() {
                    // 1. if `e1` is arrow type, i.e., lambda abstraction
                    Self::Lambda(lambda) => {
                        assert_eq!(lambda.typed(), true, "expect `lambda` to be typed");
                        let Type::TArrow(t) = ty else {
                            return false;
                        };
                        // we now have *enough* information to type check `e2`
                        let first = app.t2.ty_check_inner(t.ty1, context);
                        // short circuit
                        if !first {
                            return false;
                        }
                        // if `e2` type checks, now we can check the inner expression of lambda
                        let second = lambda.exp.ty_check_inner(t.ty2, context);
                        first && second
                    }
                    // 2. if `e1` is a variable.
                    Self::Var(v) => {
                        // check the current context
                        match context.lookup(&v) {
                            Some(t) => {
                                // we could only accept arrow type here
                                // otherwise the type will simply not check
                                let Type::TArrow(t) = t else {
                                    return false;
                                };
                                if t.ty2 != ty {
                                    return false;
                                }
                                app.t2.ty_check_inner(t.ty1, context)
                            }
                            None => false,
                        }
                    }
                    // 3. now we arrives at a situation where no *explicit* type information
                    //    is enough to conduct the type check.
                    //    what should we do then?
                    _ => {
                        // basically the essential problem here, is that the type of `e1`
                        // is unclear, though we *may* have enough (type) information
                        // to type check the term.
                        // so let's infer it based on the context.
                        let Some(t) = app.t1.ty_infer(&context) else {
                            return false;
                        };

                        // once we successfully get the type by inference,
                        // we can then try to type check the rest.
                        let Type::TArrow(t) = t else {
                            return false;
                        };
                        if t.ty2 != ty {
                            return false;
                        }
                        app.t2.ty_check_inner(t.ty1, context)
                    }
                }
            }
            // t-var
            Self::Var(v) => context.lookup(&v).unwrap_or(Type::TDummy) == ty,
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
