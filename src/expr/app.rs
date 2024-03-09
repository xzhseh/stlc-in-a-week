use crate::Exp;

/// Treat this as a function application
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct App {
    pub t1: Exp,
    pub t2: Exp,
}

impl App {
    pub fn new(t1: Exp, t2: Exp) -> Self {
        Self { t1, t2 }
    }

    pub fn new_with_box(t1: Exp, t2: Exp) -> Box<Self> {
        Box::new(Self { t1, t2 })
    }
}
