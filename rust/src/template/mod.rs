pub mod input;

#[derive(Debug, PartialEq)]
struct OutputToken;

impl OutputToken {
    pub fn new() -> Self {
        Self
    }
}

type ProcessedInput = Vec<OutputToken>;

fn parse_input(input: &str) -> ProcessedInput {
    let mut processed_output = Vec::new();

    for line in input.lines() {
        let trimmed_line = line.trim();

        if trimmed_line == "" {
            continue;
        }

        let token = OutputToken::new();

        processed_output.push(token);
    }

    processed_output
}

fn handle_processed_input(input: &mut ProcessedInput) {
    // Do something
}

pub fn day_11_part_1(input: &'static str) -> String {
    let mut processed_input = parse_input(input);

    handle_processed_input(&mut processed_input);

    "TBA".to_string()
}

pub fn day_11_part_2(input: &'static str) -> String {
    let mut processed_input = parse_input(input);

    handle_processed_input(&mut processed_input);

    "TBA".to_string()
}

#[cfg(test)]
mod test {

    use super::*;

    static TEST_INPUT: &str = r#""#;

    #[test]
    fn test_parse_input() {
        let processed_input = parse_input(TEST_INPUT);

        assert_eq!(processed_input, vec![]);
    }
}
