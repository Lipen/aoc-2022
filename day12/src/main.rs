use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Cli {
    /// Input
    #[arg(value_name = "FILE")]
    #[arg(default_value = "data/sample.txt")]
    input: PathBuf,

    /// Part
    #[arg(value_enum, short, long)]
    #[arg(default_value = "both")]
    part: Part,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
enum Part {
    One,
    Two,
    Both,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    println!("args = {:?}", args);

    let mut start = (0, 0);
    let mut end = (0, 0);
    let data = BufReader::new(File::open(args.input)?)
        .lines()
        .filter_map(|x| x.ok())
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    match c {
                        'S' => {
                            start = (i, j);
                            // Lowest elevation:
                            0
                        }
                        'E' => {
                            end = (i, j);
                            // Highest elevation:
                            ('z' as u8) - ('a' as u8)
                        }
                        _ => (c as u8) - ('a' as u8),
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    // println!("start = {:?}", start);
    // println!("end = {:?}", end);
    // println!("data:");
    // for i in 0..data.len() {
    //     println!(
    //         "{}",
    //         itertools::join(data[i].iter().map(|x| (x + 'a' as u8) as char), "")
    //     )
    // }

    let mut graph = Graph::new();
    let mut grid: Vec<Vec<NodeIndex>> = Vec::new();
    for i in 0..data.len() {
        grid.push(Vec::new());
        for j in 0..data[i].len() {
            let cur = graph.add_node((i, j));
            grid[i].push(cur);
            let cur_level = data[i][j];
            if i > 0 {
                let prev = grid[i - 1][j];
                let prev_level = data[i - 1][j];
                if cur_level <= prev_level + 1 {
                    graph.add_edge(prev, cur, ());
                }
                if prev_level <= cur_level + 1 {
                    graph.add_edge(cur, prev, ());
                }
            }
            if j > 0 {
                let prev = grid[i][j - 1];
                let prev_level = data[i][j - 1];
                if cur_level <= prev_level + 1 {
                    graph.add_edge(prev, cur, ());
                }
                if prev_level <= cur_level + 1 {
                    graph.add_edge(cur, prev, ());
                }
            }
        }
    }
    let start = grid[start.0][start.1];
    let end = grid[end.0][end.1];

    if matches!(args.part, Part::One | Part::Both) {
        println!("==> Solving part one...");
        let result = dijkstra(&graph, start, Some(end), |_| 1);
        println!(
            "Shortest path from {:?} to {:?} is {}",
            start, end, result[&end]
        );
    }

    if matches!(args.part, Part::Two | Part::Both) {
        println!("==> Solving part two...");
        graph.reverse();
        let result = dijkstra(&graph, end, None, |_| 1);
        let (&v, &dist) = result
            .iter()
            .filter(|(v, dist)| {
                let &(i, j) = graph.node_weight(**v).unwrap();
                data[i][j] == 0
            })
            .min_by_key(|(_, dist)| **dist)
            .unwrap();
        println!("Shortest path from {:?} to {:?} is {}", v, end, dist,);
    }

    Ok(())
}
