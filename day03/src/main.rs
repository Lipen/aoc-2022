use std::collections::HashSet;
use std::convert::TryFrom;
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

fn char_to_priority(c: char) -> u32 {
    if matches!(c, 'a'..='z') {
        (c as u8 - 'a' as u8) as u32 + 1
    } else if matches!(c, 'A'..='Z') {
        (c as u8 - 'A' as u8) as u32 + 27
    } else {
        panic!("Invalid character: {}", c);
    }
}

struct ChunkIter<'a, T, const N: usize> {
    data: &'a [T],
    index: usize,
}

impl<'a, T, const N: usize> Iterator for ChunkIter<'a, T, N> {
    type Item = &'a [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.data.len() {
            return None;
        }
        let chunk = <&[T; N]>::try_from(&self.data[self.index..self.index + N]).unwrap();
        self.index += N;
        Some(chunk)
    }
}

/// **Usage:**
///
/// ```
/// // &data: &[T]
/// for block in chunks::<_, 3>(&data) {
///     // block: [T; 3]
/// }
/// ```
fn chunks<T, const N: usize>(data: &[T]) -> ChunkIter<T, N> {
    assert_eq!(data.len() % N, 0);
    ChunkIter { data, index: 0 }
}

fn intersection(mut sets: Vec<HashSet<char>>) -> HashSet<char> {
    if sets.is_empty() {
        return HashSet::new();
    }

    if sets.len() == 1 {
        return sets.pop().unwrap();
    }

    let mut result = sets.pop().unwrap();
    result.retain(|item| sets.iter().all(|set| set.contains(item)));
    result
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    println!("args = {:?}", args);

    let path = args.input;

    let data = BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    println!("==> Solving part one...");
    let mut total_priority1 = 0;
    for line in &data {
        assert_eq!(line.len() % 2, 0);
        let (left, right) = line.split_at(line.len() / 2);
        let left_chars: HashSet<char> = left.iter().copied().collect();
        let right_chars: HashSet<char> = right.iter().copied().collect();
        let intersection = left_chars.intersection(&right_chars).copied().collect_vec();
        assert_eq!(intersection.len(), 1);
        let common = intersection[0];
        let priority = char_to_priority(common);
        total_priority1 += priority;
    }
    println!("Total priority: {}", total_priority1);

    println!("==> Solving part two...");
    let mut total_priority2 = 0;
    for [a, b, c] in chunks::<_, 3>(&data) {
        let a_chars: HashSet<char> = a.iter().copied().collect();
        let b_chars: HashSet<char> = b.iter().copied().collect();
        let c_chars: HashSet<char> = c.iter().copied().collect();
        let intersection = intersection(vec![a_chars, b_chars, c_chars]);
        assert_eq!(intersection.len(), 1);
        let common = intersection.into_iter().next().unwrap();
        let priority = char_to_priority(common);
        total_priority2 += priority;
    }
    println!("Total priority: {}", total_priority2);

    Ok(())
}
