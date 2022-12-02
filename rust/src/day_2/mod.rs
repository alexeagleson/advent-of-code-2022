pub mod input;

#[derive(Debug, PartialEq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

trait Score {
    fn score(&self) -> i32;
}

impl Score for Rps {
    fn score(&self) -> i32 {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }
}

impl<'a> From<&'a str> for Rps {
    fn from(s: &'a str) -> Self {
        match s {
            "A" => Rps::Rock,
            "B" => Rps::Paper,
            "C" => Rps::Scissors,
            "X" => Rps::Rock,
            "Y" => Rps::Paper,
            "Z" => Rps::Scissors,
            _ => panic!("Cannot convert input to Rps: {}", s),
        }
    }
}

enum Outcome {
    Lose,
    Tie,
    Win,
}

impl Score for Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Tie => 3,
            Outcome::Win => 6,
        }
    }
}

impl<'a> From<&'a str> for Outcome {
    fn from(s: &'a str) -> Self {
        match s {
            "X" => Outcome::Lose,
            "Y" => Outcome::Tie,
            "Z" => Outcome::Win,
            _ => panic!("Cannot convert that input to an Outcome: {}", s),
        }
    }
}

fn what_to_choose(opponent: &Rps, intended_outcome: &Outcome) -> Rps {
    match (opponent, intended_outcome) {
        (Rps::Rock, Outcome::Lose) => Rps::Scissors,
        (Rps::Rock, Outcome::Tie) => Rps::Rock,
        (Rps::Rock, Outcome::Win) => Rps::Paper,
        (Rps::Paper, Outcome::Lose) => Rps::Rock,
        (Rps::Paper, Outcome::Tie) => Rps::Paper,
        (Rps::Paper, Outcome::Win) => Rps::Scissors,
        (Rps::Scissors, Outcome::Lose) => Rps::Paper,
        (Rps::Scissors, Outcome::Tie) => Rps::Scissors,
        (Rps::Scissors, Outcome::Win) => Rps::Rock,
    }
}

fn rps_battle(lhs: &Rps, rhs: &Rps) -> Outcome {
    match (lhs, rhs) {
        (Rps::Rock, Rps::Rock) => Outcome::Tie,
        (Rps::Rock, Rps::Paper) => Outcome::Win,
        (Rps::Rock, Rps::Scissors) => Outcome::Lose,
        (Rps::Paper, Rps::Rock) => Outcome::Lose,
        (Rps::Paper, Rps::Paper) => Outcome::Tie,
        (Rps::Paper, Rps::Scissors) => Outcome::Win,
        (Rps::Scissors, Rps::Rock) => Outcome::Win,
        (Rps::Scissors, Rps::Paper) => Outcome::Lose,
        (Rps::Scissors, Rps::Scissors) => Outcome::Tie,
    }
}
type SingleMatchup = (Rps, Rps);
type Matchups = Vec<SingleMatchup>;

fn parse_input_part_1(input: &'static str) -> Matchups {
    let mut matchups: Matchups = Default::default();

    for line in input.lines() {
        if line.trim() == "" {
            continue;
        } else {
            let mut split = line.trim().split(" ");
            let opponent_move: Rps = split.next().unwrap().into();
            let my_move: Rps = split.next().unwrap().into();
            let matchup: SingleMatchup = (opponent_move, my_move);
            matchups.push(matchup);
        }
    }

    matchups
}

fn parse_input_part_2(input: &'static str) -> Matchups {
    let mut matchups: Matchups = Default::default();

    for line in input.lines() {
        if line.trim() == "" {
            continue;
        } else {
            let mut split = line.trim().split(" ");
            let opponent_move: Rps = split.next().unwrap().into();
            let intended_outcome: Outcome = split.next().unwrap().into();
            let my_move = what_to_choose(&opponent_move, &intended_outcome);
            let matchup: SingleMatchup = (opponent_move, my_move);
            matchups.push(matchup);
        }
    }

    matchups
}

fn get_score(matchup: &SingleMatchup) -> i32 {
    let outcome = rps_battle(&matchup.0, &matchup.1);
    let my_choice = &matchup.1;

    outcome.score() + my_choice.score()
}

fn get_total_scores(matchups: &Matchups) -> i32 {
    matchups
        .iter()
        .fold(0, |acc, matchup| acc + get_score(matchup))
}

pub fn day_2_part_1(input: &'static str) -> i32 {
    let matchups = parse_input_part_1(input);

    get_total_scores(&matchups)
}

pub fn day_2_part_2(input: &'static str) -> i32 {
    let matchups = parse_input_part_2(input);

    get_total_scores(&matchups)
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
    A Y
    B X
    C Z
    "#;

    #[test]
    fn part_1() {
        let matchups = super::parse_input_part_1(TEST_INPUT);

        assert_eq!(get_total_scores(&matchups), 15);
    }

    #[test]
    fn part_2() {
        let matchups = super::parse_input_part_2(TEST_INPUT);

        assert_eq!(get_total_scores(&matchups), 12);
    }

    #[test]
    fn correct_selection() {
        assert_eq!(what_to_choose(&Rps::Paper, &Outcome::Lose), Rps::Rock);

        assert_eq!(what_to_choose(&Rps::Scissors, &Outcome::Tie), Rps::Scissors);

        assert_eq!(what_to_choose(&Rps::Rock, &Outcome::Win), Rps::Paper);
    }
}
