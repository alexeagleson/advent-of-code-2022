use std::{char::ParseCharError, collections::HashSet, str::FromStr};

pub mod input;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct Delta {
    x: i32,
    y: i32,
}

impl From<Delta> for Option<Direction> {
    fn from(delta: Delta) -> Self {
        let Delta { x, y } = delta;

        assert_valid_delta(&delta);

        match (x, y) {
            (0, 2) => Some(Direction::North),
            (1, 2) => Some(Direction::NorthEast),
            (2, 2) => Some(Direction::NorthEast),
            (2, 1) => Some(Direction::NorthEast),
            (2, 0) => Some(Direction::East),
            (2, -1) => Some(Direction::SouthEast),
            (2, -2) => Some(Direction::SouthEast),
            (1, -2) => Some(Direction::SouthEast),
            (0, -2) => Some(Direction::South),
            (-1, -2) => Some(Direction::SouthWest),
            (-2, -2) => Some(Direction::SouthWest),
            (-2, -1) => Some(Direction::SouthWest),
            (-2, 0) => Some(Direction::West),
            (-2, 1) => Some(Direction::NorthWest),
            (-2, 2) => Some(Direction::NorthWest),
            (-1, 2) => Some(Direction::NorthWest),
            (_x, _y) => None,
        }
    }
}

fn assert_valid_delta(delta: &Delta) {
    if delta.x > 2 || delta.x < -2 || delta.y > 2 || delta.y < -2 {
        panic!("Delta out of bounds {:?}", delta)
    }
}

impl Position {
    pub fn get_delta_to_head(&self, head: &Position) -> Delta {
        Delta {
            x: head.x - self.x,
            y: head.y - self.y,
        }
    }

    pub fn add_delta(&mut self, delta: &Delta) {
        self.x += delta.x;
        self.y += delta.y;
    }
}

