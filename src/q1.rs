use std::collections::VecDeque;
use std::fs::read_to_string;
use std::str::Chars;

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
            self.buffer
                .pop_back()
                .expect("Bufer full but can't pop back?");
        }
        self.buffer.push_front(new_char);
        let matching = self
            .buffer
            .iter()
            .zip(&self.end_state)
            .filter(|&(a, b)| a == b)
            .count();
        if matching == word_size {
            return Some(self.number_value);
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
        sum += iterate_over_line(&line, false, 10);
        sum += iterate_over_line(&line, true, 1);
    }
    println!("Answer is: {sum}")
}

fn iterate_over_line(line: &String, reverse: bool, factor: u32) -> u32 {
    const RADIX: u32 = 10;
    let iter: Box<dyn Iterator<Item = char>> = if reverse {
        Box::new(line.chars().rev())
    } else {
        Box::new(line.chars())
    };

    for item in iter {
        match item.to_digit(RADIX) {
            Some(x) => {
                return x * factor;
            }
            None => (),
        }
    }
    return 0;
}
