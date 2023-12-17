fn main() {
    let input = include_str!("./input.txt");
    let output = process(&input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut sum = 0;
    let mut line_num = 1;

    for line in input.lines() {
        let mut curr_line_val = iterate_over_line(&line, false, 10);
        curr_line_val += iterate_over_line(line, true, 1);
        dbg!(line_num, curr_line_val, line);
        sum += curr_line_val;
        line_num += 1;
    }

    return sum.to_string();
}

fn iterate_over_line(line: &str, reverse: bool, factor: u32) -> u32 {
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
             None => ()
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let example_input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let expected_output = 142;

        let result = super::process(&example_input);
        assert_eq!(result, expected_output.to_string());
    }
}
