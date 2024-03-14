use crate::{stlc_err::StlcError, Exp, Strategy};

type Result<T> = std::result::Result<T, StlcError>;

impl Exp {
    /// TODO(Day3-Q1): Write a *helper* function that evaluate *one* step
    /// further using call-by-value evaluation strategy for the given expression
    /// ----
    /// Hint: whenever you stuck, consider review the three operational rules
    /// for call-by-value in the handout, and check if your implementation
    /// accurately follows the rules
    pub fn eval_one_step_cbv(self) -> Result<Exp> {
        todo!()
    }

    /// TODO(Day3-Q2): Same as what we have done for cbv,
    /// it's time to implement the same *helper* function for call-by-name!
    /// ----
    /// Hint: the operational rules are your best friends
    pub fn eval_one_step_cbn(self) -> Result<Exp> {
        todo!()
    }

    /// TODO(Day3-Q3): Write a "driver" function to evaluate the given expression
    /// exactly the given steps, so that we don't need to manually evaluate it step by step.
    /// Of course, you need to distinguish between different evaluation strategies.
    /// This would be *especially* useful when we are dealing with yCombinator later.
    pub fn eval_multi_step(self, _step: u32, _strategy: Strategy) -> Result<Exp> {
        todo!()
    }

    /// TODO(Day3-Q4'): Choose your favorite upper bound number of evalutation steps
    /// when evaluating the corresponding expression to its normal form.
    pub fn upper_bound(&self) -> u32 {
        114514
    }

    /// TODO(Day3-Q4): Write a function to help reduce the current expression
    /// to its normal form under the specified strategy.
    /// Note: instead of the expression, you should also return how many *steps*
    /// you have taken to evaluate the given expression to its normal form.
    pub fn eval_to_normal_form(self, _strategy: Strategy) -> Result<(Exp, u32)> {
        todo!()
    }
}
