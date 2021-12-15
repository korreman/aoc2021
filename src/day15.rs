use std::cmp::Ordering;
use std::collections::BinaryHeap;

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
                    + new_y / self.height
                    - 1)
                    % 9
                    + 1) as u8;
            }
        }
        Self {
            data: new_data,
            width: new_width,
            height: new_height,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Entry {
    cost: usize,
    pos: Pos,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct State {
    costs: Grid<usize>,
    queue: BinaryHeap<Entry>,
}

impl State {
    fn new(width: usize, height: usize) -> Self {
        let mut data = vec![usize::MAX; width * height];
        data[0] = 0;

        let mut queue = BinaryHeap::new();
        queue.push(Entry {
            pos: Pos { x: 0, y: 0 },
            cost: 0,
        });

        Self {
            costs: Grid {
                data,
                width,
                height,
            },
            queue,
        }
    }

    fn relax(&mut self, p: Pos, value: usize) {
        if let Some(cell) = self.costs.get_mut(p) {
            if value < *cell {
                *cell = value;
                self.queue.push(Entry {
                    cost: value,
                    pos: p,
                });
            }
        }
    }

    fn pop(&mut self) -> Option<Entry> {
        let mut result = None;
        while let Some(entry) = self.queue.pop() {
            if entry.cost == *self.costs.get(entry.pos).unwrap() {
                result = Some(entry);
                break;
            }
        }
        result
    }
}

fn task(map: &Grid<u8>) -> u64 {
    let mut state = State::new(map.width, map.height);
    let mut result1 = u64::MAX;
    while let Some(entry) = state.pop() {
        if entry.pos
            == (Pos {
                x: map.width - 1,
                y: map.height - 1,
            })
        {
            result1 = entry.cost as u64;
            break;
        }
        for q in map.neighbors(entry.pos) {
            let q_cost = map.get(q).unwrap();
            state.relax(q, entry.cost + *q_cost as usize);
        }
    }
    result1
}
