use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Cli {
    /// Input
    #[arg(value_name = "FILE")]
    #[arg(default_value = "data/sample.txt")]
    input: PathBuf,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Item {
    Rock,
    Paper,
    Scissors,
}

impl Item {
    fn score(&self) -> u32 {
        match self {
            Item::Rock => 1,
            Item::Paper => 2,
            Item::Scissors => 3,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
}

fn play(a: Item, b: Item) -> Outcome {
    use Item::*;
    use Outcome::*;
    match (a, b) {
        (Rock, Rock) => Draw,
        (Rock, Paper) => Win,
        (Rock, Scissors) => Loss,
        (Paper, Rock) => Loss,
        (Paper, Paper) => Draw,
        (Paper, Scissors) => Win,
        (Scissors, Rock) => Win,
        (Scissors, Paper) => Loss,
        (Scissors, Scissors) => Draw,
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    println!("args = {:?}", args);

    let path = args.input;
    let lines = BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|x| x.ok());

    let data = lines.map(|line| {
        let chars = line.chars().collect_vec();
        assert_eq!(chars.len(), 3);
        assert_eq!(chars[1], ' ');
        let a = chars[0];
        let b = chars[2];
        (a, b)
    }).collect_vec();

    println!("==> Solving part one...");
    let score1: u32 = data.iter().map(|(a, b)| {
        let opponent = match a {
            'A' => Item::Rock,
            'B' => Item::Paper,
            'C' => Item::Scissors,
            _ => panic!("Bad opponent move {:?}", a),
        };
        let answer = match b {
            'X' => Item::Rock,
            'Y' => Item::Paper,
            'Z' => Item::Scissors,
            _ => panic!("Bad answer move {:?}", b),
        };
        answer.score() + play(opponent, answer).score()
    }).sum();
    println!("Score: {}", score1);

    println!("==> Solving part two...");
    let score2: u32 = data.iter().map(|(a, b)| {
        let opponent = match a {
            'A' => Item::Rock,
            'B' => Item::Paper,
            'C' => Item::Scissors,
            _ => panic!("Bad opponent move {:?}", a),
        };
        let answer = match b {
            // Need to lose:
            'X' => match opponent {
                Item::Rock => Item::Scissors,
                Item::Paper => Item::Rock,
                Item::Scissors => Item::Paper,
            },

            // Need to draw:
            'Y' => opponent,

            // Need to win:
            'Z' => match opponent {
                Item::Rock => Item::Paper,
                Item::Paper => Item::Scissors,
                Item::Scissors => Item::Rock,
            },

            _ => panic!("Bad answer move {:?}", b),
        };
        answer.score() + play(opponent, answer).score()
    }).sum();
    println!("Score: {}", score2);

    Ok(())
}
