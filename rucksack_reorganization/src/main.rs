use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn priority(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        return (item as u32) - ('a' as u32) + 1;
    } else if item.is_ascii_uppercase() {
        return (item as u32) - ('A' as u32) + 27;
    } else {
        panic!("This should never happen.");
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/input").expect("File not found");
    let priority_sum: u32 = contents
        .lines()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(first, second)| {
            let first_set = HashSet::<_>::from_iter(first.chars());
            let second_set = HashSet::<_>::from_iter(second.chars());
            let mut intersection = first_set.intersection(&second_set);
            let naughty_item = intersection.next();
            assert_eq!(intersection.next(), None);
            priority(*naughty_item.unwrap())
        })
        .sum();

    println!("Question 1: {:?}", priority_sum);

    let badge_sum: u32 = contents
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let mut sets = chunk.map(|line| HashSet::<_>::from_iter(line.chars()));
            let mut intersection = sets.next().unwrap();
            for other in sets {
                intersection.retain(|c| other.contains(c));
            }
            assert_eq!(intersection.len(), 1);
            priority(*intersection.iter().next().unwrap())
        })
        .sum();
    println!("Question 2: {:?}", badge_sum);
}
