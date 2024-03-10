use crate::Exp;

pub struct Var(String);
impl Var {
    pub fn build(var: &str) -> Exp {
        Var(var.into()).into()
    }
}

impl From<Var> for Exp {
    fn from(value: Var) -> Self {
        Exp::Var(value.0)
    }
}
