//! Minimal iterator chains on this one, just some basic stupid parsing that's entirely over fit to the input
use aoc::Parser;

/// A very specific parser for this challenge's input
/// I'm fully aware the type's kind of complex but I don't care
#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut stacks = Vec::<Vec<char>>::new();
    let mut commands = Vec::<(usize, usize, usize)>::new();
    let mut command_mode = false;
    'newline: for line in input.lines().map(|l| l.trim_end()) {
        if !command_mode {
            if line.is_empty() {
                command_mode = true;
                continue;
            }
            if line.starts_with(" 1") {
                continue;
            }
            let mut chars = line.chars();
            let mut stack_idx = 0;
            while let Some(_open) = chars.next() {
                if let Some(label) = chars.next() {
                    while stacks.len() < (stack_idx + 1) {
                        stacks.push(Vec::new());
                    }
                    if label.is_ascii_alphabetic() {
                        stacks[stack_idx].insert(0, label);
                    }
                    let _close = chars.next().unwrap();
                    let Some(_space) = chars.next() else {
                        continue 'newline;
                    };
                    stack_idx += 1;
                }
            }
        } else {
            let mut parts = line.split(' ');
            parts.next().unwrap();
            let count = parts.next().unwrap().parse::<usize>().unwrap();
            parts.next().unwrap();
            let source = parts.next().unwrap().parse::<usize>().unwrap();
            parts.next().unwrap();
            let target = parts.next().unwrap().parse::<usize>().unwrap();
            commands.push((count, source - 1, target - 1));
        }
    }
    (stacks, commands)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = aoc::Args::parse();
    let path = if cli.real {
        "input/05/real.txt"
    } else {
        "input/05/example.txt"
    };

    let input = std::fs::read_to_string(path)?;

    let (mut stacks, commands) = parse_input(&input);
    for (count, source, target) in commands {
        let source_len = stacks[source].len();
        let mut to_move = stacks[source]
            .drain(source_len - count..)
            .collect::<Vec<_>>();
        if !cli.part_two {
            to_move.reverse();
        }
        stacks[target].append(&mut to_move);
    }
    let mut result = String::new();
    for stack in stacks {
        if let Some(label) = stack.last() {
            result.push(*label);
        }
    }
    println!("{}", result);
    Ok(())
}
