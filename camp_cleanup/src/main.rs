use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::iter::Iterator;
use std::ops::Range;

fn get_ranges<'a>(contents: &'a str) -> impl Iterator<Item = (Range<i32>, Range<i32>)> + 'a {
    let match_to_int = |m: Option<regex::Match>| -> i32 { m.unwrap().as_str().parse().unwrap() };
    contents.lines().map(move |line| {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
        }
        let caps = RE.captures(line).unwrap();
        (
            match_to_int(caps.get(1))..(match_to_int(caps.get(2)) + 1),
            match_to_int(caps.get(3))..(match_to_int(caps.get(4)) + 1),
        )
    })
}

fn main() {
    let contents = fs::read_to_string("inputs/input").expect("File not found");
    let ranges = get_ranges(&contents);

    let first_question: i32 = ranges
        .map(|(first, second)| {
            assert!(!first.is_empty());
            assert!(!second.is_empty());
            if (first.start >= second.start && first.end <= second.end)
                || (second.start >= first.start && second.end <= first.end)
            {
                return 1;
            }
            0
        })
        .sum();

    println!("First question: {:?}", first_question);

    let ranges = get_ranges(&contents);

    let second_question: i32 = ranges
        .map(|(first, second)| {
            assert!(!first.is_empty());
            assert!(!second.is_empty());
            if first.start >= second.end || first.end <= second.start {
                return 0;
            }
            1
        })
        .sum();

    println!("Second question: {:?}", second_question);
}
