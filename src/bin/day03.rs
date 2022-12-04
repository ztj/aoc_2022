use std::collections::HashMap;
use std::collections::HashSet;

use aoc::Parser;

fn priority(c: char) -> u32 {
    match c {
        'A'..='Z' => (c as u32) - 38,
        'a'..='z' => (c as u32) - 96,
        _ => panic!("Invalid character"),
    }
}

fn part_one(input: String) {
    let total: u32 = input
        .lines()
        .flat_map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let (mut left_set, mut right_set) = (
                HashSet::with_capacity(line.len() / 2), // Rough guess for capacity
                HashSet::with_capacity(line.len() / 2),
            );
            left.chars().zip(right.chars()).find_map(|(l, r)| {
                if l == r {
                    return Some(priority(l));
                };
                if left_set.contains(&r) {
                    return Some(priority(r));
                }
                if right_set.contains(&l) {
                    return Some(priority(l));
                }
                left_set.insert(l);
                right_set.insert(r);
                None
            })
        })
        .sum();
    println!("{total}");
}

fn part_two(input: String) {
    let mut lines = input.lines();
    let total: u32 = std::iter::from_fn(|| Some([lines.next()?, lines.next()?, lines.next()?]))
        .flat_map(|bags| {
            let mut history = HashMap::<char, [bool; 3]>::new();
            bags.into_iter().enumerate().find_map(|(idx, bag)| {
                bag.chars().find_map(|c| {
                    let record = history.entry(c).or_default();
                    record[idx] = true;
                    record.iter().all(|r| *r).then(|| priority(c))
                })
            })
        })
        .sum();
    println!("{total}");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = aoc::Args::parse();
    let path = if cli.real {
        "input/03/real.txt"
    } else {
        "input/03/example.txt"
    };

    let input = std::fs::read_to_string(path)?;

    if !cli.part_two {
        part_one(input);
    } else {
        part_two(input);
    }
    Ok(())
}