impl FromStr for Direction {
    type Err = ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::East),
            "U" => Ok(Direction::North),
            "L" => Ok(Direction::West),
            "D" => Ok(Direction::South),
            _ => unreachable!("Not a direction"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct MoveAction {
    direction: Direction,
    quantity: i32,
}

impl From<&Direction> for Delta {
    fn from(dir: &Direction) -> Self {
        match dir {
            Direction::North => Self { x: 0, y: 1 },
            Direction::NorthEast => Self { x: 1, y: 1 },
            Direction::East => Self { x: 1, y: 0 },
            Direction::SouthEast => Self { x: 1, y: -1 },
            Direction::South => Self { x: 0, y: -1 },
            Direction::SouthWest => Self { x: -1, y: -1 },
            Direction::West => Self { x: -1, y: 0 },
            Direction::NorthWest => Self { x: -1, y: 1 },
        }
    }
}

fn parse_input(str: &str) -> Vec<MoveAction> {
    let mut move_actions = Vec::new();
    for line in str.lines() {
        let trimmed_line = line.trim();

        if trimmed_line == "" {
            continue;
        }

        let parsed_line = trimmed_line.split_once(" ").expect("Failed to parse line");

        let action = MoveAction {
            direction: Direction::from_str(parsed_line.0).expect("Invalid direction"),
            quantity: i32::from_str_radix(parsed_line.1, 10).expect("Invalid number of moves"),
        };

        move_actions.push(action);
    }
    move_actions
}

fn run_move_action(
    action: &MoveAction,
    head: &mut Position,
    tails: &mut Vec<Position>,
    tail_positions_visited: &mut HashSet<Position>,
) {
    let head_move_delta: Delta = (&action.direction).into();
    head.add_delta(&head_move_delta);

    for tail_idx in 0..tails.len() {
        let tail = &tails[tail_idx];
        let is_first_tail = tail_idx == 0;
        let is_last_tail = tail_idx == tails.len() - 1;

        let tail_delta = if is_first_tail {
            tail.get_delta_to_head(&head)
        } else {
            tail.get_delta_to_head(&tails[tail_idx - 1])
        };

        let tail_move_direction: Option<Direction> = tail_delta.into();

        if let Some(tail_move_direction) = tail_move_direction {
            let tail_move_delta: Delta = (&tail_move_direction).into();
            tails[tail_idx].add_delta(&tail_move_delta);

            if is_last_tail {
                tail_positions_visited.insert(tails[tails.len() - 1].clone());
            }
        }
    }
}

fn run_move_actions(
    actions: &Vec<MoveAction>,
    head: &mut Position,
    tails: &mut Vec<Position>,
) -> HashSet<Position> {
    let mut tail_positions_visited: HashSet<Position> = HashSet::new();

    tail_positions_visited.insert(tails[0].clone());

    for action in actions {
        for _ in 0..action.quantity {
            run_move_action(&action, head, tails, &mut tail_positions_visited);
        }
    }

    tail_positions_visited
}

pub fn day_9_part_1(input: &'static str) -> String {
    let mut head = Position { x: 0, y: 0 };

    let mut tails = vec![Position { x: 0, y: 0 }];

    let actions = parse_input(input);

    let tail_visited = run_move_actions(&actions, &mut head, &mut tails);

    tail_visited.len().to_string()
}

pub fn day_9_part_2(input: &'static str) -> String {
    let mut head = Position { x: 0, y: 0 };

    let mut tails = vec![
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
    ];

    let actions = parse_input(input);

    let tail_visited = run_move_actions(&actions, &mut head, &mut tails);

    tail_visited.len().to_string()
}

#[cfg(test)]
mod test {

    use super::*;

    static TEST_INPUT: &str = r#"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "#;

    #[test]
    fn test_delta_and_direction() {
        let head = Position { x: 3, y: 3 };

        let tail = Position { x: 2, y: 1 };

        let delta = tail.get_delta_to_head(&head);

        assert_eq!(delta, Delta { x: 1, y: 2 });

        let tail_move_direction: Option<Direction> = delta.into();

        assert_eq!(tail_move_direction, Some(Direction::NorthEast));
    }

    #[test]
    fn test_delta_add() {
        let mut tail = Position { x: 3, y: 6 };

        let delta = Delta { x: 2, y: 1 };

        tail.add_delta(&delta);

        assert_eq!(tail, Position { x: 5, y: 7 });
    }

    #[test]
    fn test_parse_input() {
        let actions = parse_input(TEST_INPUT);

        assert_eq!(
            actions[0],
            MoveAction {
                direction: Direction::East,
                quantity: 4,
            },
        );

        assert_eq!(
            actions[6],
            MoveAction {
                direction: Direction::West,
                quantity: 5,
            },
        );
    }

    #[test]
    fn test_moves() {
        let mut head = Position { x: 0, y: 0 };

        let mut tails = vec![Position { x: 0, y: 0 }];

        let actions = parse_input(TEST_INPUT);

        let mut tail_positions_visited: HashSet<Position> = HashSet::new();

        for (idx, action) in actions.iter().enumerate() {
            for _ in 0..action.quantity {
                run_move_action(&action, &mut head, &mut tails, &mut tail_positions_visited);
            }

            let tail = &tails[0];

            if idx == 0 {
                assert_eq!(head, Position { x: 4, y: 0 });
                assert_eq!(tail, &Position { x: 3, y: 0 });
            } else if idx == 1 {
                assert_eq!(head, Position { x: 4, y: 4 });
                assert_eq!(tail, &Position { x: 4, y: 3 });
            } else if idx == 1 {
                assert_eq!(head, Position { x: 1, y: 4 });
                assert_eq!(tail, &Position { x: 2, y: 3 });
            };
        }

        let tail = &tails[0];

        assert_eq!(head, Position { x: 2, y: 2 });
        assert_eq!(tail, &Position { x: 1, y: 2 });
    }

    #[test]
    fn test_visited() {
        let mut head = Position { x: 0, y: 0 };

        let mut tails = vec![Position { x: 0, y: 0 }];

        let actions = parse_input(TEST_INPUT);

        let tail_visited = run_move_actions(&actions, &mut head, &mut tails);

        assert_eq!(tail_visited.len(), 13);
    }

    static TEST_INPUT_LONGER_ROPE: &str = r#"
        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20
    "#;

    #[test]
    fn test_parse_input_longer() {
        let actions = parse_input(TEST_INPUT_LONGER_ROPE);

        assert_eq!(
            actions[4],
            MoveAction {
                direction: Direction::East,
                quantity: 17,
            },
        );

        assert_eq!(
            actions[6],
            MoveAction {
                direction: Direction::West,
                quantity: 25,
            },
        );
    }

    #[test]
    fn test_moves_longer_rope() {
        let mut head = Position { x: 0, y: 0 };

        let mut tails = vec![
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
        ];

        let actions = parse_input(TEST_INPUT_LONGER_ROPE);

        let tail_visited = run_move_actions(&actions, &mut head, &mut tails);

        assert_eq!(tail_visited.len(), 36);
    }
}
