use crate::{
    expr::{
        app::App, cond::Cond, decr::Decr, incr::Incr, is_zero::IsZero, lambda::Lambda, var::Var,
    },
    stlc_err::StlcError,
    Exp, Strategy,
};

type Result<T> = std::result::Result<T, StlcError>;

impl Exp {
    pub fn ref_grow_omega() -> ! {
        // (λx. x x x) (λ. x x x)
        let e = Lambda::build(
            "x",
            App::build(
                App::build(Var::build("x"), Var::build("x")),
                Var::build("x"),
            ),
        );
        let _grow_omega = App::build(e.clone(), e);
        loop {}
    }

    pub fn ref_is_stuck(&self, strategy: Strategy) -> bool {
        match strategy {
            Strategy::CallByValue => {
                let e = self.clone().eval_one_step_cbv().unwrap();
                e == self.clone()
            }
            Strategy::CallByName => {
                let e = self.clone().eval_one_step_cbn().unwrap();
                e == self.clone()
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct YCombinator {
    // The *driver* function for yCombinator
    y: Exp,
    // The *payload* function carried the "recursion" logic
    // must be *lambda abstraction*
    f: Exp,
}

impl YCombinator {
    pub fn ref_new(f: Exp) -> Self {
        // λF. (λx. F (x x)) (λx. F (x x))
        let e = Lambda::build(
            "x",
            App::build(
                Var::build("F"),
                App::build(Var::build("x"), Var::build("x")),
            ),
        );
        let y = Lambda::build("F", App::build(e.clone(), e));
        Self { y, f }
    }

    /// Yes, just a simple times function, e.g., 10 * 30 == 300
    /// this will be used as the payload function, i.e., f
    pub fn ref_gen_built_in_times() -> Exp {
        Lambda::build(
            "rec",
            Lambda::build(
                "x",
                Lambda::build(
                    "y",
                    Lambda::build(
                        "z",
                        Cond::build(
                            IsZero::build(Var::build("z")),
                            0.into(),
                            Cond::build(
                                IsZero::build(Var::build("y")),
                                App::build(
                                    App::build(
                                        App::build(Var::build("rec"), Var::build("x")),
                                        Var::build("x"),
                                    ),
                                    Decr::build(Var::build("z")),
                                ),
                                Incr::build(App::build(
                                    App::build(
                                        App::build(Var::build("rec"), Var::build("x")),
                                        Decr::build(Var::build("y")),
                                    ),
                                    Var::build("z"),
                                )),
                            ),
                        ),
                    ),
                ),
            ),
        )
    }

    /// eq 1 1 -> true;
    /// eq 2 3 -> false;
    pub fn ref_gen_built_in_equal() -> Exp {
        Lambda::build(
            "rec",
            Lambda::build(
                "x",
                Lambda::build(
                    "y",
                    Cond::build(
                        Cond::build(
                            IsZero::build(Var::build("x")),
                            Exp::True,
                            IsZero::build(Var::build("y")),
                        ),
                        Cond::build(
                            IsZero::build(Var::build("x")),
                            IsZero::build(Var::build("y")),
                            Exp::False,
                        ),
                        App::build(
                            App::build(Var::build("rec"), Decr::build(Var::build("x"))),
                            Decr::build(Var::build("y")),
                        ),
                    ),
                ),
            ),
        )
    }

    fn ref_build_eval_expr(&self, inputs: Vec<Exp>) -> Exp {
        let mut e = App::build(self.y.clone(), self.f.clone());
        for input in inputs {
            e = App::build(e, input);
        }
        e
    }

    pub fn ref_eval(self, inputs: Vec<Exp>, strategy: Strategy) -> Result<(Exp, u32)> {
        let e = self.ref_build_eval_expr(inputs);
        let (result, steps) = e.ref_eval_to_normal_form(strategy)?;
        Ok((result, steps))
    }
}
