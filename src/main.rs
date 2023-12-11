mod q1;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let question_number  = &args[1];
    let question_number: u32 = 
        match question_number.parse(){
            Ok(num) => num,
            Err(e) => panic!("Question number parse error: {e}")
        };

    match question_number{
        1 => q1::run(),
        _ => panic!("Unrecognized question number")
    }
}
