use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StlcError {
    /// Indicating the current expression is impossible
    /// to evaluate further under current operational rules
    /// in Call-By-Value strategy.
    /// e.g., `(λx. x) + 1` is a stuck expression. (why?)
    StuckExpressionCbv(String),

    /// Same as above, but under Call-By-Name's context.
    /// Note that `(λx. λy. inc y) ω 1` is not a stuck expression for cbn
    /// but it will eventually get stuck for cbv.
    /// No worries if this does not make sense at present, you will learn
    /// what's omega (i.e., ω) in day-4.
    StuckExpressionCbn(String),

    /// An invalid expression is literally something we can't find
    /// a pattern to match with.
    InvalidExpression(String),

    /// Indicating the current evaluation steps exceed the preset
    /// limit. This may or may not due to the expression itself
    /// can't be *reduced* to normal form, or it's just under
    /// our operational semantics this will take a huge amount
    /// of steps to reduce the input expression to its normal form.
    ExceedEvalLimit(String),
    // TODO: add more custom errors to fit your need(s)!
}

impl fmt::Display for StlcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StlcError::StuckExpressionCbv(err) => write!(f, "StuckExpressionCbv({})", err),
            StlcError::StuckExpressionCbn(err) => write!(f, "StuckExpressionCbn({})", err),
            StlcError::InvalidExpression(err) => write!(f, "InvalidExpression({})", err),
            StlcError::ExceedEvalLimit(err) => write!(f, "ExceedEvalLimit({})", err),
        }
    }
}
