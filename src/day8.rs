// Not particularly proud of this one.
// I think making an array wrapper for digits was the wrong approach,
// and I should just've used strings all the way through.
// But hey, it brute forces the solution in like 30ms on my machine.
use itertools::Itertools;

pub fn run(input: &str) -> (u32, u32) {
    let entries: Vec<Entry> = input
        .lines()
        .map(|line| {
            let (ss, ds) = line.split_once('|').unwrap();
            let ss: Vec<Digit> = ss
                .split_ascii_whitespace()
                .map(|s| Digit::from_str(s))
                .collect();
            let ds: Vec<Digit> = ds
                .split_ascii_whitespace()
                .map(|d| Digit::from_str(d))
                .collect();
            let mut samples = [Digit::from_str(""); 10];
            let mut data = [Digit::from_str(""); 4];
            ss.into_iter()
                .zip(0..10)
                .for_each(|(digit, i)| samples[i] = digit);
            ds.into_iter()
                .zip(0..4)
                .for_each(|(digit, i)| data[i] = digit);
            Entry { samples, data }
        })
        .collect();

    let result1 = entries
        .iter()
        .flat_map(|entry| entry.data.iter())
        .filter(|digit| {
            let c = digit.count();
            c == 2 || c == 4 || c == 3 || c == 7
        })
        .count();

    let mut values: Vec<usize> = Vec::new();
    for entry in entries {
        for perm in Itertools::permutations(0..7, 7) {
            if entry
                .samples
                .iter()
                .all(|digit| DIGITS.contains(&digit.reorder(perm.as_slice())))
            {
                let mut value = entry.data[3].reorder(perm.as_slice()).read();
                value += entry.data[2].reorder(perm.as_slice()).read() * 10;
                value += entry.data[1].reorder(perm.as_slice()).read() * 100;
                value += entry.data[0].reorder(perm.as_slice()).read() * 1000;
                values.push(value);
                break;
            }
        }
    }
    let result2: usize = values.iter().sum();
    (result1 as u32, result2 as u32)
}

#[derive(Debug)]
struct Entry {
    samples: [Digit; 10],
    data: [Digit; 4],
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Digit {
    data: [bool; 7],
}

impl Digit {
    fn from_str(s: &str) -> Self {
        let mut data = [false; 7];
        for c in s.as_bytes() {
            data[(c - 0x61) as usize] = true;
        }
        Self { data }
    }

    fn count(&self) -> usize {
        self.data.iter().filter(|&&b| b == true).count()
    }

    fn reorder(self, order: &[usize]) -> Self {
        let mut new_data = [false; 7];
        for (&o, i) in order.iter().zip(0..7) {
            new_data[i] = self.data[o];
        }
        Self { data: new_data }
    }

    fn read(&self) -> usize {
        DIGITS.iter().position(|&d| d == *self).unwrap()
    }
}

const DIGITS: [Digit; 10] = [
    Digit { data: [true, true, true, false, true, true, true] },
    Digit { data: [false, false, true, false, false, true, false] },
    Digit { data: [true, false, true, true, true, false, true] },
    Digit { data: [true, false, true, true, false, true, true] },
    Digit { data: [false, true, true, true, false, true, false] },
    Digit { data: [true, true, false, true, false, true, true] },
    Digit { data: [true, true, false, true, true, true, true] },
    Digit { data: [true, false, true, false, false, true, false] },
    Digit { data: [true, true, true, true, true, true, true] },
    Digit { data: [true, true, true, true, false, true, true] },
];
