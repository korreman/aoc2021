pub fn run(input: &str) -> (u32, u32) {
    let cells: Vec<Cell> = input
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '.' => Cell::Free,
            'v' => Cell::South,
            '>' => Cell::East,
            _ => panic!("parse error: bad cell"),
        })
        .collect();

    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let mut grid = Grid {
        width,
        height,
        data: cells,
    };

    let mut steps: u32 = 1;
    while !grid.step() {
        steps += 1;
    }

    (steps, 0)
}

#[derive(Clone, Eq, PartialEq)]
enum Cell {
    Free,
    South,
    East,
}

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<Cell>,
}

impl Grid {
    fn step(&mut self) -> bool {
        let mut grid1 = self.data.clone();
        for i in 0..self.data.len() {
            if self.data[i] == Cell::East {
                let neighbor = (i + 1) % self.width + (i / self.width) * self.width;
                if self.data[neighbor] == Cell::Free {
                    grid1[i] = Cell::Free;
                    grid1[neighbor] = Cell::East;
                }
            }
        }
        let mut grid2 = grid1.clone();
        for i in 0..grid1.len() {
            if grid1[i] == Cell::South {
                let neighbor = (i + self.width) % (self.width * self.height);
                if grid1[neighbor] == Cell::Free {
                    grid2[i] = Cell::Free;
                    grid2[neighbor] = Cell::South;
                }
            }
        }
        let result = self.data == grid2;
        self.data = grid2;
        result
    }

    fn print(&self) {
        for j in 0..self.height {
            for i in 0..self.width {
                let c = match self.data[i + j * self.width] {
                    Cell::Free => '.',
                    Cell::South => 'v',
                    Cell::East => '>',
                };
                print!("{}", c);
            }
            println!();
        }
    }
}
