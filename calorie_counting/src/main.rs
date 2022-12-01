use clap::Parser;
use std::collections::BinaryHeap;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    input_file: PathBuf,
}

fn part_one(contents: &str) -> i32 {
    contents
        .split("\n\n")
        .map(|elf| -> i32 {
            elf.split('\n')
                .map(|c| c.parse::<i32>().unwrap_or_default())
                .sum()
        })
        .max()
        .unwrap()
}

fn part_two(contents: &str) -> i32 {
    // I'm repeating the code, but it's for practice purposes.
    let mut heap = BinaryHeap::new();
    contents
        .split("\n\n")
        .map(|elf| -> i32 {
            elf.split('\n')
                .map(|c| c.parse::<i32>().unwrap_or_default())
                .sum()
        })
        .for_each(|sum| heap.push(sum));

    (0..3).map(|_| heap.pop().unwrap()).sum()
}

fn main() {
    let cli = Cli::parse();
    let contents = fs::read_to_string(cli.input_file).expect("File not found");
    println!("Part 1: {}", part_one(&contents));
    println!("Part 2: {}", part_two(&contents));
}
