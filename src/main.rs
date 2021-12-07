use std::time::Instant;

mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    run(day3::run, "data/day3.txt", "Day 3");
    run(day4::run, "data/day4.txt", "Day 4");
    run(day5::run, "data/day5.txt", "Day 5");
    run(day6::run, "data/day6.txt", "Day 6");
    run(day7::run, "data/day7.txt", "Day 7");
}

fn run<R: std::fmt::Display, T: Fn(&str) -> (R, R)>(task: T, input_path: &str, name: &str) {
    let input = std::fs::read_to_string(input_path).unwrap();

    let start = Instant::now();
    let (res1, res2) = task(input.as_str());
    let end = Instant::now();

    let delta = end.duration_since(start);
    println!(
        "{} | Part 1: {} | Part 2: {} | Time: {:?}",
        name,
        res1,
        res2,
        delta
    );
}
