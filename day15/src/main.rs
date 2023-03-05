use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;
use itertools::Itertools;
use lazy_regex::regex_captures;

// Note:
//   On sample:
//     cargo run -- data/sample.txt -r 10 -m 20
//   On input:
//     cargo run -- data/input.txt -r 2000000 -m 4000000
//     (or just `cargo r`)

#[derive(Parser, Debug)]
#[command(author, version)]
struct Cli {
    /// Input
    #[arg(value_name = "FILE")]
    #[arg(default_value = "data/input.txt")]
    input: PathBuf,

    /// Row
    #[arg(short, long)]
    #[arg(default_value_t = 2_000_000)]
    row: i32,

    /// Max
    #[arg(short, long)]
    #[arg(default_value_t = 4_000_000)]
    max: i32,
}

/// Computes the Manhattan distance between two points.
fn manhattan((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> u32 {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

#[derive(Debug)]
struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
    radius: u32,
}

impl Sensor {
    fn new(pos: (i32, i32), beacon: (i32, i32)) -> Self {
        Sensor {
            pos,
            beacon,
            radius: manhattan(pos, beacon),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Line {
    rising: bool,
    vertical: i32,
}

impl Line {
    fn intersect(&self, other: &Line) -> (i32, i32) {
        assert_ne!(self.rising, other.rising);
        if self.rising {
            // y =  x + q1
            // y = -x + q2
            // ~> 0 = 2x + q1-q2
            // ~~> x = (q2-q1) / 2
            // ~~> y = x + q1
            let x = (other.vertical - self.vertical) / 2;
            let y = x + self.vertical;
            (x, y)
        } else {
            // y = -x + q1
            // y =  x + q2
            // ~> 0 = 2x + q2-q1
            // ~~> x = (q1-q2) / 2
            // ~~> y = x + q2
            let x = (self.vertical - other.vertical) / 2;
            let y = x + other.vertical;
            (x, y)
        }
    }
}

fn is_free(point: (i32, i32), sensors: &[Sensor]) -> bool {
    for sensor in sensors {
        if manhattan(sensor.pos, point) <= sensor.radius {
            return false;
        }
    }
    true
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    println!("args = {:?}", args);

    let data = BufReader::new(File::open(args.input)?)
        .lines()
        .filter_map(|x| x.ok())
        .map(|line| {
            let (_, xs, ys, xb, yb) = regex_captures!(
                r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
                &line
            )
            .unwrap();
            let xs = xs.parse::<i32>().unwrap();
            let ys = ys.parse::<i32>().unwrap();
            let xb = xb.parse::<i32>().unwrap();
            let yb = yb.parse::<i32>().unwrap();
            let sensor = (xs, ys);
            let beacon = (xb, yb);
            Sensor::new(sensor, beacon)
        })
        .collect_vec();

    println!("==> Solving part one...");
    let min_x = data
        .iter()
        .map(|sensor| sensor.pos.0 - sensor.radius as i32)
        .min()
        .unwrap();
    let max_x = data
        .iter()
        .map(|sensor| sensor.pos.0 + sensor.radius as i32)
        .max()
        .unwrap();
    let intervals = data
        .iter()
        .filter_map(|sensor| {
            let dist_row = sensor.pos.1.abs_diff(args.row);
            (sensor.radius >= dist_row).then(|| {
                let d = (sensor.radius - dist_row) as i32;
                (sensor.pos.0 - d, sensor.pos.0 + d)
            })
        })
        .collect_vec();
    let count_union: usize = intervals
        .iter()
        .copied()
        .sorted_unstable_by_key(|interval| interval.0)
        .coalesce(|prev, next| {
            if next.0 <= prev.1 {
                Ok((prev.0, max(prev.1, next.1)))
            } else {
                Err((prev, next))
            }
        })
        .map(|(x, y)| (y - x + 1) as usize)
        .sum();
    let count_beacons: usize = data
        .iter()
        .map(|sensor| sensor.beacon)
        .filter(|&(xb, yb)| xb >= min_x && xb <= max_x && yb == args.row)
        .unique()
        .count();
    println!(
        "On the row y={}, there are {} positions that cannot contain a beacon.",
        args.row,
        count_union - count_beacons,
    );

    // Credits for the second part:
    //   https://github.com/BuonHobo/advent-of-code/blob/master/2022/15/Alex/second.py
    println!("==> Solving part two...");
    let mut lines = Vec::new();
    for sensor in data.iter() {
        let (x, y) = sensor.pos;
        let r = sensor.radius as i32;

        // top rising:
        //   y = x + q + r + 1
        //   ~> q = y - x - r - 1
        lines.push(Line {
            rising: true,
            vertical: y - x - r - 1,
        });

        // top descending:
        //   y = -x + q + radius + 1
        //   ~> q = y + x - radius - 1
        lines.push(Line {
            rising: false,
            vertical: y + x - r - 1,
        });

        // bot rising:
        //   y = x + q - radius - 1
        //   ~> q = y - x + radius + 1
        lines.push(Line {
            rising: true,
            vertical: y - x + r + 1,
        });

        // bot descending:
        //   y = -x + q - radius - 1
        //   ~> q = y + x + radius + 1
        lines.push(Line {
            rising: false,
            vertical: y + x + r + 1,
        });
    }

    let mut counted_lines = lines.into_iter().counts();
    counted_lines.retain(|_, count| *count >= 2);
    let (rising_lines, descending_lines): (Vec<_>, Vec<_>) =
        counted_lines.into_keys().partition(|line| line.rising);
    // println!("Total rising lines: {}", rising_lines.len());
    // println!("Total descending lines: {}", descending_lines.len());

    let mut intersections = Vec::new();
    for rising in &rising_lines {
        for descending in &descending_lines {
            intersections.push(rising.intersect(descending))
        }
    }
    // println!("Total intersections: {}", intersections.len());

    for point in intersections {
        if (0..=args.max).contains(&point.0)
            && (0..=args.max).contains(&point.1)
            && is_free(point, &data)
        {
            println!("Distress beacon is at {:?}", point);
            println!(
                "Tuning frequency is {}",
                (point.0 as i128 * 4_000_000) + point.1 as i128
            );
            break;
        }
    }

    Ok(())
}
