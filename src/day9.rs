use std::collections::HashSet;

pub fn run(input: &str) -> (u32, u32) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut result1: u32 = 0;

    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for x in 0..height {
        for y in 0..width {
            let h = grid[x][y];
            let a = grid.get(x + 1).and_then(|g| g.get(y)).unwrap_or(&10);
            // what to do about underflow?
            let b = grid.get(x - 1).and_then(|g| g.get(y)).unwrap_or(&10);
            let c = grid.get(x).and_then(|g| g.get(y + 1)).unwrap_or(&10);
            let d = grid.get(x).and_then(|g| g.get(y - 1)).unwrap_or(&10);
            if h < *a && h < *b && h < *c && h < *d {
                low_points.push((x, y));
                result1 += h + 1;
            }
        }
    }
    let mut basins: Vec<u32> = low_points
        .iter()
        .map(|(x, y)| {
            let mut basin = HashSet::new();
            basin.insert((*x, *y));
            let mut old_basin = HashSet::new();
            while old_basin != basin {
                old_basin = basin.clone();
                for (x1, y1) in &old_basin {
                    for (x2, y2) in [
                        Some((*x1 + 1, *y1)),
                        x1.checked_sub(1).map(|x1| (x1, *y1)),
                        Some((*x1, *y1 + 1)),
                        y1.checked_sub(1).map(|y1| (*x1, y1)),
                    ]
                    .iter()
                    .filter_map(|p| *p)
                    {
                        if grid.get(x2).and_then(|g| g.get(y2)).copied().unwrap_or(9) != 9 {
                            basin.insert((x2, y2));
                        }
                    }
                }
            }
            basin.len() as u32
        })
        .collect();

    basins.sort_by(|a, b| b.cmp(a));
    let result2 = basins[0] * basins[1] * basins[2];

    (result1, result2)
}
