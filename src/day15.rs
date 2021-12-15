use std::collections::BTreeSet;

pub fn run(input: &str) -> (u64, u64) {
    let data = input
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let map = Grid {
        data,
        width,
        height,
    };

    let result1 = task(&map);
    let large_map = map.project(5);
    let result2 = task(&large_map);
    (result1, result2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: usize,
    y: usize,
}

struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn get(&self, p: Pos) -> Option<&T> {
        if p.x < self.width && p.y < self.height {
            Some(&self.data[p.x + p.y * self.width])
        } else {
            None
        }
    }

    fn get_mut(&mut self, p: Pos) -> Option<&mut T> {
        if p.x < self.width && p.y < self.height {
            Some(&mut self.data[p.x + p.y * self.width])
        } else {
            None
        }
    }

    fn neighbors(&self, p: Pos) -> Vec<Pos> {
        let left = if p.x > 0 {
            Some(Pos { x: p.x - 1, y: p.y })
        } else {
            None
        };
        let right = if p.x < self.width - 1 {
            Some(Pos { x: p.x + 1, y: p.y })
        } else {
            None
        };
        let up = if p.y > 0 {
            Some(Pos { x: p.x, y: p.y - 1 })
        } else {
            None
        };
        let down = if p.y < self.height - 1 {
            Some(Pos { x: p.x, y: p.y + 1 })
        } else {
            None
        };
        [left, right, up, down].iter().filter_map(|&x| x).collect()
    }
}

impl<T: std::fmt::Display> Grid<T> {
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.data[x + y * self.width]);
            }
            println!("");
        }
    }
}

impl Grid<u8> {
    fn project(&self, n: usize) -> Self {
        let new_width = self.width * n;
        let new_height = self.height * n;
        let mut new_data = vec![0; new_width * new_height];
        for new_x in 0..new_width {
            for new_y in 0..new_height {
                let x = new_x % self.width;
                let y = new_y % self.height;
                new_data[new_x + new_y * new_width] = ((self.data[x + y * self.width] as usize
                    + new_x / self.width
                    + new_y / self.height - 1)
                    % 9 + 1) as u8;
            }
        }
        Self {
            data: new_data,
            width: new_width,
            height: new_height,
        }
    }
}

struct State {
    costs: Grid<u32>,
    queue: BTreeSet<(u32, Pos)>,
}

impl State {
    fn new(width: usize, height: usize) -> Self {
        let mut data = vec![u32::MAX; width * height];
        data[0] = 0;

        let mut queue = BTreeSet::new();
        queue.insert((0, Pos { x: 0, y: 0 }));

        Self {
            costs: Grid {
                data,
                width,
                height,
            },
            queue,
        }
    }

    fn update(&mut self, p: Pos, value: u32) {
        if let Some(cell) = self.costs.get_mut(p) {
            if value < *cell {
                *cell = value;
                self.queue.remove(&(*cell, p));
                self.queue.insert((value, p));
            }
        }
    }

    fn pop(&mut self) -> Option<(u32, Pos)> {
        let min_entry = self.queue.iter().next()?.clone();
        self.queue.remove(&min_entry);
        Some(min_entry)
    }
}

fn task(map: &Grid<u8>) -> u64 {
    let mut state = State::new(map.width, map.height);
    let mut result1 = 0;
    while let Some((p_cost, p)) = state.pop() {
        if p == (Pos {
            x: map.width - 1,
            y: map.height - 1,
        }) {
            result1 = p_cost as u64;
            break;
        }
        for q in map.neighbors(p) {
            if let Some(q_cost) = map.get(q) {
                //println!("Relaxing {:?} to {}", q, p_cost + *q_cost as u32);
                state.update(q, p_cost + *q_cost as u32);
            }
        }
    }
    result1
}
