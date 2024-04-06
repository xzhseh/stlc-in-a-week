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
