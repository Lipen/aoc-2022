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

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Noop,
    Add(i32),
}

fn parse_instruction(s: &str) -> Instruction {
    match s {
        "noop" => Instruction::Noop,
        _ if s.starts_with("addx ") => {
            let parts = s.split_whitespace().collect_vec();
            assert_eq!(parts.len(), 2);
            let n = parts[1].parse::<i32>().unwrap();
            Instruction::Add(n)
        }
        _ => panic!("Could not parse instruction from {:?}", s),
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    println!("args = {:?}", args);

    let data = BufReader::new(File::open(args.input)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| parse_instruction(&line))
        .collect_vec();

    println!("Data length: {}", data.len());

    println!("==> Solving part one...");
    let mut x = 1;
    // Note: state[i] represents the value of register *during* i-th cycle.
    // Also note that 'cycles' in the problem statement are 1-based, but 'state' is 0-based.
    let mut state = vec![x];
    for item in &data {
        match item {
            Instruction::Noop => {
                state.push(x);
                // println!("noop -> state[i = {}] = {}", state.len()-1, x);
            }
            Instruction::Add(n) => {
                state.push(x);
                // println!("add({}) -> state[i = {}] = {}", n, state.len()-1, x);
                x += n;
                state.push(x);
                // println!("add({}) -> state[i = {}] = {}", n,state.len()-1, x);
            }
        }
    }
    // println!("State length: {}", state.len());
    // for (i, &value) in state.iter().enumerate() {
    //     println!("state[i = {}] = {}", i, value);
    // }
    let sum: i32 = [20, 60, 100, 140, 180, 220]
        .map(|c| {
            // println!("cycle {}: state is {}", c, state[c]);
            c as i32 * state[c - 1]
        })
        .into_iter()
        .sum();
    println!("Part one: {}", sum);

    println!("==> Solving part two...");
    for (i, &x) in state.iter().enumerate() {
        let col = (i % 40) as i32;
        // if col == x - 1 || col == x || col == x + 1 {
        if ((x - 1)..=(x + 1)).contains(&col) {
            print!("#");
        } else {
            print!(".");
        }
        if col == 39 {
            println!();
        }
    }

    Ok(())
}
