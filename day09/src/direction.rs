use crate::point::Point;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn delta(self) -> Point {
        match self {
            Direction::Right => Point::new(1, 0),
            Direction::Left => Point::new(-1, 0),
            Direction::Up => Point::new(0, 1),
            Direction::Down => Point::new(0, -1),
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => panic!("Bad direction {:?}", c),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Right => write!(f, "R"),
            Direction::Left => write!(f, "L"),
            Direction::Up => write!(f, "U"),
            Direction::Down => write!(f, "D"),
        }
    }
}
