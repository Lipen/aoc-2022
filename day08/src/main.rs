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

fn calculate_visibility_matrix(data: &[Vec<u32>]) -> Vec<Vec<bool>> {
    let n = data.len();
    let m = data[0].len();
    let mut visible = vec![vec![false; m]; n];

    // Fill the edges:
    for i in 0..n {
        if i == 0 || i == n - 1 {
            for j in 0..m {
                visible[i][j] = true;
            }
        } else {
            visible[i][0] = true;
            visible[i][m - 1] = true;
        }
    }

    // Fill the interior:
    for i in 1..(n - 1) {
        for j in 1..(m - 1) {
            let v = data[i][j];
            // Check if all trees in the same row or column are shorter:
            if (0..j).all(|y| data[i][y] < v)
                || ((j + 1)..n).all(|y| data[i][y] < v)
                || (0..i).all(|x| data[x][j] < v)
                || ((i + 1)..m).all(|x| data[x][j] < v)
            {
                // If all trees are shorter in all directions, mark tree as visible:
                visible[i][j] = true;
            }
        }
    }

    visible
}

fn calculate_scenic_score_matrix(data: &[Vec<u32>]) -> Vec<Vec<usize>> {
    let n = data.len();
    let m = data[0].len();
    let mut scenic_score = vec![vec![0usize; m]; n];

    for i in 1..(n - 1) {
        for j in 1..(m - 1) {
            let v = data[i][j];
            let mut left = 0;
            for b in (0..j).rev() {
                left += 1;
                if data[i][b] >= v {
                    break;
                }
            }
            let mut right = 0;
            for b in (j + 1)..m {
                right += 1;
                if data[i][b] >= v {
                    break;
                }
            }
            let mut up = 0;
            for x in (0..i).rev() {
                up += 1;
                if data[x][j] >= v {
                    break;
                }
            }
            let mut down = 0;
            for x in (i + 1)..n {
                down += 1;
                if data[x][j] >= v {
                    break;
                }
            }
            scenic_score[i][j] = left * right * up * down;
            // println!("[i={}][j={}]: score={}, left={}, right={}, up={}, down={}", i, j, scenic_score[i][j], left, right, up, down);
        }
    }

    scenic_score
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    println!("args = {:?}", args);

    let data = BufReader::new(File::open(args.input)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    // `data` is `n`x`m` matrix
    let n = data.len();
    let m = data[0].len();

    println!("Data:");
    for i in 0..n {
        for j in 0..m {
            print!("{}", data[i][j]);
        }
        println!();
    }

    println!("==> Solving part one...");
    let visible = calculate_visibility_matrix(&data);
    println!("Visibility matrix:");
    for i in 0..n {
        for j in 0..m {
            if visible[i][j] {
                print!("1");
            } else {
                print!("0");
            }
        }
        println!();
    }
    let mut total_visible = 0;
    for i in 0..n {
        for j in 0..m {
            if visible[i][j] {
                total_visible += 1;
            }
        }
    }
    println!("Total visible: {}", total_visible);

    println!("==> Solving part two...");
    let scenic_score = calculate_scenic_score_matrix(&data);
    println!("Scenic score:");
    for i in 0..n {
        for j in 0..m {
            print!("{} ", scenic_score[i][j]);
        }
        println!();
    }
    let max_scenic_score = scenic_score
        .iter()
        .map(|row| row.iter().copied().max().unwrap())
        .max()
        .unwrap();
    println!("Max scenic score: {}", max_scenic_score);

    println!();
    println!("Answer for the first part is {}", total_visible);
    println!("Answer for the second part is {}", max_scenic_score);

    Ok(())
}
