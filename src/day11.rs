pub fn run(input: &str) -> (u64, u64) {
    let mut grid: Vec<Vec<Octopus>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Octopus {
                    energy: c.to_digit(10).unwrap() as u8,
                    has_flashed: false,
                })
                .collect()
        })
        .collect();

    let mut result1 = 0;
    let mut result2 = 0;
    let mut num_flashes: u64 = 0;
    let mut step = 0;
    loop {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                grid[i][j].energy += 1;
            }
        }
        loop {
            let mut stable = true;
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    let octopus = &mut grid[i][j];
                    if octopus.energy > 9 && !octopus.has_flashed {
                        stable = false;
                        octopus.has_flashed = true;
                        grid.get_mut(i + 1)
                            .and_then(|r| r.get_mut(j - 1))
                            .map(|o| o.energy += 1);
                        grid.get_mut(i + 1)
                            .and_then(|r| r.get_mut(j))
                            .map(|o| o.energy += 1);
                        grid.get_mut(i + 1)
                            .and_then(|r| r.get_mut(j + 1))
                            .map(|o| o.energy += 1);
                        grid.get_mut(i)
                            .and_then(|r| r.get_mut(j + 1))
                            .map(|o| o.energy += 1);
                        grid.get_mut(i - 1)
                            .and_then(|r| r.get_mut(j + 1))
                            .map(|o| o.energy += 1);
                        grid.get_mut(i - 1)
                            .and_then(|r| r.get_mut(j))
                            .map(|o| o.energy += 1);
                        grid.get_mut(i - 1)
                            .and_then(|r| r.get_mut(j - 1))
                            .map(|o| o.energy += 1);
                        grid.get_mut(i)
                            .and_then(|r| r.get_mut(j - 1))
                            .map(|o| o.energy += 1);
                    }
                }
            }
            if stable {
                break;
            }
        }
        for row in &mut grid {
            for octopus in row {
                if octopus.has_flashed {
                    num_flashes += 1;
                    octopus.energy = 0;
                    octopus.has_flashed = false;
                }
            }
        }

        step += 1;
        let synced = grid.iter().flat_map(|row| row.iter()).all(|octo| octo.energy == 0);
        if synced {
            result2 = step;
            break;
        }

        if step == 100 {
            result1 = num_flashes;
        }
    }

    (result1, result2)
}

struct Octopus {
    energy: u8,
    has_flashed: bool,
}

fn print_grid(grid: &Vec<Vec<Octopus>>) {
    for row in grid {
        for o in row {
            print!("{}", o.energy);
        }
        println!("");
    }
}
