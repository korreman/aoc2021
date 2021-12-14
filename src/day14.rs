use itertools::Itertools;
use std::collections::HashMap;

pub fn run(input: &str) -> (u64, u64) {
    let (state, rulelist) = input.split_once("\n\n").unwrap();

    let mut rules: HashMap<(char, char), char> = HashMap::new();
    for line in rulelist.lines() {
        let (pair, res) = line.split_once(" -> ").unwrap();
        let t: Vec<char> = pair.chars().take(2).collect();

        let entry = (t[0], t[1]);
        let res = res.chars().next().unwrap();
        rules.insert(entry, res);
    }

    let result1 = task(state, &rules, 10);
    let result2 = task(state, &rules, 40);
    (result1, result2)
}

type Count = HashMap<(char, char), u64>;

fn task(starting_state: &str, rules: &HashMap<(char, char), char>, num_steps: u32) -> u64 {
    let start_vec: Vec<char> = starting_state.chars().collect();

    let mut state: Count = HashMap::new();
    for (&a, &b) in start_vec.iter().tuple_windows() {
        inc_map(&mut state, &(a, b), 1);
    }

    for _ in 0..num_steps {
        let mut new_state = HashMap::new();
        for (&(a, b), &count) in state.iter() {
            if let Some(&c) = rules.get(&(a, b)) {
                inc_map(&mut new_state, &(a, c), count);
                inc_map(&mut new_state, &(c, b), count);
            } else {
                inc_map(&mut new_state, &(a, b), count);
            }
        }
        state = new_state;
    }

    let mut counts = HashMap::new();
    counts.insert(starting_state.chars().next().unwrap(), 1);
    for (&(_, b), &count) in state.iter() {
        inc_map(&mut counts, &b, count);
    }

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn inc_map<H: std::hash::Hash + Eq + Copy>(map: &mut HashMap<H, u64>, pair: &H, count: u64) {
    if let Some(n) = map.get_mut(pair) {
        *n += count;
    } else {
        map.insert(*pair, count);
    }
}
