use aoc::Parser;

// I wrote this one before I decided to split up part 1 & 2 executions
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = aoc::Args::parse();
    let path = if cli.real {
        "input/01/real.txt"
    } else {
        "input/01/example.txt"
    };

    let mut calories: i32 = 0;
    let mut max_calories = [0, 0, 0];

    let input = std::fs::read_to_string(path)?;
    for line in input
        .lines()
        .map(|line| line.trim())
        .chain(std::iter::once(""))
    {
        if line.is_empty() {
            if max_calories[0] < calories {
                max_calories[0] = calories;
                max_calories.sort();
            }

            calories = 0;
            continue;
        }

        calories += line.parse::<i32>()?;
    }

    let total_calories: i32 = max_calories.into_iter().sum();

    if !cli.part_two {
        println!("{}", max_calories[2]);
    } else {
        println!("{total_calories}");
    }

    Ok(())
}
