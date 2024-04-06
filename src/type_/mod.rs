use core::fmt;

use self::tarrow::TArrow;

pub mod tarrow;

/// the simple type(s) for our `Exp`
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    /// type variables
    TVar(String),

    /// arrow type, a.k.a. the function-like type
    TArrow(Box<TArrow>),

    /// yet a integer type
    TInt,

    /// yet a boolean type
    TBool,

    /// a dummy type - used when you want to prevent
    /// conflict in checking
    TDummy,
}

/// todo: add a macro to automatically implement all these
impl Type {
    pub fn is_var(&self) -> bool {
        if let Self::TVar(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_arrow(&self) -> bool {
        if let Self::TArrow(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_int(&self) -> bool {
        if let Self::TInt = self {
            true
        } else {
            false
        }
    }

    pub fn is_bool(&self) -> bool {
        if let Self::TBool = self {
            true
        } else {
            false
        }
    }
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        Self::TVar(value.into())
    }
}

impl From<String> for Type {
    fn from(value: String) -> Self {
        Self::TVar(value)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Self::TVar(v) => write!(f, "{}", v),
            Self::TArrow(t) => write!(f, "{}", *t),
            Self::TInt => write!(f, "int"),
            Self::TBool => write!(f, "bool"),
            Self::TDummy => write!(f, "dummy"),
        }
    }
}

/// type constraint is just a equation between `type`
/// e.g., X0 = X1, TInt = X2, TBool = TInt, X114514 = X1919810 -> TInt, etc.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TyConstraint(Type, Type);

impl TyConstraint {
    pub fn build(t1: Type, t2: Type) -> Self {
        Self(t1, t2)
    }

    pub fn left(&self) -> Type {
        self.0.clone()
    }

    pub fn right(&self) -> Type {
        self.1.clone()
    }
}

impl fmt::Display for TyConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} == {}", self.left(), self.right())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TyConstraints(Vec<TyConstraint>);

impl TyConstraints {
    pub fn build(v: Vec<TyConstraint>) -> Self {
        Self(v)
    }

    pub fn inner(self) -> Vec<TyConstraint> {
        self.0
    }

    pub fn merge(v: Vec<TyConstraints>) -> Self {
        let mut merged = vec![];
        for t in v {
            merged.extend(t.inner());
        }
        Self(merged)
    }

    pub fn empty() -> Self {
        Self::build(vec![])
    }
}

impl From<&[TyConstraint]> for TyConstraints {
    fn from(value: &[TyConstraint]) -> Self {
        Self(value.into_iter().map(|t| t.clone()).collect())
    }
}
