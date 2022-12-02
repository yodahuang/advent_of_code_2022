use std::fs;

#[derive(Clone, Copy)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

enum OutCome {
    Win,
    Draw,
    Lose,
}

impl Action {
    fn score(&self) -> i8 {
        match self {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissors => 3,
        }
    }

    fn from_score(score: i8) -> Action {
        match score {
            1 => Action::Rock,
            2 => Action::Paper,
            0 => Action::Scissors,
            _ => panic!("Forbidden."),
        }
    }

    fn vs(&self, other: &Action) -> OutCome {
        match (self.score() + 3 - other.score()) % 3 {
            0 => OutCome::Draw,
            1 => OutCome::Win,
            2 => OutCome::Lose,
            other => panic!("This is impossible from math: {:?}", other),
        }
    }
}

impl OutCome {
    fn against(&self, other: &Action) -> Action {
        match self {
            OutCome::Win => Action::from_score((other.score() + 1) % 3),
            OutCome::Draw => other.clone(),
            OutCome::Lose => Action::from_score((other.score() + 2) % 3),
        }
    }

    fn score(&self) -> i8 {
        match self {
            OutCome::Win => 6,
            OutCome::Draw => 3,
            OutCome::Lose => 0,
        }
    }
}

fn parse_line(line: &str) -> (Action, Action) {
    let mut elements = line.split(' ');
    let opponent = match elements.next() {
        Some("A") => Action::Rock,
        Some("B") => Action::Paper,
        Some("C") => Action::Scissors,
        other => panic!("Unexpected character {:?}", other),
    };
    let me = match elements.next() {
        Some("X") => Action::Rock,
        Some("Y") => Action::Paper,
        Some("Z") => Action::Scissors,
        other => panic!("Unexpected character {:?}", other),
    };
    assert!(elements.next() == None);
    return (opponent, me);
}

fn parse_line_real(line: &str) -> (Action, OutCome) {
    let mut elements = line.split(' ');
    let opponent = match elements.next() {
        Some("A") => Action::Rock,
        Some("B") => Action::Paper,
        Some("C") => Action::Scissors,
        other => panic!("Unexpected character {:?}", other),
    };
    let me = match elements.next() {
        Some("X") => OutCome::Lose,
        Some("Y") => OutCome::Draw,
        Some("Z") => OutCome::Win,
        other => panic!("Unexpected character {:?}", other),
    };
    assert!(elements.next() == None);
    return (opponent, me);
}

fn main() {
    let contents = fs::read_to_string("inputs/input").expect("File not found");
    let total_score: i32 = contents
        .split_terminator('\n')
        .map(parse_line)
        .map(|(opponent, me)| me.score() + me.vs(&opponent).score())
        .map(|x| x as i32)
        .sum();
    println!("Question 1: {:?}", total_score);
    let total_score: i32 = contents
        .split_terminator('\n')
        .map(parse_line_real)
        .map(|(opponent, me)| me.against(&opponent).score() + me.score())
        .map(|x| x as i32)
        .sum();
    println!("Question 2: {:?}", total_score)
}
