use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;
use itertools::Itertools;

use crate::direction::Direction;
use crate::point::Point;

mod direction;
mod point;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Cli {
    /// Input
    #[arg(value_name = "FILE")]
    #[arg(default_value = "data/sample.txt")]
    input: PathBuf,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn solve_part_one(data: &[(Direction, usize)], verbose: u8) -> usize {
    // The origin of the grid (0,0) is in the lower left corner:
    //  - The X axis is pointing right.
    //  - The Y axis is pointing up.
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);
    let mut visited = HashSet::new();
    visited.insert(tail);
    for &(dir, n) in data {
        let delta = dir.delta();
        if verbose > 0 {
            print!("== {} {} ==\n\n", dir, n);
        }
        for _ in 0..n {
            let orig = head;
            head += delta;
            // Fancy observation: when 'head' is far away from 'tail',
            //   the tail must move diagonally, as specified in the problem statement.
            // In fact, the diagonal position is exactly the original head position,
            //   i.e. the position, where the head was before its movement.
            if (head.x - tail.x).abs() > 1 || (head.y - tail.y).abs() > 1 {
                tail = orig;
            }
            visited.insert(tail);
            if verbose == 1 {
                print_state1(head, tail, &visited);
            }
        }
        if verbose >= 2 {
            print_state1(head, tail, &visited);
        }
    }
    visited.len()
}

fn solve_part_two(data: &[(Direction, usize)], length: usize, verbose: u8) -> usize {
    let mut rope = vec![Point::new(0, 0); length];
    let mut visited = HashSet::new();
    visited.insert(Point::new(0, 0));
    if verbose > 0 {
        print_state2(&rope, &visited);
    }

    for &(dir, n) in data {
        let delta = dir.delta();
        if verbose > 0 {
            print!(
                "== {} {} ==\n\n",
                match (delta.x, delta.y) {
                    (1, 0) => "R",
                    (-1, 0) => "L",
                    (0, 1) => "U",
                    (0, -1) => "D",
                    _ => unreachable!(),
                },
                n
            );
        }
        for _ in 0..n {
            rope[0] += delta;
            for i in 1..length {
                let head = rope[i - 1];
                let tail = rope[i];
                let d = head - tail;
                if d.x.abs() > 1 || d.y.abs() > 1 {
                    rope[i] = tail + d.clamp_unit();
                } else {
                    break;
                }
            }
            visited.insert(*rope.last().unwrap());
            if verbose >= 2 {
                print_state2(&rope, &visited);
            }
        }
        if verbose == 1 {
            print_state2(&rope, &visited);
        }
    }

    visited.len()
}

fn print_state1(head: Point, tail: Point, visited: &HashSet<Point>) {
    let x_min = visited
        .iter()
        .map(|p| p.x)
        .chain(vec![0, head.x, tail.x])
        .min()
        .unwrap();
    let x_max = visited
        .iter()
        .map(|p| p.x)
        .chain(vec![0, head.x, tail.x])
        .max()
        .unwrap();
    let y_min = visited
        .iter()
        .map(|p| p.y)
        .chain(vec![0, head.y, tail.y])
        .min()
        .unwrap();
    let y_max = visited
        .iter()
        .map(|p| p.y)
        .chain(vec![0, head.y, tail.y])
        .max()
        .unwrap();
    for y in ((y_min - 1)..=(y_max + 1)).rev() {
        for x in (x_min - 1)..=(x_max + 1) {
            let p = Point::new(x, y);
            if head == p {
                print!("H");
            } else if tail == p {
                print!("T");
            } else if visited.contains(&p) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn print_state2(rope: &[Point], visited: &HashSet<Point>) {
    let x_min = visited
        .iter()
        .map(|p| p.x)
        .chain(rope.iter().map(|p| p.x))
        .min()
        .unwrap()
        .min(0);
    let x_max = visited
        .iter()
        .map(|p| p.x)
        .chain(rope.iter().map(|p| p.x))
        .max()
        .unwrap()
        .max(0);
    let y_min = visited
        .iter()
        .map(|p| p.y)
        .chain(rope.iter().map(|p| p.y))
        .min()
        .unwrap()
        .min(0);
    let y_max = visited
        .iter()
        .map(|p| p.y)
        .chain(rope.iter().map(|p| p.y))
        .max()
        .unwrap()
        .max(0);
    for y in ((y_min - 1)..=(y_max + 1)).rev() {
        for x in (x_min - 1)..=(x_max + 1) {
            let p = Point::new(x, y);
            let mut done = false;
            for i in 0..rope.len() {
                if rope[i] == p {
                    if i == 0 {
                        print!("H");
                    } else {
                        print!("{}", i);
                    }
                    done = true;
                    break;
                }
            }
            if !done {
                if visited.contains(&p) {
                    print!("#");
                } else if x == 0 && y == 0 {
                    print!("s");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
    println!();
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    println!("args = {:?}", args);

    let path = args.input;
    let verbose = args.verbose;

    let data = BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| {
            let parts = line.split_whitespace().collect_vec();
            assert_eq!(parts.len(), 2);
            let dir: Direction = parts[0].chars().next().unwrap().into();
            let n = parts[1].parse::<usize>().unwrap();
            (dir, n)
        })
        .collect_vec();

    if verbose >= 2 {
        println!("Data:");
        for &(dir, n) in &data {
            println!("{} {}", dir, n);
        }
    }

    println!("==> Solving part one...");
    let ans1 = solve_part_one(&data, verbose);
    println!("Total visited by tail: {}", ans1);

    println!("==> Solving part two...");
    let ans2 = solve_part_two(&data, 10, verbose);
    println!("Total visited by 10-length rope tail: {}", ans2);

    Ok(())
}
