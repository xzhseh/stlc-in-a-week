use stlc::{app::App, lambda::Lambda, Exp};

fn main() {
    // Encode `(\x. x) (\y. y)` as an `Exp` enum
    let x = String::from("x");
    let y = String::from("y");
    let exp1 = Exp::App(Box::new(App::new(
        Exp::Lambda(Box::new(Lambda::new(x.clone(), Exp::Var(x)))),
        Exp::Lambda(Box::new(Lambda::new(y.clone(), Exp::Var(y)))),
    )));
    println!("exp1: {:#?}", exp1);
}
