use core::cmp;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(&input);
    dbg!(output);
}

fn process(input: &str) -> String {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let mut sum = 0;
    let mut game_id = 0;

    for mut line in input.lines() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        
        game_id += 1;
        // Using line number as game id is hack,
        // but string parsing is not the fun part of aoc.
        
        let colon_index = line.find(':').expect("No colon in line");
        line = &line[colon_index + 2..];
        for pull in line.split(";") {
            // dbg!(pull);
            for color in pull.split(",") {
                // dbg!(color);
                let parts = color.trim().split(" ").collect::<Vec<&str>>();
                let num = parts[0].parse::<u32>().unwrap();
                let color = parts[1];
                match color {
                    "red" => red = cmp::max(red, num),
                    "green" => green = cmp::max(green, num),
                    "blue" => blue = cmp::max(blue, num),
                    _ => panic!("Unrecognized color"),
                }
            }
        }

        if red <= MAX_RED && green <= MAX_GREEN && blue <= MAX_BLUE {
            sum += dbg!(game_id);
        }
    }
    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let example_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected_output = 8;

        let result = super::process(&example_input);
        assert_eq!(result, expected_output.to_string());
    }
}
