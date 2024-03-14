use core::fmt;

use crate::Exp;

/// Treat this as a function application.
/// In general `t1` should be an *arrow* type (i.e., t1 :: a -> a),
/// which in our case is a lambda abstraction.
/// And of course, `t2` could literally be anything that is *valid*.
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
        Box::new(Self::new(t1, t2))
    }

    /// A constructor level *syntax sugar* just to make everything easier.
    /// i.e., instead of manually writing `Exp::App(Box::new(App::new(t1, t2)))`,
    /// we can directly write `App::build(t1, t2)` now.
    pub fn build(t1: Exp, t2: Exp) -> Exp {
        Self::new(t1, t2).into()
    }
}

impl From<App> for Exp {
    fn from(value: App) -> Self {
        Exp::App(Box::new(value))
    }
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) ({})", self.t1, self.t2)
    }
}
