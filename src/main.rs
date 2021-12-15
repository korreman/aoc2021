use std::time::Instant;

mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

fn main() {
    run(day3::run, "data/day3.txt", "Day 3");
    run(day4::run, "data/day4.txt", "Day 4");
    run(day5::run, "data/day5.txt", "Day 5");
    run(day6::run, "data/day6.txt", "Day 6");
    run(day7::run, "data/day7.txt", "Day 7");
    run(day8::run, "data/day8.txt", "Day 8");
    run(day9::run, "data/day9.txt", "Day 9");
    run(day10::run, "data/day10.txt", "Day 10");
    run(day11::run, "data/day11.txt", "Day 11");
    run(day12::run, "data/day12.txt", "Day 12");
    run(day13::run, "data/day13.txt", "Day 13");
    run(day14::run, "data/day14.txt", "Day 14");
    run(day15::run, "data/day15.txt", "Day 15");
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
