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
    time::{Duration, Instant},
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

fn print_list_msg() {
    println!(
        "\ncurrently supported syntax is as below. ({})\n",
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
                format!("Out[{}]: ", OUT_COUNTER.to_string().bold()).color(color),
                output,
            )
        );
        OUT_COUNTER += 1;
    }
}

fn push_with_parenthesis(s: &mut String, e: &Exp) {
    s.push('(');
    s.push_str(&e.to_string());
    s.push(')');
}

/// A very tiny parse utility to construct a speicified `Exp`
/// from command line.
/// Mainly used for testing and playing around with your stlc.
fn parse(mut lhs: String, curr: String, rhs: String) -> Exp {
    if curr == "var" {
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
        "\nwhat do you want to {} for {}?",
        "build".bright_blue().bold().underline(),
        format!("{}{}{}", lhs, curr.green().underline().bold(), rhs),
    );
    print_prompt();
    let input = read_line();
    match input.to_ascii_lowercase().as_str() {
        "list" => {
            print_list_msg();
            parse(lhs, curr, rhs)
        }
        "omega" => Lambda::build("x", App::build(Var::build("x"), Var::build("x"))),
        "var" => parse("".to_string(), "var".to_string(), "".to_string()),
        "app" => {
            lhs.push_str("(");
            let curr_rhs = " t2)".to_string() + &rhs;
            let t1 = parse(lhs.clone(), "t1".to_string(), curr_rhs);
            push_with_parenthesis(&mut lhs, &t1);
            lhs.push(' ');
            let curr_rhs = ")".to_string() + &rhs;
            let t2 = parse(lhs, "t2".to_string(), curr_rhs);
            App::build(t1, t2)
        }
        "cond" => {
            lhs.push_str("(if ");
            let curr_rhs = " then t2 else t3)".to_string() + &rhs;
            let t1 = parse(lhs.clone(), "t1".to_string(), curr_rhs);
            push_with_parenthesis(&mut lhs, &t1);
            lhs.push_str(" then ");
            let curr_rhs = " else t3)".to_string() + &rhs;
            let t2 = parse(lhs.clone(), "t2".to_string(), curr_rhs);
            push_with_parenthesis(&mut lhs, &t2);
            lhs.push_str(" else ");
            let curr_rhs = ")".to_string() + &rhs;
            let t3 = parse(lhs, "t3".to_string(), curr_rhs);

            Cond::build(t1, t2, t3)
        }
        "incr" => {
            lhs.push_str("(incr ");
            let curr_rhs = ")".to_string() + &rhs;
            let e = parse(lhs, "e".to_string(), curr_rhs);
            Incr::build(e)
        }
        "decr" => {
            lhs.push_str("(decr ");
            let curr_rhs = ")".to_string() + &rhs;
            let e = parse(lhs, "e".to_string(), curr_rhs);
            Decr::build(e)
        }
        "lambda" => {
            println!(
                "\nenter your lambda abstraction {} below. (i.e., λ{}. e)",
                "argument".green().underline(),
                "x".green().bold().underline()
            );
            print_prompt();
            let input = read_line();
            LAMBDA_CONTEXT.lock().insert(input.clone());
            lhs.push_str(format!("(λ{}. ", input).as_str());
            let curr_rhs = ")".to_string() + &rhs;
            let result = parse(lhs, "e".to_string(), curr_rhs);
            LAMBDA_CONTEXT.lock().remove(input.as_str());
            Lambda::build(&input, result)
        }
        "true" => Exp::True,
        "false" => Exp::False,
        "is_zero" => {
            lhs.push_str("(is_zero ");
            let curr_rhs = ")".to_string() + &rhs;
            let e = parse(lhs, "e".to_string(), curr_rhs);
            IsZero::build(e)
        }
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
            parse(lhs, curr, rhs)
        }
    }
}

fn print_statistics(duration: Duration, steps: u32) {
    println!("\n{}", "statistics".bold());
    println!("----");
    println!("{}:  {} ms", "time".green(), duration.as_millis().to_string().underline());
    println!("{}: {} steps", "steps".green(), steps.to_string().underline());
    println!("----");
}

pub fn start_interactive_shell() {
    println!("Congratulations, the program compiles.");
    println!(
        "{}",
        format!(
            "You could type `{}` to display all available expr(s).",
            "list".green()
        )
    );

    loop {
        let lhs = String::from("");
        let rhs = String::from("");
        let exp = parse(lhs, "begin".to_string(), rhs);
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
                        "{} has not been supported, PR(s) welcome.",
                        input.red().underline()
                    );
                    print_out(output.into(), Color::BrightRed);
                    println!("please choose an available strategy.");
                }
            }
        }
        println!("\ndo you wish to use official or your own evaluation implementation?");
        println!(
            "(note: please implement `{}` before choosing your own.",
            "eval_to_normal_form".to_string().underline()
        );
        println!("\n1. {} 2. {}", "official".green(), "your own".green());
        print_prompt();
        let mut flag = false;
        loop {
            let input = read_line();
            match input.as_str() {
                "1" => break,
                "2" => {
                    flag = true;
                    break;
                }
                _ => {
                    print_out("please type the correct number.".into(), Color::Red);
                    continue;
                }
            }
        }
        println!(
            "\nstart evaluating {} to normal form by {} using {}.",
            exp.to_string().underline(),
            eval_strategy.to_string().green().underline().bold(),
            format!(
                "{} implementation",
                if flag {
                    "your own".underline()
                } else {
                    "official".underline()
                }
            ),
        );
        let start = Instant::now();
        let result = if flag {
            exp.clone().ref_eval_to_normal_form(eval_strategy)
        } else {
            exp.clone().ref_eval_to_normal_form(eval_strategy)
        };
        let duration = start.elapsed();
        match result {
            Ok((res, steps)) => {
                print_out(
                    res.to_string().underline().bold().green(),
                    Color::BrightBlue,
                );
                print_statistics(duration, steps);
            }
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
