pub mod input;

#[derive(Debug, Ord, Eq)]
struct Elf {
    #[allow(dead_code)]
    pub num: usize,
    pub calories: i32,
}

impl Elf {
    pub fn new(num: usize) -> Self {
        Self { num, calories: 0 }
    }

    pub fn eat(&mut self, calories: i32) {
        self.calories += calories
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.calories == other.calories
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.calories.partial_cmp(&other.calories)
    }
}

fn input_to_elves(input: &'static str) -> Vec<Elf> {
    let mut elf_index: usize = 0;
    let mut elves: Vec<Elf> = vec![Elf::new(elf_index)];

    for line in input.lines() {
        if line.trim() == "" {
            elf_index += 1;
            elves.push(Elf::new(elf_index));
        } else {
            let current_elf = elves
                .get_mut(elf_index)
                .expect("You forgot to push a new elf to this index");
            let calories = line.trim().parse::<i32>().expect("Not a number");
            current_elf.eat(calories);
        }
    }

    elves
}

fn find_best_fed_elf(elves: &Vec<Elf>) -> &Elf {
    let mut most_cals: i32 = 0;
    let mut most_cals_index: usize = 0;
    elves.iter().enumerate().for_each(|(idx, elf)| {
        if elf.calories > most_cals {
            most_cals = elf.calories;
            most_cals_index = idx;
        }
    });
    elves.get(most_cals_index).unwrap()
}

pub fn day_1_part_1(input: &'static str) -> i32 {
    let elves = input_to_elves(input);

    let best_fed_elf = find_best_fed_elf(&elves);

    best_fed_elf.calories
}

pub fn day_1_part_2(input: &'static str) -> i32 {
    let mut elves = input_to_elves(input);
    elves.sort();

    elves.reverse();

    let top_3_cals = elves
        .iter()
        .take(3)
        .fold(0, |acc, elem| acc + elem.calories);

    top_3_cals
}

#[cfg(test)]
mod test {
    static TEST_INPUT: &str = r#"
    1000
    2000
    3000

    4000

    5000
    6000

    7000
    8000
    9000

    10000
    "#;

    #[test]
    fn part_1() {
        let calories = super::day_1_part_1(TEST_INPUT);

        assert_eq!(calories, 24000);
    }

    #[test]
    fn part_2() {
        let calories = super::day_1_part_2(TEST_INPUT);

        assert_eq!(calories, 45000);
    }
}
