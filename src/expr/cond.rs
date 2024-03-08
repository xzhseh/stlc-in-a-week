use crate::Exp;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cond {
    pub r#if: Exp,
    pub r#then: Exp,
    pub r#else: Exp,
}

impl Cond {
    pub fn new(r#if: Exp, r#then: Exp, r#else: Exp) -> Self {
        Self {
            r#if,
            r#then,
            r#else,
        }
    }
}
