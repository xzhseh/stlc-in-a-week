use crate::Exp;

pub struct Nat(u32);
impl Nat {
    pub fn build(n: u32) -> Exp {
        n.into()
    }
}

impl From<u32> for Exp {
    fn from(value: u32) -> Self {
        Exp::Nat(value)
    }
}
