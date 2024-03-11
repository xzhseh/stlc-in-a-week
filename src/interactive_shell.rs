//! A minimal ipython-like interactive environment.
//! Mainly used for playing / testing / evaluating your stlc implementation.
//! I'll add more exercises related features in the future.
//! The ultimate goal is to achieve a minimal ghci-like interpreter.

use colored::*;
use lazy_static::lazy_static;
use spin::Mutex;
use std::{
    collections::BTreeSet,
    io::{self, Write},
};

lazy_static! {
    static ref LAMBDA_CONTEXT: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());
}

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
    println!("{}              -- (λx. x x)", "omega".green());
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

fn print_out(output: ColoredString, color: Color) {
    static mut OUT_COUNTER: u32 = 1;
    unsafe {
        println!(
            "\n{}",
            format!(
                "{}{}",
                format!("OUT [{}]: ", OUT_COUNTER.to_string().bold()).color(color),
                output,
            )
        );
        OUT_COUNTER += 1;
    }
}

/// A very tiny parse utility to construct a speicified `Exp`
/// from command line.
/// Mainly used for testing and playing around with your stlc.
fn parse(name: &str, hint: Option<ColoredString>) -> Exp {
    if name == "var" {
        print!("\navailable variable(s) in current lambda context: ");
        let len = LAMBDA_CONTEXT.lock().len();
        if len == 0 {
            println!("{}.", "(null)".red().underline());
        } else {
            for (i, e) in LAMBDA_CONTEXT.lock().iter().enumerate() {
                if i == len - 1 {
                    print!("{}.\n", e.green().underline());
                    break;
                } else {
                    print!("{}, ", e.green().underline());
                }
            }
        }
        println!(
            "(of course you could choose any {} that fits your need. 🤪)",
            "free variable".green().underline()
        );
        print!("\nenter {} below.\n", "variable name".green().underline());
        let _ = io::stdout().flush();
        print_prompt();
        let input = read_line();
        return Var::build(input.as_str());
    }
    println!(
        "\nwhat do you want to {} for {}? {}\n{}\n{}",
        "build".bright_blue(),
        name.green().underline().bold(),
        hint.unwrap_or("".into()),
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
            parse(name, None)
        }
        "omega" => Lambda::build("x", App::build(Var::build("x"), Var::build("x"))),
        "var" => parse("var", None),
        "app" => App::build(
            parse(
                "app.t1",
                Some(format!("(i.e., app {} t2)", "t1".green().bold().underline()).into()),
            ),
            parse(
                "app.t2",
                Some(format!("(i.e., app t1 {})", "t2".green().bold().underline()).into()),
            ),
        ),
        "cond" => Cond::build(
            parse(
                "cond.if",
                Some(
                    format!(
                        "(i.e., if {} then t2 else t3)",
                        "t1".green().bold().underline()
                    )
                    .into(),
                ),
            ),
            parse(
                "cond.then",
                Some(
                    format!(
                        "(i.e., if t1 then {} else t3)",
                        "t2".green().bold().underline()
                    )
                    .into(),
                ),
            ),
            parse(
                "cond.else",
                Some(
                    format!(
                        "(i.e., if t1 then t2 else {})",
                        "t3".green().bold().underline()
                    )
                    .into(),
                ),
            ),
        ),
        "incr" => Incr::build(parse(
            "incr.e",
            Some(format!("(i.e., incr {})", "e".green().bold().underline()).into()),
        )),
        "decr" => Decr::build(parse(
            "decr.e",
            Some(format!("(i.e., decr {})", "e".green().bold().underline()).into()),
        )),
        "lambda" => {
            println!(
                "\nenter your lambda abstraction {} below. (i.e., λ{}. e)",
                "argument".green().underline(),
                "x".green().bold().underline()
            );
            print_prompt();
            let input = read_line();
            LAMBDA_CONTEXT.lock().insert(input.clone());
            let result = Lambda::build(
                input.as_str(),
                parse(
                    "lambda.e",
                    Some(format!("(i.e., λx. {})", "e".green().bold().underline()).into()),
                ),
            );
            LAMBDA_CONTEXT.lock().remove(input.as_str());
            result
        }
        "true" => Exp::True,
        "false" => Exp::False,
        "is_zero" => IsZero::build(parse(
            "is_zero.e",
            Some(format!("(i.e., is_zero {})", "e".green().bold().underline()).into()),
        )),
        "nat" => {
            println!(
                "\nenter a {} below.",
                "non-negative number".green().underline()
            );
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
        _ => {
            let output = format!(
                "{} is not a valid expression to choose.",
                input.red().underline()
            );
            print_out(output.into(), Color::BrightRed);
            parse(name, None)
        }
    }
}

pub fn start_interactive_shell() {
    println!("Congratulations, the program compiles.");

    loop {
        let exp = parse("begin", None);
        let output = format!(
            "your expression {} has been built.",
            exp.to_string().bold().underline().green()
        );
        print_out(output.into(), Color::BrightBlue);
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
                    let output = format!(
                        "\n{} has not been supported, PR(s) welcome.",
                        input.red().underline()
                    );
                    print_out(output.into(), Color::BrightRed);
                    println!("please choose an available strategy.");
                }
            }
        }
        println!(
            "\nstart evaluating {} to normal form by {}.",
            exp.to_string().underline(),
            eval_strategy.to_string().green().underline().bold()
        );
        match exp.clone().eval_to_normal_form(eval_strategy) {
            Ok(res) => print_out(
                res.to_string().underline().bold().green(),
                Color::BrightBlue,
            ),
            Err(err) => {
                let output = format!(
                    "failed to evaluate {}, error: {}",
                    exp.to_string().underline().red(),
                    err
                );
                print_out(output.into(), Color::BrightRed);
            }
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
