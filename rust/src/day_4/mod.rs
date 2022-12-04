pub mod input;

#[derive(Debug, PartialEq)]
struct CleaningAssignment {
    start: i32,
    end: i32,
}

impl CleaningAssignment {
    pub fn from_str_range(str_range: &'static str) -> Self {
        // str_range is the form "9-10" or "50-60"

        let mut start_and_end = str_range.trim().split("-");

        let start =
            i32::from_str_radix(start_and_end.next().unwrap(), 10).expect("Start value invalid");
        let end =
            i32::from_str_radix(start_and_end.next().unwrap(), 10).expect("End value invalid");

        CleaningAssignment { start, end }
    }

    pub fn fully_overlaps_with(&self, other: &CleaningAssignment) -> bool {
        if self.start >= other.start && self.end <= other.end {
            true
        } else if other.start >= self.start && other.end <= self.end {
            true
        } else {
            false
        }
    }

    pub fn partially_overlaps_with(&self, other: &CleaningAssignment) -> bool {
        if self.start >= other.start && self.start <= other.end {
            true
        } else if self.end >= other.start && self.end <= other.end {
            true
        } else if other.start >= self.start && other.start <= self.end {
            true
        } else if other.end >= self.start && other.end <= self.end {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq)]
struct CleaningAssignmentPair {
    left: CleaningAssignment,
    right: CleaningAssignment,
}

impl CleaningAssignmentPair {
    pub fn from_str_range_pair(str_range_pair: &'static str) -> Self {
        // str_range is the form "1-2,9-10" or "10-20,50-60"

        let mut left_and_right = str_range_pair.trim().split(",");

        let left =
            CleaningAssignment::from_str_range(left_and_right.next().expect("Left range invalid"));
        let right =
            CleaningAssignment::from_str_range(left_and_right.next().expect("Right range invalid"));

        Self { left, right }
    }

    pub fn fully_overlaps(&self) -> bool {
        self.left.fully_overlaps_with(&self.right)
    }

    pub fn partially_overlaps(&self) -> bool {
        self.left.partially_overlaps_with(&self.right)
    }
}

trait Overlap {
    fn overlapping_assignments(&self) -> i32;

    fn partially_overlapping_assignments(&self) -> i32;
}

impl Overlap for Vec<CleaningAssignmentPair> {
    fn overlapping_assignments(&self) -> i32 {
        self.iter().fold(
            0,
            |acc, curr| if curr.fully_overlaps() { acc + 1 } else { acc },
        )
    }

    fn partially_overlapping_assignments(&self) -> i32 {
        self.iter().fold(0, |acc, curr| {
            if curr.partially_overlaps() {
                acc + 1
            } else {
                acc
            }
        })
    }
}

fn parse_input_to_pairs(input: &'static str) -> Vec<CleaningAssignmentPair> {
    let mut pairs = Vec::new();
    for line in input.lines() {
        if line.trim() == "" {
            continue;
        }

        pairs.push(CleaningAssignmentPair::from_str_range_pair(line));
    }

    pairs
}

pub fn day_4_part_1(input: &'static str) -> i32 {
    let pairs = parse_input_to_pairs(input);

    pairs.overlapping_assignments()
}

pub fn day_4_part_2(input: &'static str) -> i32 {
    let pairs = parse_input_to_pairs(input);

    pairs.partially_overlapping_assignments()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str_range() {
        let val = CleaningAssignment { start: 10, end: 20 };

        assert_eq!(val, CleaningAssignment::from_str_range("10-20"));
    }

    #[test]
    fn test_from_str_range_pair() {
        let val = CleaningAssignmentPair {
            left: CleaningAssignment { start: 10, end: 20 },
            right: CleaningAssignment { start: 30, end: 40 },
        };

        assert_eq!(
            val,
            CleaningAssignmentPair::from_str_range_pair("10-20,30-40")
        );
    }

    #[test]
    fn test_fully_overlaps_with() {
        let val = CleaningAssignment::from_str_range("2-8");
        let val2 = CleaningAssignment::from_str_range("3-7");

        assert_eq!(val.fully_overlaps_with(&val2), true);

        let val = CleaningAssignment::from_str_range("5-8");
        let val2 = CleaningAssignment::from_str_range("3-7");

        assert_eq!(val.fully_overlaps_with(&val2), false);

        let val = CleaningAssignment::from_str_range("6-6");
        let val2 = CleaningAssignment::from_str_range("4-6");

        assert_eq!(val.fully_overlaps_with(&val2), true);
    }

    #[test]
    fn test_fully_overlaps_pair() {
        let val = CleaningAssignmentPair::from_str_range_pair("2-8,3-7");
        assert_eq!(val.fully_overlaps(), true);

        let val = CleaningAssignmentPair::from_str_range_pair("5-7,7-9");
        assert_eq!(val.fully_overlaps(), false);

        let val = CleaningAssignmentPair::from_str_range_pair("6-6,4-6");
        assert_eq!(val.fully_overlaps(), true);
    }

    #[test]
    fn test_partially_overlaps_with() {
        let val = CleaningAssignment::from_str_range("2-8");
        let val2 = CleaningAssignment::from_str_range("3-7");

        assert_eq!(val.partially_overlaps_with(&val2), true);

        let val = CleaningAssignment::from_str_range("5-8");
        let val2 = CleaningAssignment::from_str_range("3-7");

        assert_eq!(val.partially_overlaps_with(&val2), true);

        let val = CleaningAssignment::from_str_range("6-6");
        let val2 = CleaningAssignment::from_str_range("4-6");

        assert_eq!(val.partially_overlaps_with(&val2), true);

        let val = CleaningAssignment::from_str_range("2-3");
        let val2 = CleaningAssignment::from_str_range("4-5");

        assert_eq!(val.partially_overlaps_with(&val2), false);
    }

    static TEST_INPUT: &str = r#"
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    "#;

    #[test]
    fn part_1() {
        let pairs = parse_input_to_pairs(TEST_INPUT);

        assert_eq!(pairs.overlapping_assignments(), 2);
    }

    #[test]
    fn part_2() {
        let pairs = parse_input_to_pairs(TEST_INPUT);

        assert_eq!(pairs.partially_overlapping_assignments(), 4);
    }
}
