use std::collections::VecDeque;

pub mod input;

#[derive(Debug, PartialEq)]
enum OperationType {
    Add,
    Multiply,
}

#[derive(Debug, PartialEq)]
enum OperationValue {
    Old,
    Value(u128),
}

#[derive(Debug, PartialEq)]
struct Monkey {
    index: usize,
    items: VecDeque<u128>,
    operation_type: OperationType,
    operation_value: OperationValue,
    divisible_by: u128,
    monkey_throw_true: usize,
    monkey_throw_false: usize,
    inspection_count: u128,
}

impl Monkey {}

type ProcessedInput = Vec<Monkey>;

fn parse_input(input: &str) -> ProcessedInput {
    let mut processed_output = Vec::new();

    let mut lines = input.lines();

    loop {
        let line_1 = lines.next().map(|line| line.trim());

        if line_1 == Some("") {
            continue;
        }

        // Line 1
        let index = if let Some(line_1) = line_1 {
            let monkey_num = line_1
                .trim()
                .split_once(" ")
                .expect("Failed to parse line")
                .1
                .strip_suffix(":")
                .expect("Strip monkey suffix failed");

            usize::from_str_radix(monkey_num, 10).expect("Parse monkey num failed")
        } else {
            // Reached end of input text
            break;
        };

        // Line 2
        let monkey_items_line = lines.next().expect("Failed to parse items").trim();

        let monkey_items_nums_string = monkey_items_line
            .strip_prefix("Starting items: ")
            .expect("Failed to parse");

        let items = monkey_items_nums_string
            .split(", ")
            .map(|item| u128::from_str_radix(item, 10).expect("Failed to parse"))
            .collect::<VecDeque<_>>();

        // Line 3
        let line_3 = lines.next().expect("Failed to parse").trim();
        let stripped_prefix_line_3 = line_3.strip_prefix("Operation: new = old ").unwrap();

        let (operation_type_string, operation_value_string) =
            stripped_prefix_line_3.split_once(" ").unwrap();

        let operation_type: OperationType = match operation_type_string {
            "+" => OperationType::Add,
            "*" => OperationType::Multiply,
            _ => panic!(),
        };

        let operation_value: OperationValue = match operation_value_string {
            "old" => OperationValue::Old,
            val => {
                let value = u128::from_str_radix(val, 10).unwrap();
                OperationValue::Value(value)
            }
        };

        // Line 4
        let line_4 = lines.next().expect("Failed to parse").trim();
        let stripped_prefix_line_4 = line_4.strip_prefix("Test: divisible by ").unwrap();

        let divisible_by = u128::from_str_radix(stripped_prefix_line_4, 10).unwrap();

        // Line 5
        let line_5 = lines.next().expect("Failed to parse").trim();
        let stripped_prefix_line_5 = line_5.strip_prefix("If true: throw to monkey ").unwrap();
        let monkey_throw_true: usize = usize::from_str_radix(stripped_prefix_line_5, 10).unwrap();

        // Line 6
        let line_6 = lines.next().expect("Failed to parse").trim();
        let stripped_prefix_line_6 = line_6.strip_prefix("If false: throw to monkey ").unwrap();
        let monkey_throw_false: usize = usize::from_str_radix(stripped_prefix_line_6, 10).unwrap();

        let monkey = Monkey {
            index,
            items,
            operation_type,
            operation_value,
            divisible_by,
            monkey_throw_true,
            monkey_throw_false,
            inspection_count: 0,
        };

        processed_output.push(monkey);
    }

    processed_output
}

fn handle_processed_input(
    monkeys: &mut ProcessedInput,
    has_ability_to_feel_relief: bool,
    baseline: u128,
) {
    for monkey_index in 0..monkeys.len() {
        while monkeys[monkey_index].items.len() > 0 {
            let monkey = &mut monkeys[monkey_index];
            let item = monkey.items.pop_front();
            if let Some(item) = item {
                let operation_value = match &monkey.operation_value {
                    OperationValue::Old => item.clone(),
                    OperationValue::Value(val) => val.clone(),
                };

                let new_value_after_inspection = match monkey.operation_type {
                    OperationType::Add => (item + operation_value) % baseline,
                    OperationType::Multiply => (item * operation_value) % baseline,
                };

                let new_value_after_relief = if has_ability_to_feel_relief {
                    &new_value_after_inspection / 3
                } else {
                    new_value_after_inspection
                };

                let throw_target_test = &new_value_after_relief % monkey.divisible_by == 0;

                let monkey_throw_true = monkey.monkey_throw_true;
                let monkey_throw_false = monkey.monkey_throw_false;

                monkey.inspection_count += 1;

                drop(monkey);

                match throw_target_test {
                    true => monkeys[monkey_throw_true]
                        .items
                        .push_back(new_value_after_relief),
                    false => monkeys[monkey_throw_false]
                        .items
                        .push_back(new_value_after_relief),
                };
            }
        }
    }
}

fn monkey_business(input: &ProcessedInput) -> u128 {
    let mut vals = input
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<_>>();

    vals.sort();

    let most = vals.pop().unwrap();
    let second_most = vals.pop().unwrap();

    most * second_most
}

pub fn day_11_part_1(input: &'static str) -> String {
    let mut processed_input = parse_input(input);

    let baseline = processed_input
        .iter()
        .map(|monkey| monkey.divisible_by)
        .product::<u128>();

    for _ in 0..20 {
        handle_processed_input(&mut processed_input, true, baseline);
    }

    let monkey_business = monkey_business(&processed_input);

    monkey_business.to_string()
}

pub fn day_11_part_2(input: &'static str) -> String {
    let mut processed_input = parse_input(input);

    let baseline = processed_input
        .iter()
        .map(|monkey| monkey.divisible_by)
        .product::<u128>();

    for _ in 0..10000 {
        handle_processed_input(&mut processed_input, false, baseline);
    }

    let monkey_business = monkey_business(&processed_input);

    monkey_business.to_string()
}

#[cfg(test)]
mod test {

    use crate::day_11::input::TEST_INPUT;

    use super::*;

    #[test]
    fn test_parse_input() {
        let processed_input = parse_input(TEST_INPUT);

        assert_eq!(processed_input[3].monkey_throw_false, 1);
    }

    #[test]
    fn test_monkey_business() {
        let mut processed_input = parse_input(TEST_INPUT);

        let baseline = processed_input
            .iter()
            .map(|monkey| monkey.divisible_by)
            .product::<u128>();

        for _ in 0..20 {
            handle_processed_input(&mut processed_input, true, baseline);
        }

        let monkey_business = monkey_business(&processed_input);

        assert_eq!(processed_input[3].monkey_throw_false, 1);

        assert_eq!(monkey_business, 10605);
    }
}
