use crate::Exp;

pub struct Decr(Exp);
impl Decr {
    pub fn build(exp: Exp) -> Exp {
        Decr(exp).into()
    }
}

impl From<Decr> for Exp {
    fn from(value: Decr) -> Self {
        Exp::Decr(Box::new(value.0))
    }
}
