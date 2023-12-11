mod solutions;
mod util;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let question_number = &args[1];
    let question_number: u32 = match question_number.parse() {
        Ok(num) => num,
        Err(e) => panic!("Question number parse error: {e}"),
    };
    let question_input = &args[2];

    match question_number {
        1 => solutions::q1::run(question_input),
        _ => panic!("Unrecognized question number"),
    }
}
