fn main() {
    let input = include_str!("./input.txt");
    let output = process(&input);
    dbg!(output);
}

fn process(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test(){
        let example_input = "";
        let expected_output = 0;

        let result = super::process(&example_input);
        assert_eq!(result, expected_output.to_string());
    }
}
