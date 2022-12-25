use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;
use itertools::Itertools;
use once_cell_regex::regex;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Cli {
    /// Input
    #[arg(value_name = "FILE")]
    #[arg(default_value = "data/sample.txt")]
    input: PathBuf,
}

fn parse_stacks(lines: &[String]) -> Vec<VecDeque<char>> {
    let n = lines
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut stacks = vec![VecDeque::new(); n];
    for j in (0..lines.len() - 1).rev() {
        let line = &lines[j];
        for i in 0..n {
            let pos = 1 + 4 * i;
            if pos < line.len() {
                let c = line.as_bytes()[pos] as char;
                if c != ' ' {
                    stacks[i].push_back(c);
                }
            }
        }
    }
    stacks
}

fn parse_instruction(line: &str) -> (usize, usize, usize) {
    let re = regex!(r"^move (\d+) from (\d+) to (\d+)$");
    let captures = re.captures(line).unwrap();
    let n = captures[1].parse::<usize>().unwrap();
    let from = captures[2].parse::<usize>().unwrap();
    let to = captures[3].parse::<usize>().unwrap();
    (n, from, to)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    println!("args = {:?}", args);

    let lines = BufReader::new(File::open(args.input)?)
        .lines()
        .filter_map(|x| x.ok())
        .collect_vec();

    let i = lines.iter().position(|line| line.is_empty()).unwrap();
    let stacks = parse_stacks(&lines[..i]);
    let instructions = lines[i + 1..]
        .iter()
        .map(|line| parse_instruction(line.as_str()))
        .collect_vec();

    println!("==> Solving part one...");
    {
        let mut state = stacks.clone();
        for &(n, from, to) in &instructions {
            for _ in 0..n {
                let elem = state[from - 1].pop_back().unwrap();
                state[to - 1].push_back(elem);
            }
        }
        let s: String = state
            .iter()
            .map(|x| x.back().unwrap())
            .collect();
        println!("Top of stacks: {}", s);
    }

    println!("==> Solving part two...");
    {
        let mut state = stacks.clone();
        for &(n, from, to) in &instructions {
            let mut tmp = VecDeque::new();
            for _ in 0..n {
                let elem = state[from - 1].pop_back().unwrap();
                tmp.push_back(elem);
            }
            while let Some(elem) = tmp.pop_back() {
                state[to - 1].push_back(elem);
            }
        }
        let s: String = state
            .iter()
            .map(|x| x.back().unwrap())
            .collect();
        println!("Top of stacks: {}", s);
    }

    Ok(())
}
