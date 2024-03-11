//! A minimal ipython-like interactive environment.
//! Mainly used for playing / testing / evaluating your stlc implementation.
//! I'll add more exercises related features in the future.
//! The ultimate goal is to achieve a minimal ghci-like interpreter.

use colored::*;
use std::io::{self, Write};

use crate::{
    expr::{
        app::App, cond::Cond, decr::Decr, incr::Incr, is_zero::IsZero, lambda::Lambda, var::Var,
    },
    Exp, Strategy,
};

fn print_help_msg() {
    println!(
        "\ncurrently supported syntax is as below. ({})",
        "case-insensitive".underline()
    );
    println!("{} v              -- variable", "var".green());
    println!("{} e           -- lambda abstraction", "lambda".green());
    println!("{} t1 t2          -- application", "app".green());
    println!("{} t1 t2 t3      -- condition", "cond".green());
    println!("{}               -- constant true", "true".green());
    println!("{}              -- constant false", "false".green());
    println!("{} n              -- non-negative number", "nat".green());
    println!("{} e          -- well, obviously", "is_zero".green());
    println!("{} e             -- increment", "incr".green());
    println!("{} e             -- decrement", "decr".green());
}

fn read_line() -> String {
    let mut input = String::new();
    let Ok(_) = io::stdin().read_line(&mut input) else {
        panic!("failed to read line");
    };
    input.trim().into()
}

fn print_prompt() {
    static mut IN_COUNTER: u32 = 1;
    unsafe {
        print!(
            "\n{}",
            format!("In [{}]: ", IN_COUNTER.to_string().bold()).green()
        );
        let _ = io::stdout().flush();
        IN_COUNTER += 1;
    }
}

/// A very tiny parse utility to construct a speicified `Exp`
/// from command line.
/// Mainly used for testing and playing around with your stlc.
fn parse(name: &str) -> Exp {
    if name == "var" {
        println!("\nenter {} below.", "variable name".green());
        print_prompt();
        let input = read_line();
        return Var::build(input.as_str());
    }
    println!(
        "\nwhat do you want to {} for {}?\n{}\n{}",
        "build".bright_blue(),
        name.green(),
        format!(
            "(type `{}` to display all available expr(s))",
            "help".green()
        ),
        format!(
            "(type `{}` to quit the interactive shell)",
            "ctrl-c".green()
        ),
    );
    print_prompt();
    let input = read_line();
    match input.to_ascii_lowercase().as_str() {
        "help" => {
            print_help_msg();
            parse(name)
        }
        "var" => parse("var"),
        "app" => App::build(parse("app.t1"), parse("app.t2")),
        "cond" => Cond::build(parse("cond.if"), parse("cond.then"), parse("cond.else")),
        "incr" => Incr::build(parse("incr.e")),
        "decr" => Decr::build(parse("decr.e")),
        "lambda" => {
            println!("\nenter your lambda abstraction {} below.", "arg".green());
            print_prompt();
            let input = read_line();
            Lambda::build(input.as_str(), parse("lambda.e"))
        }
        "true" => Exp::True,
        "false" => Exp::False,
        "is_zero" => IsZero::build(parse("is_zero.e")),
        "nat" => {
            println!("\nenter a {} below.", "non-negative number".green());
            print_prompt();
            let num = read_line();
            let Ok(num) = num.parse::<u32>() else {
                panic!(
                    "{}",
                    format!(
                        "failed to parse the given number {}",
                        num.to_string().underline()
                    )
                    .red()
                );
            };
            num.into()
        }
        // Endless loop until quit...
        _ => parse(name),
    }
}

pub fn start_interactive_shell() {
    println!("Congratulations, the program compiles.");

    loop {
        let exp = parse("begin");
        println!(
            "\n{}",
            format!(
                "your expression {} has been built.",
                exp.to_string().bold().underline().green()
            )
        );
        println!(
            "\nwhich {} would you select?\n{}",
            "evaluation strategy".bold(),
            format!(
                "currently available: {} (call-by-value), {} (call-by-name)",
                "cbv".underline().green(),
                "cbn".underline().green()
            )
        );
        let eval_strategy;
        loop {
            print_prompt();
            let input = read_line();
            match input.as_str() {
                "cbv" => {
                    eval_strategy = Strategy::CallByValue;
                    break;
                }
                "cbn" => {
                    eval_strategy = Strategy::CallByName;
                    break;
                }
                _ => {
                    println!(
                        "\n{} has not been supported, PR(s) welcome.",
                        input.red().underline()
                    );
                    println!("please choose an available strategy.");
                }
            }
        }
        println!(
            "\nstart evaluating expression to normal form by {}.",
            eval_strategy.to_string().green().underline().bold()
        );
        let result;
        match exp.clone().eval_to_normal_form(eval_strategy) {
            Ok(res) => result = res,
            Err(err) => {
                println!(
                    "failed to evaluate {}, error: {}",
                    exp.to_string().underline().red(),
                    err
                );
                continue;
            }
        }
        static mut OUT_COUNTER: u32 = 1;
        unsafe {
            println!(
                "\n{}",
                format!(
                    "OUT [{}]: {}",
                    OUT_COUNTER.to_string().bold(),
                    result.to_string().underline().bold()
                )
                .blue()
            );
            OUT_COUNTER += 1;
        }
        print!(
            "\npress `{}` to quit, or `{}` to continue.",
            "ctrl-c".green(),
            "enter".green()
        );
        let _ = io::stdout().flush();
        let _ = read_line();
    }
}
