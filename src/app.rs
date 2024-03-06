use crate::Exp;

/// Treat this as a function application
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct App {
    t1: Exp,
    t2: Exp,
}

impl App {
    pub fn new(t1: Exp, t2: Exp) -> Self {
        Self { t1, t2 }
    }
}
