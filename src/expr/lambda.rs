use core::fmt;

use crate::Exp;

/// The actual representation of a lambda abstraction.
/// e.g., `\x. t` would be represents as `Lambda { arg: "x", exp: Exp }`
/// Note: This abstraction is probably the *most* important base for our stlc.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Lambda {
    pub arg: String,
    pub exp: Exp,
}

impl Lambda {
    pub fn new(arg: String, exp: Exp) -> Self {
        Self { arg, exp }
    }

    pub fn new_with_box(arg: String, exp: Exp) -> Box<Self> {
        Box::new(Self { arg, exp })
    }

    pub fn build(arg: String, exp: Exp) -> Exp {
        Self::new(arg, exp).into()
    }
}

impl From<Lambda> for Exp {
    fn from(value: Lambda) -> Self {
        Exp::Lambda(Box::new(value))
    }
}

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "λ{}. {}", self.arg, self.exp)
    }
}
