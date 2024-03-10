use crate::Exp;

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
    pub fn is_stuck(&self) -> bool {
        todo!()
    }
}

#[allow(dead_code)]
struct YCombinator(Exp);

impl YCombinator {
    /// TODO(Day4-Q4): Initlialize your definition of yCombinator here
    #[allow(dead_code)]
    pub fn new() -> Self {
        todo!()
    }
}
