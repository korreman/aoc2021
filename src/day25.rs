pub fn run(input: &str) -> (u32, u32) {
    let cells: Vec<Cell> = input
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '.' => Cell::Free,
            'v' => Cell::Downer,
            '>' => Cell::Easter,
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
    while grid.step() {
        steps += 1;
    }

    (steps, 0)
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Cell {
    Free,
    Downer,
    Easter,
}

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<Cell>,
}

impl Grid {
    fn step(&mut self) -> bool {
        let mut new_grid = self.data.clone();
        let mut moved = false;
        for i in 0..self.data.len() {
            if self.data[i] == Cell::Easter {
                let neighbor = (i + 1) % self.width + (i / self.width) * self.width;
                if self.data[neighbor] == Cell::Free {
                    new_grid[i] = Cell::Free;
                    new_grid[neighbor] = Cell::Easter;
                    moved = true;
                }
            }
        }
        self.data = new_grid;
        let mut new_grid = self.data.clone();
        for i in 0..self.data.len() {
            if self.data[i] == Cell::Downer {
                let neighbor = (i + self.width) % (self.width * self.height);
                if self.data[neighbor] == Cell::Free {
                    new_grid[i] = Cell::Free;
                    new_grid[neighbor] = Cell::Downer;
                    moved = true;
                }
            }
        }
        self.data = new_grid;
        moved
    }

    fn print(&self) {
        for j in 0..self.height {
            for i in 0..self.width {
                let c = match self.data[i + j * self.width] {
                    Cell::Free => '.',
                    Cell::Downer => 'v',
                    Cell::Easter => '>',
                };
                print!("{}", c);
            }
            println!();
        }
    }
}
