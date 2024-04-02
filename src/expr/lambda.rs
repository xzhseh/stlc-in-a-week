use core::fmt;

use crate::{type_::Type, Exp};

/// The actual representation of a lambda abstraction.
/// e.g., `λx. t` would be represents as `Lambda { arg: "x", exp: Exp }`
/// Note: This abstraction is probably the *most* important base for our stlc.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Lambda {
    pub arg: String,
    pub exp: Exp,
    /// represents if the current `Exp`
    /// is typed or untyped.
    /// note: this *must be consistent*
    /// across the entire `Exp`.
    pub ty: Option<Type>,
}

impl Lambda {
    pub fn new(arg: String, exp: Exp) -> Self {
        Self { arg, exp, ty: None }
    }

    pub fn new_with_type(arg: String, exp: Exp, ty: Type) -> Self {
        Self {
            arg,
            exp,
            ty: Some(ty),
        }
    }

    pub fn new_with_box(arg: String, exp: Exp) -> Box<Self> {
        Box::new(Self { arg, exp, ty: None })
    }

    pub fn build(arg: &str, exp: Exp) -> Exp {
        Self::new(arg.into(), exp).into()
    }

    pub fn build_with_type(arg: &str, exp: Exp, ty: Type) -> Exp {
        Self::new_with_type(arg.into(), exp, ty).into()
    }

    /// whether the current lambda exp is typed or not.
    pub fn typed(&self) -> bool {
        self.ty.is_some()
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
