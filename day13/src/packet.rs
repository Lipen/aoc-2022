use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter::zip;

use itertools::join;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Packet {
    Num(i32),
    List(Vec<Self>),
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Num(x) => write!(f, "{}", x),
            Packet::List(xs) => write!(f, "[{}]", join(xs, ",")),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Num(left), Packet::Num(right)) => Ord::cmp(left, right),
            (Packet::List(left), Packet::List(right)) => compare_lists(left, right),
            (Packet::Num(left), Packet::List(right)) => compare_lists(&[Packet::Num(*left)], right),
            (Packet::List(left), Packet::Num(right)) => compare_lists(left, &[Packet::Num(*right)]),
        }
    }
}

fn compare_lists(left: &[Packet], right: &[Packet]) -> Ordering {
    for (a, b) in zip(left, right) {
        match Ord::cmp(a, b) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => {}
        }
    }
    Ord::cmp(&left.len(), &right.len())
}
