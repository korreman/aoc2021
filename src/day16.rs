use nom::{bits::{bits, complete::*}, IResult};
use nom::{sequence::preceded};

pub fn run(input: &str) -> (u64, u64) {
    let inputs = input.lines().map(|line| hex::decode(line).unwrap());
    for i in inputs {
        println!("{:?}", i);
    }

    let result1 = 0;
    let result2 = 0;
    (result1, result2)
}

fn parse_literal(i: &[u8]) -> IResult<&[u8], usize> {
    take(5)(bits(i))
}

#[derive(Debug)]
enum Packet {
    Literal(u32),
    Operator {
        version: u32,
        packets: Box<[Packet]>,
    },
}

impl Packet {
    fn version_sum(&self) -> u32 {
        match self {
            Packet::Literal(_) => 0,
            Packet::Operator { version: n, packets } => {
                *n + packets.iter().map(|p| p.version_sum()).sum::<u32>()
            }
        }
    }
}
