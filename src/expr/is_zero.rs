use crate::Exp;

pub struct IsZero(Exp);
impl IsZero {
    pub fn build(exp: Exp) -> Exp {
        IsZero(exp).into()
    }
}

impl From<IsZero> for Exp {
    fn from(value: IsZero) -> Self {
        Exp::IsZero(Box::new(value.0))
    }
}
