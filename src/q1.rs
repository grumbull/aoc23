use std::collections::VecDeque;
use std::fs::read_to_string;

fn get_digit_at(line: &String, i: usize) -> Option<u32> {
    const RADIX: u32 = 10;
    match line.chars().nth(i) {
        Some(x) => x.to_digit(RADIX),
        None => None,
    }
}

struct WordBuffer {
    end_state: Vec<char>,
    number_value: u32,
    buffer: VecDeque<char>,
}

// Current impl doesn't work bc buffer needs to check all possible
// words.
impl WordBuffer {
    fn new(end_state: &String, number: u32) -> WordBuffer {
        return WordBuffer {
            end_state: end_state.chars().collect(),
            number_value: number,
            buffer: VecDeque::new(),
        };
    }

    fn push(&mut self, new_char: char) -> Option<u32> {
        let word_size = self.end_state.len();
        let buffer_full = self.buffer.len() == word_size;
        if buffer_full {
            self.buffer.pop_back().expect("Bufer full but can't pop back?");
        }
        self.buffer.push_front(new_char);
        let matching = self.buffer.iter().zip(&self.end_state).filter(|&(a, b)| a == b).count();
        if matching == word_size { 
            return Some(self.number_value)
        }
        return None;
    }
}

pub fn run() {
    println!("Question 1");

    let mut lines = Vec::new();

    for line in read_to_string("input/1-input.txt").unwrap().lines() {
        lines.push(line.to_string())
    }

    let mut sum = 0;

    for line in lines {
        let mut first = false;
        let mut last = false;
        let len = line.len();
        for i in 0..len {
            if first && last {
                break;
            };
            if !first {
                match get_digit_at(&line, i) {
                    Some(x) => {
                        first = true;
                        sum += x * 10
                    }
                    None => ()
                }
            }
            if !last {
                match get_digit_at(&line, len - i - 1) {
                    Some(x) => {
                        last = true;
                        sum += x
                    }
                    None => (),
                }
            }
        }
    }
    println!("Answer is: {sum}")
}
