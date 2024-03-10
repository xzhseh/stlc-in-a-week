use std::io;

use stlc::{
    expr::{
        app::App, cond::Cond, decr::Decr, incr::Incr, is_zero::IsZero, lambda::Lambda, var::Var,
    },
    Exp,
};

fn read_line() -> String {
    let mut input = String::new();
    let Ok(_) = io::stdin().read_line(&mut input) else {
        panic!("failed to read line");
    };
    input.trim().into()
}

/// A very tiny parse utility to construct a speicified `Exp`
/// from command line.
/// Mainly used for testing and playing around with your stlc.
fn parse(name: &str) -> Exp {
    if name == "var" {
        println!("enter variable name below.");
        let input = read_line();
        return Var::build(input.as_str());
    }
    println!("what do you want to build for {}?", name);
    let input = read_line();
    match input.to_ascii_lowercase().as_str() {
        "var" => parse("var"),
        "app" => App::build(parse("app.t1"), parse("app.t2")),
        "cond" => Cond::build(parse("cond.if"), parse("cond.then"), parse("cond.else")),
        "incr" => Incr::build(parse("incr.e")),
        "decr" => Decr::build(parse("decr.e")),
        "lambda" => {
            println!("enter your lambda abstraction arg below.");
            let input = read_line();
            Lambda::build(input.as_str(), parse("lambda.e"))
        }
        "true" => Exp::True,
        "false" => Exp::False,
        "is_zero" => IsZero::build(parse("is_zero.e")),
        "nat" => {
            println!("enter a non-negative number below.");
            let num = read_line();
            let Ok(num) = num.parse::<u32>() else {
                panic!("failed to parse the given number {num}");
            };
            num.into()
        }
        _ => panic!("invalid input: {}", input),
    }
}

fn main() {
    println!("Congratulations, the program compiles.");

    loop {
        let exp = parse("begin");
        println!("\nfinish parsing, get expression: {}", exp);
        println!("start to evaluate expression to normal form by call-by-name.");
        let Ok(res) = exp.eval_to_normal_form(stlc::Strategy::CallByName) else {
            println!("failed to evaluate the input expression, this may be due to it's a stuck expression.");
            println!("end the loop.");
            return;
        };
        println!("finish evaluating, result: {}", res);
        println!("press ctrl-c to quit, or enter to continue.");
        let _ = read_line();
    }
}
