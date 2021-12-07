pub fn run(input: &str) -> (u32, u32){
    // ----- Part 1 -----
    // Count the difference between the number of 0's and 1's in each bit.
    let mut balances: [i32; 12] = [0; 12];
    for line in input.lines() {
        for (c, f) in line.chars().zip(&mut balances) {
            match c {
                '0' => *f -= 1,
                '1' => *f += 1,
                _ => {}
            }
        }
    }

    // Generate gamma, one bit at a time.
    let mut gamma: u32 = 0;
    for f in &balances {
        gamma <<= 1;
        // NOTE: no spec for what to do if f == 0
        if *f > 0 {
            gamma |= 1;
        }
    }
    // epsilon is found by inverting the gamma (first 12 bits)
    let epsilon = (gamma ^ u32::MAX) & 0x00000fff;

    // ----- Part 2 -----
    let mut oxygen_set: Vec<&str> = input.lines().collect();
    let mut co2_set = oxygen_set.clone();
    filter_rating(&mut oxygen_set, true);
    filter_rating(&mut co2_set, false);
    let oxygen = u32::from_str_radix(oxygen_set[0], 2).unwrap();
    let co2 = u32::from_str_radix(co2_set[0], 2).unwrap();

    // ----- Output -----
    (epsilon * gamma, oxygen * co2)
}

fn filter_rating(set: &mut Vec<&str>, parity: bool) {
    for i in 0..12 {
        let mut balance: i32 = 0;
        for num in set.iter() {
            if num.chars().nth(i) == Some('1') {
                balance += 1
            } else {
                balance -= 1
            };
        }

        let filter_elem = if (balance >= 0) ^ parity { '1' } else { '0' };
        set.retain(|num| num.chars().nth(i) == Some(filter_elem));
        if set.len() == 1 {
            return;
        }
    }
}
