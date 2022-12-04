use std::ops::RangeInclusive;

use aoc::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = aoc::Args::parse();
    let path = if cli.real {
        "input/04/real.txt"
    } else {
        "input/04/example.txt"
    };

    let input = std::fs::read_to_string(path)?;

    let total = input
        .lines()
        .filter_map(|line| -> Option<()> {
            let pair = line.split_once(',').unwrap();
            let begin_end_one = pair.0.split_once('-').unwrap();
            let begin_end_two = pair.1.split_once('-').unwrap();
            let range_one =
                begin_end_one.0.parse::<u32>().unwrap()..=begin_end_one.1.parse::<u32>().unwrap();
            let range_two =
                begin_end_two.0.parse::<u32>().unwrap()..=begin_end_two.1.parse::<u32>().unwrap();
            if !cli.part_two {
                either_range_contains_the_other(range_one, range_two).then_some(())
            } else {
                ranges_overlap_at_all(range_one, range_two).then_some(())
            }
        })
        .count();
    println!("{total}");

    Ok(())
}

fn either_range_contains_the_other(a: RangeInclusive<u32>, b: RangeInclusive<u32>) -> bool {
    (a.start() <= b.start() && a.end() >= b.end()) || (b.start() <= a.start() && b.end() >= a.end())
}

fn ranges_overlap_at_all(a: RangeInclusive<u32>, b: RangeInclusive<u32>) -> bool {
    b.contains(a.start()) || b.contains(a.end()) || a.contains(b.end()) || a.contains(b.end())
}
