use crate::Exp;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cond {
    r#if: Exp,
    r#then: Exp,
    r#else: Exp,
}
