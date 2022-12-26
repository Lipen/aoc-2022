use std::fs::read_to_string;
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

fn all_different(s: &[u8]) -> bool {
    for i in 0..s.len() {
        for j in (i + 1)..s.len() {
            if s[i] == s[j] {
                return false;
            }
        }
    }
    return true;
}

fn solve(s: &str, k: usize) -> usize {
    for i in k..s.len() {
        if all_different(s[i - k..i].as_bytes()) {
            return i;
        }
    }
    panic!("Could not find")
}

fn solve_part_one(s: &str) -> usize {
    solve(s, 4)
}

fn solve_part_two(s: &str) -> usize {
    solve(s, 14)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    println!("args = {:?}", args);

    let s = read_to_string(args.input)?;
    println!("Input string length: {}", s.len());

    println!("==> Solving part one...");
    let ans1 = solve_part_one(&s);
    println!("Answer: {}", ans1);

    println!("==> Solving part two...");
    let ans2 = solve_part_two(&s);
    println!("Answer: {}", ans2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample1() {
        let s = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(7, solve_part_one(s));
    }

    #[test]
    fn part1_sample2() {
        let s = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(5, solve_part_one(s));
    }

    #[test]
    fn part1_sample3() {
        let s = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(6, solve_part_one(s));
    }

    #[test]
    fn part1_sample4() {
        let s = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(10, solve_part_one(s));
    }

    #[test]
    fn part1_sample5() {
        let s = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(11, solve_part_one(s));
    }

    #[test]
    fn part2_sample1() {
        let s = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(19, solve_part_two(s));
    }

    #[test]
    fn part2_sample2() {
        let s = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(23, solve_part_two(s));
    }

    #[test]
    fn part2_sample3() {
        let s = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(23, solve_part_two(s));
    }

    #[test]
    fn part2_sample4() {
        let s = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(29, solve_part_two(s));
    }

    #[test]
    fn part2_sample5() {
        let s = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(26, solve_part_two(s));
    }
}
