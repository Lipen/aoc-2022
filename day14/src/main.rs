use std::cmp::{max, min};
use std::collections::HashSet;
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

    let data = BufReader::new(File::open(args.input)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| {
            line.split(" -> ")
                .map(|s| {
                    let (x, y) = s.split_once(',').unwrap();
                    let x = x.parse::<i32>().unwrap();
                    let y = y.parse::<i32>().unwrap();
                    (x, y)
                })
                .collect_vec()
        })
        .collect_vec();

    let mut map = HashSet::new();
    for formation in data.iter() {
        for (c1, c2) in formation.iter().tuple_windows() {
            // println!("{:?} -> {:?}", c1, c2);
            for x in min(c1.0, c2.0)..=max(c1.0, c2.0) {
                for y in min(c1.1, c2.1)..=max(c1.1, c2.1) {
                    map.insert((x, y));
                }
            }
        }
    }
    let map = map;
    let num_rocks = map.len();
    let max_y = map.iter().max_by_key(|p| p.1).unwrap().1;

    println!("==> Solving part one...");
    let mut occupied = map.clone();
    for _ in 0..10000 {
        let mut p = (500, 0);
        assert!(!map.contains(&p));
        while p.1 <= max_y {
            let down = (p.0, p.1 + 1);
            if !occupied.contains(&down) {
                p = down;
            } else {
                let down_left = (p.0 - 1, p.1 + 1);
                if !occupied.contains(&down_left) {
                    p = down_left;
                } else {
                    let down_right = (p.0 + 1, p.1 + 1);
                    if !occupied.contains(&down_right) {
                        p = down_right;
                    } else {
                        // println!("Sand settled at {:?}", p);
                        occupied.insert(p);
                        break;
                    }
                }
            }
        }
    }
    let num_sand =occupied.len() - num_rocks;
    println!("Units of sand: {}", num_sand);

    println!("==> Solving part two...");
    let mut occupied = map.clone();
    'outer: for _ in 0..100000 {
        let mut p = (500, 0);
        if map.contains(&p) {
            break;
        }
        while p.1 <= max_y {
            let down = (p.0, p.1 + 1);
            if !occupied.contains(&down) {
                p = down;
            } else {
                let down_left = (p.0 - 1, p.1 + 1);
                if !occupied.contains(&down_left) {
                    p = down_left;
                } else {
                    let down_right = (p.0 + 1, p.1 + 1);
                    if !occupied.contains(&down_right) {
                        p = down_right;
                    } else {
                        // println!("Sand settled at {:?}", p);
                        occupied.insert(p);
                        continue 'outer;
                    }
                }
            }
        }
        // println!("Sand settled at the floor {:?}", p);
        occupied.insert(p);
    }
    let num_sand = occupied.len() - num_rocks;
    println!("Units of sand: {}", num_sand);

    Ok(())
}
