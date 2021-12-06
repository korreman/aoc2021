pub fn run() {
    let input = std::fs::read_to_string("data/day6.txt").unwrap();

    // ----- Parsing -----
    let fish: Vec<u32> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    // ----- Computations -----
    let result1 = reproduce(fish.as_slice(), 80);
    let result2 = reproduce(fish.as_slice(), 256);

    // ----- Output -----
    println!("Day 6 - Part 1: {} - Part 2: {}", result1, result2);
}

fn reproduce(fish: &[u32], days: u32) -> u128 {
    let mut fish_counts: Vec<u128> = Vec::new();
    for i in 0..9 {
        let amount = fish.iter().filter(|&&f| f == i).count() as u128;
        fish_counts.push(amount);
    }
    for _ in 0..days {
        fish_counts.rotate_left(1);
        fish_counts[6] += *fish_counts.last().unwrap();
    }
    fish_counts.iter().map(|n| *n).sum()
}
