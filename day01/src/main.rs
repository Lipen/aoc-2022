use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;

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

    let path = args.input;
    let lines = BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|x| x.ok());

    let mut data: Vec<u32> = Vec::new();
    let mut current: u32 = 0;
    for line in lines {
        if line.is_empty() {
            if current != 0 {
                data.push(current);
                current= 0;
            }
        } else {
            current += line.parse::<u32>()?;
        }
    }
    if current != 0 {
        data.push(current);
        current= 0;
    }
    assert_eq!(current, 0);

    println!("==> Solving part one...");
    let max = data.iter().max().unwrap();
    println!("Max: {}", max);

    println!("==> Solving part two...");
    let sorted = {
        let mut res = data.clone();
        res.sort();
        res.reverse();
        res
    };
    let top3 = sorted[0] + sorted[1] + sorted[2];
    println!("Sum of top 3: {}", top3);

    Ok(())
}
