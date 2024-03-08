use crate::Exp;

/// The actual representation of a lambda abstraction
/// e.g., `\x. t` would be represents as `Lambda { arg: "x", exp: Exp }`
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Lambda {
    pub arg: String,
    pub exp: Exp,
}

impl Lambda {
    pub fn new(arg: String, exp: Exp) -> Self {
        Self { arg, exp }
    }
}
