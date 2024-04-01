use core::fmt;

use crate::Exp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Add {
    pub t1: Exp,
    pub t2: Exp,
}

impl Add {
    pub fn new(t1: Exp, t2: Exp) -> Self {
        Self { t1, t2 }
    }

    pub fn build(t1: Exp, t2: Exp) -> Exp {
        Self::new(t1, t2).into()
    }
}

impl From<Add> for Exp {
    fn from(value: Add) -> Self {
        Exp::Add(Box::new(value))
    }
}

impl fmt::Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) + ({})", self.t1, self.t2)
    }
}
