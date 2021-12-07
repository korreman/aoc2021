pub fn run(input: &str) -> (i32, i32) {
    // ----- Parsing -----
    let mut crabs: Vec<i32> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    // ----- Computations -----
    let result1 = part1(crabs.as_mut_slice());
    let result2 = part2(crabs.as_mut_slice());

    // ----- Output -----
    (result1, result2)
}

fn part1(crabs: &mut [i32]) -> i32 {
    crabs.sort();
    // the best position is just the mean
    let best_pos = crabs[crabs.len() / 2];
    let distance_sum: i32 = crabs.iter().map(|&x| (x - best_pos).abs()).sum();
    distance_sum
}

fn part2(crabs: &[i32]) -> i32 {
    let mut best_loss = i32::MAX;
    let mut best_pos: i32 = 0;

    // the best position this time is probably derivable here, but this was fast to code
    for p in 0..1 + *crabs.iter().max().unwrap() {
        let mut total_loss = 0;
        for &crab in crabs {
            let dist = (crab - p).abs();
            total_loss += triangular(dist + 1);
        }

        if total_loss < best_loss {
            best_loss = total_loss;
            best_pos = p;
        }
    }
    let distance_sum: i32 = crabs
        .iter()
        .map(|&x| triangular((x - best_pos).abs() + 1))
        .sum();
    distance_sum
}

fn triangular(n: i32) -> i32 {
    if n > 0 {
        n * (n - 1) / 2
    } else {
        0
    }
}
