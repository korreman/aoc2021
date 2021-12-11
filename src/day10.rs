pub fn run(input: &str) -> (u64, u64) {
    let lines: Vec<&str> = input.lines().collect();
    let mut result1 = 0;
    let mut scores = Vec::new();
    for line in lines {
        let mut stack = Vec::new();
        let mut is_corrupt = false;
        for c in line.chars() {
            if "([{<".contains(c) {
                stack.push(c);
            } else {
                let d = stack.last();
                match (d, c) {
                    (Some('('), ')') => stack.pop(),
                    (Some('['), ']') => stack.pop(),
                    (Some('{'), '}') => stack.pop(),
                    (Some('<'), '>') => stack.pop(),
                    _ => {
                        let points = match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!("bad symbol"),
                        };
                        result1 += points;
                        is_corrupt = true;
                        break;
                    }
                };
            }
        }
        if is_corrupt { continue; }

        stack.reverse();
        let mut score = 0;
        for c in stack {
            let add_points = match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("bad symbol"),
            };
            score = score * 5 + add_points;
        }
        scores.push(score);
    }
    scores.sort();
    let result2 = scores[scores.len() / 2];

    (result1, result2)
}
