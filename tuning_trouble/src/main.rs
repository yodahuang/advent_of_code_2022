use itertools::Itertools;
use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/input").unwrap();
    // https://stackoverflow.com/a/51261570
    for (i, w) in contents.as_bytes().windows(4).enumerate() {
        if w.iter().all_unique() {
            println!("Problem 1: {}", i + 4);
            break;
        }
    }
    for (i, w) in contents.as_bytes().windows(14).enumerate() {
        if w.iter().all_unique() {
            println!("Problem 1: {}", i + 14);
            break;
        }
    }
}
