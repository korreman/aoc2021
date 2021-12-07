pub fn run(input: &str) -> (u32, u32) {
    // ----- Parsing -----
    let mut lines1: Vec<Line> = input
        .split(&[' ', ',', '-', '>', '\n'][..])
        .filter(|&s| s != "")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>()
        .chunks(4)
        .map(|x| match x {
            &[x1, y1, x2, y2] => Line {x1, y1, x2, y2, finished: false},
            _ => panic!("Invalid input"), // const generics are right around the corner :D
        })
        .collect();

    // ----- Computations -----
    let lines2 = lines1.clone();
    lines1.retain(|l| l.is_straight());
    let result1 = draw_and_count(lines1);
    let result2 = draw_and_count(lines2);

    // ----- Output -----
    (result1, result2)
}

#[derive(Debug, Clone)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    finished: bool,
}

impl Line {
    fn max(&self) -> i32 {
        i32::max(i32::max(self.x1, self.x2), i32::max(self.y1, self.y2))
    }

    // checks whether the line is either vertical or horizontal
    fn is_straight(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }
}

// will iterate through every grid point that the line touches
impl Iterator for Line {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        // NOTE: [finished] is tracked as state in order to take an extra turn for the endpoint
        if self.finished { return None; }
        if self.x1 == self.x2 && self.y1 == self.y2 {
            self.finished = true;
        }
        let result = (self.x1, self.y1);
        // move (x1, y1) closer to (x2, y2)
        self.x1 += (self.x2 - self.x1).signum();
        self.y1 += (self.y2 - self.y1).signum();
        Some(result)
    }
}

fn draw_and_count(lines: Vec<Line>) -> u32 {
    // scale the grid to fit all lines
    let width = (lines.iter().map(|l| l.max()).max().unwrap() + 1) as usize;
    let mut grid: Vec<u32> = vec![0; width * width];

    for l in lines {
        for (x, y) in l {
            grid[x as usize + y as usize * width] += 1
        }
    }
    grid.iter().filter(|&&n| n >= 2).count() as u32
}
