pub fn run(input: &str) -> (u64, u64) {
    let (coords, folds) = input.split_once("\n\n").unwrap();
    let coords: Vec<Pos> = coords
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Pos {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();

    let folds: Vec<Fold> = folds
        .lines()
        .map(|line| {
            let data = line.strip_prefix("fold along ").unwrap();
            let coord: usize = data[2..].parse().unwrap();
            match data.chars().next() {
                Some('x') => Fold::X(coord),
                Some('y') => Fold::Y(coord),
                _ => panic!("invalid fold"),
            }
        })
        .collect();

    let width = coords.iter().map(|c| c.x).max().unwrap() + 1;
    let height = coords.iter().map(|c| c.y).max().unwrap() + 1;
    let mut paper = Paper::new(width, height);
    for c in coords {
        paper.mark(c);
    }

    // Part 1
    paper.fold(folds[0]);
    let result1 = paper.count();

    // Part 2
    for f in &folds[1..] {
        paper.fold(*f);
    }

    paper.print();
    (result1, 0)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

struct Paper {
    original_width: usize,
    width: usize,
    height: usize,
    data: Vec<bool>,
}

impl Paper {
    fn new(width: usize, height: usize) -> Self {
        Self {
            original_width: width,
            width,
            height,
            data: vec![false; width * height],
        }
    }

    fn mark(&mut self, p: Pos) {
        if p.x < self.width && p.y < self.height {
            self.data[p.x + p.y * self.original_width] = true;
        }
    }

    fn get(&self, p: Pos) -> bool {
        if p.x < self.width && p.y < self.height {
            self.data[p.x + p.y * self.original_width]
        } else {
            false
        }
    }

    fn fold(&mut self, fold: Fold) {
        // ... duplicate code was the fastest to implement
        match fold {
            Fold::X(new_width) => {
                for x in new_width + 1..self.width {
                    for y in 0..self.height {
                        if self.get(Pos { x, y }) {
                            self.mark(Pos {
                                x: 2 * new_width - x,
                                y,
                            });
                        }
                    }
                }
                self.width = new_width;
            }
            Fold::Y(new_height) => {
                for x in 0..self.width {
                    for y in new_height + 1..self.height {
                        if self.get(Pos { x, y }) {
                            self.mark(Pos {
                                x,
                                y: 2 * new_height - y,
                            });
                        }
                    }
                }
                self.height = new_height;
            }
        }
    }

    fn count(&self) -> u64 {
        let mut count = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get(Pos { x, y }) {
                    count += 1;
                }
            }
        }
        count
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let symbol = if self.get(Pos { x, y }) { "██" } else { "░░" };
                print!("{}", symbol);
            }
            println!("");
        }
    }
}
