use core::fmt;

use super::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TArrow {
    pub ty1: Type,
    pub ty2: Type,
}

impl TArrow {
    pub fn new(ty1: Type, ty2: Type) -> TArrow {
        Self { ty1, ty2 }
    }

    pub fn build(ty1: Type, ty2: Type) -> Type {
        Self::new(ty1, ty2).into()
    }
}

impl From<TArrow> for Type {
    fn from(value: TArrow) -> Self {
        Self::TArrow(Box::new(value))
    }
}

impl fmt::Display for TArrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.ty1, self.ty2)
    }
}
