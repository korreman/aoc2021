pub fn run(input: &str) -> (u64, u64) {
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
    (result1, result2)
}

fn reproduce(fish: &[u32], days: u32) -> u64 {
    let mut fish_counts = Vec::new();
    for i in 0..9 {
        let amount = fish.iter().filter(|&&f| f == i).count() as u64;
        fish_counts.push(amount);
    }
    for _ in 0..days {
        fish_counts.rotate_left(1);
        fish_counts[6] += *fish_counts.last().unwrap();
    }
    fish_counts.iter().map(|n| *n).sum()
}
