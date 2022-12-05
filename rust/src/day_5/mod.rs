pub mod input;

type Stacks = Vec<Vec<char>>;

pub fn parse_stacks(input: &'static str) -> Stacks {
    let mut stacks: Stacks = Vec::new();

    for line in input.lines().rev() {
        line.as_bytes()
            .chunks(4)
            .enumerate()
            .for_each(|(idx, group)| {
                if stacks.get(idx).is_none() {
                    stacks.push(Vec::new());
                }

                if group[0] == b'[' {
                    stacks[idx].push(group[1] as char);
                } else {
                    // Do nothing
                }
            });
    }

    stacks
}

#[derive(Debug)]
struct MoveInstruction {
    from_stack: usize,
    to_stack: usize,
    quantity: i32,
}

fn parse_moves(input: &'static str) -> Vec<MoveInstruction> {
    let mut move_instructions = Vec::new();
    for line in input.lines() {
        let trimmed_line = line.trim();
        if trimmed_line == "" {
            continue;
        }

        // Filter out the text in the instructions and keep only the numerical values
        let mut tokens = trimmed_line
            .split(" ")
            .filter(|val| i32::from_str_radix(val, 10).is_ok());

        // Assign the three numerical values into a move instruction
        let move_instruction = MoveInstruction {
            quantity: i32::from_str_radix(tokens.next().unwrap(), 10).unwrap(),
            from_stack: usize::from_str_radix(tokens.next().unwrap(), 10).unwrap(),
            to_stack: usize::from_str_radix(tokens.next().unwrap(), 10).unwrap(),
        };
        move_instructions.push(move_instruction);
    }
    move_instructions
}

#[derive(Debug)]
struct MoveError;

fn execute_move(stacks: &mut Stacks, move_instruction: &MoveInstruction) -> Result<(), MoveError> {
    // dbg!("{:?}", move_instruction);
    for _ in 0..move_instruction.quantity {
        let val = stacks[move_instruction.from_stack - 1]
            .pop()
            .ok_or(MoveError)?;
        // println!("MOVING {}", &val);
        stacks[move_instruction.to_stack - 1].push(val);
    }

    // dbg!("{:?}", stacks);
    Ok(())
}

fn execute_move_crane_9001(
    stacks: &mut Stacks,
    move_instruction: &MoveInstruction,
) -> Result<(), MoveError> {
    // dbg!("{:?}", move_instruction);

    let from_idx = move_instruction.from_stack - 1;
    let stack_from_len = stacks[from_idx].len();
    let elems = stacks[from_idx]
        .drain((stack_from_len - move_instruction.quantity as usize)..stack_from_len)
        .collect::<Vec<_>>();

    stacks[move_instruction.to_stack - 1].extend(elems);

    // dbg!("{:?}", &stacks[move_instruction.to_stack - 1]);

    Ok(())
}

fn execute_moves(stacks: &mut Stacks, moves: &Vec<MoveInstruction>) {
    for move_instruction in moves {
        execute_move(stacks, move_instruction);
    }
}

fn execute_moves_crane_9001(stacks: &mut Stacks, moves: &Vec<MoveInstruction>) {
    for move_instruction in moves {
        execute_move_crane_9001(stacks, move_instruction);
    }
}

fn top_crates(stacks: &Stacks) -> String {
    let mut top_crates = String::new();

    for stack in stacks {
        let top_char = stack.get(stack.len() - 1);

        if let Some(top_char) = top_char {
            top_crates.push(*top_char);
        }
    }

    top_crates
}

pub fn day_5_part_1(input_stacks: &'static str, input_moves: &'static str) -> String {
    let mut stacks = parse_stacks(input_stacks);
    let moves = parse_moves(input_moves);
    execute_moves(&mut stacks, &moves);

    top_crates(&stacks)
}

pub fn day_5_part_2(input_stacks: &'static str, input_moves: &'static str) -> String {
    let mut stacks = parse_stacks(input_stacks);
    let moves = parse_moves(input_moves);
    execute_moves_crane_9001(&mut stacks, &moves);

    top_crates(&stacks)
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
    "#;

    static TEST_INPUT_MOVES: &str = r#"
    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2
    "#;

    #[test]
    fn test_parse_moves() {
        let moves = parse_moves(TEST_INPUT_MOVES);

        // Index values should be 1 less than text values
        assert_eq!(moves[0].from_stack, 2);
        assert_eq!(moves[0].to_stack, 1);
        assert_eq!(moves[0].quantity, 1);

        assert_eq!(moves[1].from_stack, 1);
        assert_eq!(moves[1].to_stack, 3);
        assert_eq!(moves[1].quantity, 3);
        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn test_parse_stacks() {
        let stacks = parse_stacks(TEST_INPUT);

        assert_eq!(stacks[0][0], 'Z');
        assert_eq!(stacks[1][2], 'D');
        assert_eq!(stacks[2][0], 'P');

        assert_eq!(stacks.len(), 3);

        let stacks = parse_stacks(input::INPUT_DAY_5_STACKS);

        assert_eq!(stacks[1][1], 'D');
        assert_eq!(stacks[7][3], 'V');

        assert_eq!(stacks.len(), 9);
    }

    #[test]
    fn test_execute_moves() {
        let mut stacks = parse_stacks(TEST_INPUT);
        let moves = parse_moves(TEST_INPUT_MOVES);

        execute_move(&mut stacks, &moves[0])
            .map_err(|err| println!("{:?}", err))
            .unwrap();

        assert_eq!(stacks[0][2], 'D');

        execute_move(&mut stacks, &moves[1])
            .map_err(|err| println!("{:?}", err))
            .unwrap();

        assert_eq!(stacks[2][3], 'Z');

        execute_move(&mut stacks, &moves[2])
            .map_err(|err| println!("{:?}", err))
            .unwrap();

        assert_eq!(stacks.get(1), Some(&vec![]));
    }

    #[test]
    fn part_1() {
        let mut stacks = parse_stacks(TEST_INPUT);
        let moves = parse_moves(TEST_INPUT_MOVES);
        execute_moves(&mut stacks, &moves);

        assert_eq!(stacks[1][0], 'M');

        let top_crates = top_crates(&stacks);

        assert_eq!(top_crates, "CMZ".to_string());
    }

    #[test]
    fn part_2() {
        let mut stacks = parse_stacks(TEST_INPUT);
        let moves = parse_moves(TEST_INPUT_MOVES);
        execute_moves_crane_9001(&mut stacks, &moves);

        // assert_eq!(stacks[1][0], 'M');

        let top_crates = top_crates(&stacks);

        assert_eq!(top_crates, "MCD".to_string());
    }
}
