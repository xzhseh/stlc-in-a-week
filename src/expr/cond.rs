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

    pub fn new_with_box(r#if: Exp, r#then: Exp, r#else: Exp) -> Box<Self> {
        Box::new(Self {
            r#if,
            r#then,
            r#else,
        })
    }
}
