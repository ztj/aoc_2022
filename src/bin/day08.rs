use std::fmt::{Display, Formatter};

use aoc::Parser;

struct Grid {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

/// Don't try to read from this before you've added at least one row, and don't add rows with differing lengths
/// otherwise you can expect panics to occur.
impl Grid {
    fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            data: Vec::new(),
        }
    }

    fn add_row(&mut self, row: &[u8]) {
        if self.width == 0 {
            self.width = row.len();
        } else if self.width != row.len() {
            panic!("Row length does not match grid width, you can't change the row length after the first row is added");
        }
        self.height += 1;
        self.data.extend_from_slice(row);
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.data[y * self.width + x]
    }

    fn visible_from_outside(&self) -> u64 {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                // Border
                if x == 0 || y == 0 || x == self.width - 1 || y == self.height - 1 {
                    count += 1;
                    continue;
                }
                let height = self.get(x, y);
                let mut west_blocked = false;
                let mut east_blocked = false;
                let mut north_blocked = false;
                let mut south_blocked = false;
                for west in 0..x {
                    if self.get(west, y) >= height {
                        west_blocked = true;
                        break;
                    }
                }
                for east in x + 1..self.width {
                    if self.get(east, y) >= height {
                        east_blocked = true;
                        break;
                    }
                }
                for north in 0..y {
                    if self.get(x, north) >= height {
                        north_blocked = true;
                        break;
                    }
                }
                for south in y + 1..self.height {
                    if self.get(x, south) >= height {
                        south_blocked = true;
                        break;
                    }
                }
                if !west_blocked || !east_blocked || !north_blocked || !south_blocked {
                    count += 1;
                }
            }
        }
        count
    }

    fn highest_scenic_score(&self) -> u64 {
        let mut top_score: u64 = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let height = self.get(x, y);
                let mut west_score: u64 = 0;
                let mut east_score: u64 = 0;
                let mut north_score: u64 = 0;
                let mut south_score: u64 = 0;
                for west in (0..x).rev() {
                    west_score += 1;
                    if self.get(west, y) >= height {
                        break;
                    }
                }
                for east in x + 1..self.width {
                    east_score += 1;
                    if self.get(east, y) >= height {
                        break;
                    }
                }
                for north in (0..y).rev() {
                    north_score += 1;
                    if self.get(x, north) >= height {
                        break;
                    }
                }
                for south in y + 1..self.height {
                    south_score += 1;
                    if self.get(x, south) >= height {
                        break;
                    }
                }
                top_score = top_score.max(west_score * east_score * north_score * south_score);
            }
        }
        top_score
    }
}

/// Good enough for AOC, but, could be more efficient
impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// I wrote this one before I decided to split up part 1 & 2 executions
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = aoc::Args::parse();
    let path = if cli.real {
        "input/08/real.txt"
    } else {
        "input/08/example.txt"
    };

    let input = std::fs::read_to_string(path)?;
    let mut grid = Grid::new();
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| (c as u8) - 48)
                .collect::<Vec<u8>>()
        })
        .for_each(|row| grid.add_row(&row));
    //println!("{}", grid);
    if !cli.part_two {
        println!("{}", grid.visible_from_outside());
    } else {
        println!("{}", grid.highest_scenic_score());
    }
    Ok(())
}
