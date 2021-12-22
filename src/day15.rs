use std::collections::VecDeque;
use std::time::Instant;

pub fn run(input: &str) -> (u64, u64) {
    let t1 = Instant::now();

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

    let t2 = Instant::now();
    let result1 = task(&map);
    let t3 = Instant::now();
    let large_map = map.project(5);
    let t4 = Instant::now();
    let result2 = task(&large_map);
    let t5 = Instant::now();
    println!("{:?},{:?},{:?},{:?}", t2 - t1, t3 - t2, t4 - t3, t5 - t4,);
    (result1, result2)
}

fn task(map: &Grid<u8>) -> u64 {
    let mut state = State::new(map.width, map.height);
    loop {
        let (cost, pos) = state.pop();
        if pos
            == (Pos {
                x: map.width - 1,
                y: map.height - 1,
            })
        {
            break (cost as u64);
        }
        for q in map.neighbors(pos) {
            if let Some(q_cost) = map.get(q) {
                state.relax(q, cost + *q_cost as usize);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

    fn neighbors(&self, p: Pos) -> [Pos; 4] {
        [
            Pos { x: p.x - 1, y: p.y },
            Pos { x: p.x + 1, y: p.y },
            Pos { x: p.x, y: p.y - 1 },
            Pos { x: p.x, y: p.y + 1 },
        ]
    }
}

impl<T: std::fmt::Display> Grid<T> {
    fn _print(&self) {
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

struct State {
    costs: Grid<usize>,
    queue: SlidingBucketQueue<Pos>,
}

impl State {
    fn new(width: usize, height: usize) -> Self {
        let mut data = vec![usize::MAX; width * height];
        data[0] = 0;

        let mut queue = SlidingBucketQueue::new(11);
        queue.push(0, Pos { x: 0, y: 0 });
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
                self.queue.push(value, p);
            }
        }
    }

    fn pop(&mut self) -> (usize, Pos) {
        loop {
            let (cost, pos) = self.queue.pop();
            if cost == *self.costs.get(pos).unwrap() {
                break (cost, pos);
            }
        }
    }
}

struct SlidingBucketQueue<T> {
    offset: usize,
    queue: VecDeque<Vec<T>>,
}

impl<T: Clone> SlidingBucketQueue<T> {
    fn new(range: usize) -> Self {
        Self {
            offset: 0,
            queue: VecDeque::from(vec![Vec::new(); range]),
        }
    }

    fn push(&mut self, key: usize, value: T) {
        self.queue[key - self.offset].push(value);
    }

    fn pop(&mut self) -> (usize, T) {
        let value = loop {
            if let Some(v) = self.queue[0].pop() {
                break v;
            } else {
                self.offset += 1;
                self.queue.rotate_left(1);
            }
        };
        (self.offset, value)
    }
}
