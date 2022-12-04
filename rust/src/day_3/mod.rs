pub mod input;

const LOWERCASE_A: u8 = 97;
const LOWERCASE_Z: u8 = 122;
const UPPERCASE_A: u8 = 65;
const UPPERCASE_Z: u8 = 90;
const ALPHABET_OFFSET: u8 = 26;

fn is_lowercase(byte: u8) -> bool {
    byte >= LOWERCASE_A && byte <= LOWERCASE_Z
}

fn is_uppercase(byte: u8) -> bool {
    byte >= UPPERCASE_A && byte <= UPPERCASE_Z
}

fn get_priority(byte: u8) -> i32 {
    if is_lowercase(byte) {
        (byte - LOWERCASE_A + 1) as i32
    } else if is_uppercase(byte) {
        (byte - UPPERCASE_A + 1 + ALPHABET_OFFSET) as i32
    } else {
        panic!("Did not receive an upper or lowercase letter")
    }
}

#[derive(Debug)]

struct Container {
    first: Vec<u8>,
    second: Vec<u8>,
    matching_item: u8,
}

#[derive(Debug)]
struct ContainerTriple<'a> {
    a: &'a Container,
    b: &'a Container,
    c: &'a Container,
    matching_badge: u8,
}

impl<'a> ContainerTriple<'a> {
    pub fn new(a: &'a Container, b: &'a Container, c: &'a Container) -> Self {
        let matching_badge: u8 = a
            .first
            .iter()
            .zip(a.second.iter())
            .find_map(|(val_a, val_b)| {
                if b.contains_item(val_a) && c.contains_item(val_a) {
                    Some(*val_a)
                } else if b.contains_item(&val_b) && c.contains_item(&val_b) {
                    Some(*val_b)
                } else {
                    None
                }
            })
            .expect("Triple did not contain any matching badge");

        Self {
            a,
            b,
            c,
            matching_badge,
        }
    }

    pub fn matching_badge_char(&self) -> char {
        self.matching_badge as char
    }

    pub fn matching_badge_priority(&self) -> i32 {
        get_priority(self.matching_badge)
    }
}

impl Container {
    pub fn new(line: &'static str) -> Self {
        let mut first: Vec<u8> = Vec::new();
        let mut second: Vec<u8> = Vec::new();

        let line_len = line.len();

        for (idx, byte) in line.bytes().enumerate() {
            if idx < line_len / 2 {
                first.push(byte);
            } else {
                second.push(byte);
            }
        }

        let matching_item = *first
            .iter()
            .find(|byte| second.contains(byte))
            .expect("Couldn't find a matching item");

        Self {
            first,
            second,
            matching_item,
        }
    }

    pub fn matching_item(&self) -> u8 {
        self.matching_item
    }

    pub fn matching_item_char(&self) -> char {
        self.matching_item as char
    }

    pub fn priority(&self) -> i32 {
        get_priority(self.matching_item)
    }

    pub fn contains_item(&self, item: &u8) -> bool {
        self.first.contains(item) || self.second.contains(item)
    }
}

pub trait Priority {
    fn total_priority(&self) -> i32;
}

impl Priority for Vec<Container> {
    fn total_priority(&self) -> i32 {
        self.iter().fold(0, |acc, curr| acc + curr.priority())
    }
}

impl<'a> Priority for Vec<ContainerTriple<'a>> {
    fn total_priority(&self) -> i32 {
        self.iter()
            .fold(0, |acc, curr| acc + curr.matching_badge_priority())
    }
}

fn parse_input(input: &'static str) -> Vec<Container> {
    let mut containers = Vec::new();
    for line in input.lines() {
        let trimmed_line = line.trim();
        if trimmed_line == "" {
            continue;
        } else {
            let container = Container::new(trimmed_line);

            containers.push(container);
        }
    }
    containers
}

fn parse_triples<'a>(input: &'a Vec<Container>) -> Vec<ContainerTriple<'a>> {
    let triples = input
        .chunks(3)
        .map(|chunk| ContainerTriple::new(&chunk[0], &chunk[1], &chunk[2]))
        .collect::<Vec<_>>();

    triples
}

pub fn day_3_part_1(input: &'static str) -> i32 {
    let containers = parse_input(input);

    containers.total_priority()
}

pub fn day_3_part_2(input: &'static str) -> i32 {
    let containers = parse_input(input);

    let triples = parse_triples(&containers);

    triples.total_priority()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
    vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw
    "#;

    #[test]
    fn part_1() {
        // In the above example, the priority of the item type that appears in both compartments
        // of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

        let containers = parse_input(TEST_INPUT);

        assert_eq!(containers[0].matching_item_char(), 'p');

        assert_eq!(containers[0].priority(), 16);

        assert_eq!(containers[1].matching_item_char(), 'L');

        assert_eq!(containers[1].priority(), 38);

        assert_eq!(containers.total_priority(), 157);
    }

    #[test]
    fn part_2() {
        // In the first group, the only item type that appears in all three rucksacks is lowercase r;
        // this must be their badges. In the second group, their badge item type must be Z.

        // Priorities for these items must still be found to organize the sticker attachment efforts: here, they are 18 (r)
        // for the first group and 52 (Z) for the second group. The sum of these is 70.

        let containers = parse_input(TEST_INPUT);

        let triples = parse_triples(&containers);

        dbg!("{}", &triples);

        assert_eq!(triples[0].matching_badge_char(), 'r');

        assert_eq!(triples[1].matching_badge_char(), 'Z');

        assert_eq!(triples[0].matching_badge_priority(), 18);

        assert_eq!(triples[1].matching_badge_priority(), 52);

        assert_eq!(triples.total_priority(), 52 + 18);
    }
}
