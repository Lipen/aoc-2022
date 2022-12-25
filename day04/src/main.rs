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

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    println!("args = {:?}", args);

    let path = args.input;

    let data = BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| {
            let xs = line.split(',')
                .map(|s| {
                    let ys = s.split('-').collect_vec();
                    assert_eq!(ys.len(), 2);
                    let a = ys[0].parse::<u32>().unwrap();
                    let b = ys[1].parse::<u32>().unwrap();
                    assert!(a <= b);
                    (a, b)
                })
                .collect_vec();
            assert_eq!(xs.len(), 2);
            let a = xs[0];
            let b = xs[1];
            (a, b)
        })
        .collect_vec();
    println!("Data size: {}", data.len());

    println!("==> Solving part one...");
    let mut count_contains = 0;
    for (a, b) in &data {
        if (a.0 <= b.0 && b.1 <= a.1) || (b.0 <= a.0 && a.1 <= b.1) {
            count_contains += 1;
        }
    }
    println!("Number of fully contained intervals: {}", count_contains);

    println!("==> Solving part two...");
    let mut count_overlap = 0;
    for (a, b) in &data {
        if (a.0 <= b.0 && b.0 <= a.1) || (b.0 <= a.0 && a.0 <= b.1) {
            count_overlap += 1;
        }
    }
    println!("Number of overlapped intervals: {}", count_overlap);

    Ok(())
}
