use std::collections::VecDeque;
use day01::vec_deque_fixed::VecDequeFixed;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(&input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut sum = 0;
    let mut line_num = 1;

    for line in input.lines() {
        // This forward then reverse iteration is 2n, but gets the
        // benefit of early return. Efficacy depends on test input
        // and how close target strings are to the ends of the line.
        let mut curr_line_val = iterate_over_line(&line, false, 10, false);
        curr_line_val += iterate_over_line(line, true, 1, false);
        dbg!(line_num, curr_line_val, line);
        sum += curr_line_val;
        line_num += 1;
    }

    return sum.to_string();
}

fn iterate_over_line(line: &str, reverse: bool, factor: u32, debug: bool) -> u32 {
    const RADIX: u32 = 10;
    let iter: Box<dyn Iterator<Item = char>> = if reverse {
        Box::new(line.chars().rev())
    } else {
        Box::new(line.chars())
    };

    let mut word_buffer: VecDequeFixed<char> = VecDequeFixed {
        vec: VecDeque::new(),
        max_capacity: 5,
        push_front: reverse,
    };

    for item in iter {
        match item.to_digit(RADIX) {
            Some(x) => {
                return x * factor;
            }
            None => {
                word_buffer.push(item);
                if debug {
                    dbg!(&word_buffer);
                }
                // Compare word_buffer to known correct output
                // Lazy, but works.
                // TODO use some custom trie that returns the numerical value
                // when reaching a terminal state. Even that is a little tricky,
                // because ffive should match to 5.
                let buffer_contents: String = word_buffer.vec.iter().collect();
                if debug {
                    dbg!(&buffer_contents);
                }
                match match_number(&buffer_contents) {
                    Some(x) => return x * factor,
                    None => (),
                }
            }
        }
    }
    return 0;
}

fn match_number(input: &str) -> Option<u32> {
    // Can you tell I'm new to rust?
    // TODO Map {num_word -> u32}, iterate
    if input.contains("one") {
        return Some(1);
    }
    if input.contains("two") {
        return Some(2);
    }
    if input.contains("three") {
        return Some(3);
    }
    if input.contains("four") {
        return Some(4);
    }
    if input.contains("five") {
        return Some(5);
    }
    if input.contains("six") {
        return Some(6);
    }
    if input.contains("seven") {
        return Some(7);
    }
    if input.contains("eight") {
        return Some(8);
    }
    if input.contains("nine") {
        return Some(9);
    }
    return None;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let example_input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let expected_output = 281;

        let result = super::process(&example_input);
        assert_eq!(result, expected_output.to_string());
    }
}

