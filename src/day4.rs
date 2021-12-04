pub fn run() {
    let input = std::fs::read_to_string("data/day4.txt").unwrap();

    // ----- Input parsing -----
    let (d, b) = input.split_once("\n\n").unwrap();

    let draws: Vec<u32> = d.split(',').map(|d| d.parse().unwrap()).collect();

    let boards1: Vec<Board> = b
        .split("\n\n")
        .map(|b| {
            let ns = b.split_ascii_whitespace().map(|n| n.parse().unwrap());
            let mut arr: [u32; 25] = [0; 25];
            for (n, a) in ns.zip(&mut arr) {
                *a = n;
            }
            // bit of an anti-pattern to construct the full board with marks inside the parse step
            Board::new(arr)
        })
        .collect();

    // ----- Computations -----
    let boards2 = boards1.clone();
    let result1 = find_winner(boards1, &draws);
    let result2 = find_loser(boards2, &draws);

    // ----- Output -----
    println!("Day 3 - Part 1: {} - Part 2: {}", result1, result2);
}

#[derive(Clone, Debug)]
struct Board {
    // alternatively, both numbers and marks could've been contained in one array
    numbers: [u32; 25],
    marks: [bool; 25],
}

impl Board {
    fn new(numbers: [u32; 25]) -> Self {
        Self {
            numbers,
            marks: [false; 25],
        }
    }

    fn check_bingo(&self) -> bool {
        let m = self.marks;
        let mut bingo = false;
        for i in 0..5 {
            bingo = bingo
                || !m[i * 5..(i * 5) + 5].contains(&false)
                || (m[i] && m[i + 5] && m[i + 10] && m[i + 15] && m[i + 20]);
        }
        bingo
    }

    // NOTE: assumes that bingo cards do not contain duplicates
    fn try_mark_number(&mut self, n: u32) {
        if let Some(i) = self.numbers.iter().position(|&m| m == n) {
            self.marks[i] = true;
        }
    }

    fn unmarked_sum(&self) -> u32 {
        self.numbers
            .iter()
            .zip(&self.marks)
            .map(|(&n, &m)| if m { 0 } else { n })
            .sum()
    }
}

fn find_winner(mut boards: Vec<Board>, draws: &[u32]) -> u32 {
    for &draw in draws {
        boards
            .iter_mut()
            .for_each(|board| board.try_mark_number(draw));
        if let Some(board) = boards.iter().find(|board| board.check_bingo()) {
            return board.unmarked_sum() * draw;
        }
    }
    panic!("No bingo! D:");
}

fn find_loser(mut boards: Vec<Board>, draws: &[u32]) -> u32 {
    for &draw in draws {
        boards
            .iter_mut()
            .for_each(|board| board.try_mark_number(draw));
        if boards.len() == 1 && boards[0].check_bingo() {
            return boards[0].unmarked_sum() * draw;
        }
        boards.retain(|board| !board.check_bingo());
    }
    panic!("No bingo! D:");
}
