use stlc::{exercises::day4_exercise::YCombinator, interactive_shell::start_interactive_shell};

fn main() {
    let f = YCombinator::gen_built_in_times();
    println!("f: {}", f);
    let yc = YCombinator::new(f);
    let Ok((res, steps)) = yc.eval(vec![3.into(), 3.into(), 4.into()], stlc::Strategy::CallByName) else {
        panic!("failed to eval yc");
    };
    println!("result: {}", res);
    start_interactive_shell();
}
