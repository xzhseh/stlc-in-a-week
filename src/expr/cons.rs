use crate::Exp;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cons {
    pub value: Exp,
    pub next: Exp,
}

impl Cons {
    pub fn new(value: Exp, next: Exp) -> Self {
        Self { value, next }
    }
}
