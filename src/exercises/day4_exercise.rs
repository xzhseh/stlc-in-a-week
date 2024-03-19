use crate::{
    expr::{
        app::App, cond::Cond, decr::Decr, incr::Incr, is_zero::IsZero, lambda::Lambda, var::Var,
    },
    stlc_err::StlcError,
    Exp, Strategy,
};

type Result<T> = std::result::Result<T, StlcError>;

impl Exp {
    /// TODO(Day4-Q1): Write a function to observe different behavior when we
    /// evaluating expression with omega under two evaluation strategies.
    /// The current function signature indicates it will never return,
    /// but of course, feel free to change this.
    pub fn eval_omega() -> ! {
        todo!()
    }

    /// TODO(Day4-Q2): After knowing what a omega is, could you think of any expression
    /// that will *grow* larger after each evaluation step?
    /// Write the expression down and evaluate it here to prove your answer.
    pub fn grow_omega() -> ! {
        todo!()
    }

    /// TODO(Day4-Q3): Write a function to determine if the current expression gets stuck.
    /// Hint: a stuck expression is something that can not be evaluated further
    /// using any of the operational rules we have defined, and is also
    /// not a *value*. (yes, `is_value` should be of help)
    pub fn is_stuck(&self, _strategy: Strategy) -> bool {
        todo!()
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
    /// TODO(Day4-Q4): Initialize the driver function of yCombinator here
    pub fn new(f: Exp) -> Self {
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
    pub fn gen_built_in_times() -> Exp {
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

    /// TODO(Day4-Q5): Build your to-be-evaluated expression here
    fn build_eval_expr(&self, inputs: Vec<Exp>) -> Exp {
        let mut e = App::build(self.y.clone(), self.f.clone());
        for input in inputs {
            e = App::build(e, input);
        }
        e
    }

    /// TODO(Day4-Q6): Evaluate your packed yCombinator with the specific inputs to *normal form*
    pub fn eval(self, inputs: Vec<Exp>, strategy: Strategy) -> Result<(Exp, u32)> {
        let e = self.build_eval_expr(inputs);
        let (result, steps) = e.ref_eval_to_normal_form(strategy)?;
        Ok((result, steps))
    }
}
