use std::cmp::Ordering;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;
use itertools::Itertools;

use crate::parser::parse_packet;

mod packet;
mod parser;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Cli {
    /// Input
    #[arg(value_name = "FILE")]
    #[arg(default_value = "data/sample.txt")]
    input: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    println!("args = {:?}", args);

    let data = BufReader::new(File::open(args.input)?)
        .lines()
        .filter_map(|x| x.ok())
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let lines = chunk.collect_vec();
            if lines.len() > 2 {
                assert!(lines[2].is_empty());
            }
            let left = parse_packet(&lines[0]);
            let right = parse_packet(&lines[1]);
            (left, right)
        })
        .collect_vec();

    println!("==> Solving part one...");
    let sum: usize = data
        .iter()
        .enumerate()
        .filter_map(|(i, (left, right))| {
            if Ord::cmp(left, right) == Ordering::Less {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum();
    println!("Sum of indices of pairs in the right order: {}", sum);

    println!("==> Solving part two...");
    let p2 = parse_packet("[[2]]");
    let p6 = parse_packet("[[6]]");
    let sorted = data
        .into_iter()
        .flat_map(|(left, right)| [left, right])
        .chain([p2.clone(), p6.clone()])
        .sorted()
        .collect_vec();
    // println!("Sorted packets:");
    // for packet in &sorted {
    //     println!("  {}", packet);
    // }
    let pos2 = sorted.iter().position(|p| p == &p2).unwrap() + 1;
    let pos6 = sorted.iter().position(|p| p == &p6).unwrap() + 1;
    println!("Index of {} is {}", p2, pos2);
    println!("Index of {} is {}", p6, pos6);
    println!("Decoder key: {}", pos2 * pos6);

    Ok(())
}
