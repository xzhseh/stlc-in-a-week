use crate::Exp;

pub struct Incr(Exp);
impl Incr {
    pub fn build(exp: Exp) -> Exp {
        Incr(exp).into()
    }
}

impl From<Incr> for Exp {
    fn from(value: Incr) -> Self {
        Exp::Incr(Box::new(value.0))
    }
}
