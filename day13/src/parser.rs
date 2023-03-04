use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::packet::Packet;

#[derive(Parser)]
#[grammar = "packet.pest"]
pub struct PacketParser;

pub fn parse_packet(s: &str) -> Packet {
    let packet = PacketParser::parse(Rule::packet, s)
        .unwrap()
        .next()
        .unwrap();

    fn parse_packet(pair: Pair<Rule>) -> Packet {
        match pair.as_rule() {
            Rule::list => Packet::List(pair.into_inner().map(parse_packet).collect()),
            Rule::number => Packet::Num(pair.as_str().parse().unwrap()),
            _ => unreachable!(),
        }
    }

    parse_packet(packet)
}
