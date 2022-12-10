use aoc::Parser;

// Naive implementation of all_unique
fn all_unique(bytes: &[u8]) -> bool {
    if bytes.len() < 2 {
        return true;
    }
    let (head, tail) = bytes.split_at(1);
    !tail.contains(&head[0]) && all_unique(tail)
}

// I wrote this one before I decided to split up part 1 & 2 executions
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = aoc::Args::parse();
    let path = if cli.real {
        "input/06/real.txt"
    } else {
        "input/06/example.txt"
    };

    let input = std::fs::read_to_string(path)?;
    // Just one line today
    if let Some(line) = input.lines().next() {
        let window_len: usize = if !cli.part_two { 4 } else { 14 };
        line.as_bytes()
            .windows(window_len)
            .enumerate()
            .find(|(_, w)| all_unique(w))
            .iter()
            .for_each(|f| println!("{}", f.0 + window_len));
    }
    Ok(())
}
