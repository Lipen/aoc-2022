use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use itertools::Itertools;
use lazy_regex::{regex_captures, regex_is_match};
use log::{debug, info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

// Note:
//   Part one: cargo run -- data/input.txt -p one
//   Part two: cargo run -- data/input.txt -p two -r 10000

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

    /// Verbose mode
    #[arg(short, long)]
    verbose: bool,

    /// Number of rounds
    #[arg(short, long)]
    #[arg(default_value_t = 20)]
    rounds: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Part {
    One,
    Two,
    Both,
}

type N = u64;

#[derive(Debug, Clone)]
struct Monkey {
    items: RefCell<VecDeque<N>>,
    op: Operation,
    factor: N,
    monkeys: (usize, usize),
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(N),
    Mul(N),
    Square,
}

fn solve(monkeys: Vec<Monkey>, rounds: usize, worry_fn: impl Fn(N) -> N) -> u64 {
    let mut inspected: HashMap<usize, u64> = HashMap::new();

    for round in 1..=rounds {
        for id in 0..monkeys.len() {
            let m = &monkeys[id];
            debug!("Monkey {}:", id);
            while let Some(mut worry) = m.items.borrow_mut().pop_front() {
                debug!("  Monkey inspects an item with a worry level of {}.", worry);
                *inspected.entry(id).or_insert(0) += 1;

                worry = match m.op {
                    Operation::Add(rhs) => {
                        let new = worry + rhs;
                        debug!("    Worry level increased by {} to {}.", rhs, new);
                        new
                    }
                    Operation::Mul(rhs) => {
                        let new = worry * rhs;
                        debug!("    Worry level is multiplied by {} to {}.", rhs, new);
                        new
                    }
                    Operation::Square => {
                        let new = worry * worry;
                        debug!("    Worry level is multiplied by itself to {}.", new);
                        new
                    }
                };

                worry = worry_fn(worry);

                let other = if worry % m.factor == 0 {
                    debug!("    Current worry level is divisible by {}.", m.factor);
                    m.monkeys.0
                } else {
                    debug!("    Current worry level is not divisible by {}.", m.factor);
                    m.monkeys.1
                };
                monkeys[other].items.borrow_mut().push_back(worry);
                debug!(
                    "    Item with worry level {} is thrown to monkey {}.",
                    worry, other
                );
            }
        }
        // debug!(
        //     "After round {}, the monkeys are holding items with these worry levels:",
        //     round
        // );
        // for (id, m) in monkeys.iter().enumerate() {
        //     debug!("  Monkey {}: {}", id, join(m.items.borrow().iter(), ", "));
        // }
        if round == 1 || round == 20 || round % 1000 == 0 {
            debug!("== After round {} ==", round);
            for id in 0..monkeys.len() {
                debug!("  Monkey {} inspected {} items", id, inspected[&id]);
            }
        }
    }

    let top = inspected
        .values()
        .copied()
        .sorted_by_key(|&x| -(x as i64))
        .collect_vec();
    top[0] * top[1]
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    println!("args = {:?}", args);

    if args.verbose {
        TermLogger::init(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )?;
    } else {
        TermLogger::init(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )?;
    }

    let data = BufReader::new(File::open(args.input)?)
        .lines()
        .filter_map(|x| x.ok())
        .filter(|line| !line.is_empty())
        .chunks(6)
        .into_iter()
        .map(|chunk| {
            let block = chunk.collect_vec();
            assert_eq!(block.len(), 6);

            assert!(regex_is_match!(r"Monkey (\d):", &block[0]));
            // let (_, id_str) = regex_captures!(r"Monkey (\d):", &block[0]).unwrap();
            // let id = id_str.parse::<usize>().unwrap();
            // println!("id = {:?}", id);
            // assert_eq!(id, monkeys.len());

            let (_, items_str) =
                regex_captures!(r"  Starting items: ((?:\d+)(?:, \d+)*)", &block[1]).unwrap();
            let items = items_str
                .split(", ")
                .map(|s| s.parse::<N>().unwrap())
                .collect_vec();

            let (_, op_str, rhs_str) =
                regex_captures!(r"  Operation: new = old ([+*]) (\d+|old)", &block[2]).unwrap();
            let op = if rhs_str == "old" {
                assert_eq!(op_str, "*");
                Operation::Square
            } else {
                let rhs = rhs_str.parse::<N>().unwrap();
                match op_str {
                    "+" => Operation::Add(rhs),
                    "*" => Operation::Mul(rhs),
                    _ => panic!("Bad op '{}'", op_str),
                }
            };

            let (_, factor_str) =
                regex_captures!(r"  Test: divisible by (\d+)", &block[3]).unwrap();
            let factor = factor_str.parse::<N>().unwrap();

            let (_, if_true_str) =
                regex_captures!(r"    If true: throw to monkey (\d+)", &block[4]).unwrap();
            let if_true = if_true_str.parse::<usize>().unwrap();

            let (_, if_false_str) =
                regex_captures!(r"    If false: throw to monkey (\d+)", &block[5]).unwrap();
            let if_false = if_false_str.parse::<usize>().unwrap();

            Monkey {
                items: RefCell::new(VecDeque::from(items)),
                op,
                factor,
                monkeys: (if_true, if_false),
            }
        })
        .collect_vec();

    if matches!(args.part, Part::One | Part::Both) {
        info!("==> Solving part one...");
        let res1 = solve(data.clone(), args.rounds, |x| {
            let new = x / 3;
            debug!(
                "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
                new
            );
            new
        });
        info!("Part one: {}", res1);
    }

    if matches!(args.part, Part::Two | Part::Both) {
        info!("==> Solving part two...");
        let modulus: N = data.iter().map(|m| m.factor).product();
        let res2 = solve(data.clone(), args.rounds, |x| x % modulus);
        info!("Part two: {}", res2);
    }

    Ok(())
}
