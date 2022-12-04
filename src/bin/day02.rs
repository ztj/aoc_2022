use std::{fmt::Display, str::FromStr};

use aoc::Parser;

const WIN: u64 = 6;
const DRAW: u64 = 3;
const LOSE: u64 = 0;

const ROCK: u64 = 1;
const PAPER: u64 = 2;
const SCISSORS: u64 = 3;

/// Error when parsing instruction value
#[derive(Debug)]
struct ParseInsErr(Option<char>);

impl Display for ParseInsErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to match input to enum value: {:?}", self.0)
    }
}

impl std::error::Error for ParseInsErr {}

/// Error when parsing round string
#[derive(Debug)]
enum ParseRoundErr {
    Split(String),
    Left(ParseInsErr),
    Right(ParseInsErr),
}

impl Display for ParseRoundErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse round: {:?}", self)
    }
}

impl std::error::Error for ParseRoundErr {}

/// Left instruction value
enum InsLeft {
    A,
    B,
    C,
}

impl FromStr for InsLeft {
    type Err = ParseInsErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(letter) = s.chars().next() {
            match letter {
                'A' => Ok(InsLeft::A),
                'B' => Ok(InsLeft::B),
                'C' => Ok(InsLeft::C),
                _ => Err(ParseInsErr(Some(letter))),
            }
        } else {
            Err(ParseInsErr(None))
        }
    }
}

/// Right instruction value
enum InsRight {
    X,
    Y,
    Z,
}

impl FromStr for InsRight {
    type Err = ParseInsErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(letter) = s.chars().next() {
            match letter {
                'X' => Ok(InsRight::X),
                'Y' => Ok(InsRight::Y),
                'Z' => Ok(InsRight::Z),
                _ => Err(ParseInsErr(Some(letter))),
            }
        } else {
            Err(ParseInsErr(None))
        }
    }
}

/// Instructions for one round in the strategy guide
struct Round(InsLeft, InsRight);

impl TryFrom<&str> for Round {
    type Error = ParseRoundErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(split) = value.trim().split_once(' ') {
            let left: InsLeft = split.0.parse().map_err(ParseRoundErr::Left)?;
            let right: InsRight = split.1.parse().map_err(ParseRoundErr::Right)?;
            Ok(Round(left, right))
        } else {
            Err(ParseRoundErr::Split(value.to_string()))
        }
    }
}

/// Interpret the strategy as (opponent move, my move) and assuming all went as planned, derive the score
/// A | X = ROCK, B | Y = PAPER, C | Z = SCISSORS
fn score_part1(round: Round) -> u64 {
    match round {
        Round(InsLeft::A, InsRight::X) => DRAW + ROCK,
        Round(InsLeft::B, InsRight::X) => LOSE + ROCK,
        Round(InsLeft::C, InsRight::X) => WIN + ROCK,
        Round(InsLeft::A, InsRight::Y) => WIN + PAPER,
        Round(InsLeft::B, InsRight::Y) => DRAW + PAPER,
        Round(InsLeft::C, InsRight::Y) => LOSE + PAPER,
        Round(InsLeft::A, InsRight::Z) => LOSE + SCISSORS,
        Round(InsLeft::B, InsRight::Z) => WIN + SCISSORS,
        Round(InsLeft::C, InsRight::Z) => DRAW + SCISSORS,
    }
}

/// Interpret the strategy as (opponent move, desired outcome) and assuming all went as planned, derive the score
/// A = ROCK, B = PAPER, C = SCISSORS
/// X = LOSE, Y = DRAW, Z = WIN
fn score_part2(round: Round) -> u64 {
    match round {
        Round(InsLeft::A, InsRight::X) => LOSE + SCISSORS,
        Round(InsLeft::B, InsRight::X) => LOSE + ROCK,
        Round(InsLeft::C, InsRight::X) => LOSE + PAPER,
        Round(InsLeft::A, InsRight::Y) => DRAW + ROCK,
        Round(InsLeft::B, InsRight::Y) => DRAW + PAPER,
        Round(InsLeft::C, InsRight::Y) => DRAW + SCISSORS,
        Round(InsLeft::A, InsRight::Z) => WIN + PAPER,
        Round(InsLeft::B, InsRight::Z) => WIN + SCISSORS,
        Round(InsLeft::C, InsRight::Z) => WIN + ROCK,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = aoc::Args::parse();
    let path = if cli.real {
        "input/02/real.txt"
    } else {
        "input/02/example.txt"
    };

    let score = |round| {
        if !cli.part_two {
            score_part1(round)
        } else {
            score_part2(round)
        }
    };

    let input = std::fs::read_to_string(path)?;
    let score = input
        .lines()
        .map(Round::try_from)
        .map(|round| round.map(score))
        .sum::<Result<u64, ParseRoundErr>>()?;
    println!("{score}");
    Ok(())
}
