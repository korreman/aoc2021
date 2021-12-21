use std::collections::HashMap;

pub fn run(input: &str) -> (u64, u64) {
    let mut names: HashMap<&str, (u32, bool)> = HashMap::new();
    names.insert("start", (0, false));
    names.insert("end", (1, false));

    let edges: Vec<(u32, u32)> = input
        .lines()
        .map(|line| {
            let (from, to) = line.split_once('-').unwrap();
            let from_idx: u32 = names.get(from).map(|(idx, _)| *idx).unwrap_or_else(|| {
                let idx = names.len() as u32;
                names.insert(from, (idx, from.chars().next().unwrap().is_uppercase()));
                idx
            });
            let to_idx: u32 = names.get(to).map(|(idx, _)| *idx).unwrap_or_else(|| {
                let idx = names.len() as u32;
                names.insert(to, (idx, to.chars().next().unwrap().is_uppercase()));
                idx
            });
            (from_idx, to_idx)
        })
        .collect();

    let mut bigness = vec![false; names.len()];
    for (idx, big) in names.values() {
        bigness[*idx as usize] = *big;
    }

    let result1 = traverse(&edges, bigness.as_slice(), false);
    let result2 = traverse(&edges, bigness.as_slice(), true);

    (result1, result2)
}

#[derive(Debug)]
struct PathEntry {
    idx: u32,
    neighbors: Vec<u32>,
}

fn get_neighbors(node: u32, edges: &[(u32, u32)]) -> Vec<u32> {
    edges
        .iter()
        .filter_map(|(from, to)| {
            if *from == node {
                Some(*to)
            } else if *to == node {
                Some(*from)
            } else {
                None
            }
        })
        .collect()
}

fn traverse(edges: &[(u32, u32)], bigness: &[bool], mut extra_visits: bool) -> u64 {
    let mut counter = 0;
    let mut path: Vec<PathEntry> = vec![PathEntry {
        idx: 0,
        neighbors: get_neighbors(0, &edges),
    }];

    loop {
        if let Some(cur_node) = path.last_mut() {
            let next_node = cur_node.neighbors.pop();
            match next_node {
                None => {
                    let idx = path.pop().unwrap().idx;
                    if !bigness[idx as usize] && path.iter().any(|node| node.idx == idx) {
                        extra_visits = true;
                    }
                }
                Some(0) => {}
                Some(1) => counter += 1,
                Some(n) if bigness[n as usize] || !path.iter().any(|node| node.idx == n) => path
                    .push(PathEntry {
                        idx: n,
                        neighbors: get_neighbors(n, &edges),
                    }),
                Some(n) if extra_visits => {
                    extra_visits = false;
                    path.push(PathEntry {
                        idx: n,
                        neighbors: get_neighbors(n, &edges),
                    });
                }
                _ => {}
            }
        } else {
            break;
        }
    }
    counter
}
