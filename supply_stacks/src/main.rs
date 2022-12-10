use anyhow::{bail, Context, Error};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Action {
    from: u8,
    to: u8,
    num: u32,
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }
        let caps = RE.captures(s).context("Input format is not right.")?;
        Ok(Action {
            from: (&caps[2]).parse::<u8>()? - 1,
            to: (&caps[3]).parse::<u8>()? - 1,
            num: (&caps[1]).parse()?,
        })
    }
}

#[derive(Debug, Clone)]
struct Crane {
    crates: Vec<VecDeque<char>>,
}

impl FromStr for Crane {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut crate_count = 0;
        // First step: determine the size.
        for line in s.lines() {
            let line_crate_count = (line.len() + 1) / 4;
            if crate_count == 0 {
                crate_count = line_crate_count;
            } else if crate_count != line_crate_count {
                bail!(
                    "Line count does not match: got {} was {}",
                    line_crate_count,
                    crate_count
                );
            }
        }
        // Second step: make the crates.
        let mut crates = Vec::with_capacity(crate_count);
        crates.resize(crate_count, VecDeque::new());
        for line in s.lines() {
            for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
                if c != ' ' {
                    crates[i].push_back(c);
                }
            }
        }
        // Not too elegent. Remove the last useless number line.
        for c in &mut crates {
            c.pop_back();
        }
        Ok(Crane { crates })
    }
}

impl Crane {
    fn perform_9k(&mut self, action: &Action) {
        for _ in 0..action.num {
            let object = self.crates[action.from as usize]
                .pop_front()
                .expect("Accidentally empty when doing action");
            self.crates[action.to as usize].push_front(object);
        }
    }

    fn perform_9001(&mut self, action: &Action) {
        let chunk: Vec<char> = (0..action.num)
            .map(|_| {
                self.crates[action.from as usize]
                    .pop_front()
                    .expect("Accidentally empty when doing action")
            })
            .collect();
        for c in chunk.iter().rev() {
            self.crates[action.to as usize].push_front(*c);
        }
    }

    fn top_view(&self) -> String {
        self.crates
            .iter()
            .map(|column| column.front().expect("Some column is empty"))
            .collect()
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/input").unwrap();
    let mut parts = contents.split("\n\n");
    let crane = Crane::from_str(parts.next().unwrap()).unwrap();
    let actions: Vec<Action> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| Action::from_str(line).expect("Line formt error. Line is {line}"))
        .collect();
    assert_eq!(parts.next(), None);

    let mut crane_9000 = crane.clone();
    for a in &actions {
        crane_9000.perform_9k(a);
    }
    println!("Question 1: {}", crane_9000.top_view());

    let mut crane_9001 = crane.clone();
    for a in &actions {
        crane_9001.perform_9001(a);
    }
    println!("Question 2: {}", crane_9001.top_view());
}
