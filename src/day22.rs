pub fn run(input: &str) -> (i64, i64) {
    // Input space is too large to store in an array.
    // Seems like you could arrange cuboids into a tree structure.
    // Like a binary tree or an octuple tree.

    // Parse
    let actions: Vec<(Cuboid, bool)> = input
        .lines()
        .map(|line| {
            let value = line.starts_with("on");
            let nums: Vec<i64> = line[3..]
                .split(&['x', 'y', 'z', '=', '.', ',', ' '][..])
                .filter(|&n| n != "")
                .map(|n| n.parse().unwrap())
                .collect();
            (
                Cuboid {
                    x1: nums[0],
                    x2: nums[1] + 1,
                    y1: nums[2],
                    y2: nums[3] + 1,
                    z1: nums[4],
                    z2: nums[5] + 1,
                },
                value,
            )
        })
        .collect();

    let mut tree1 = Tree::Leaf {
        cuboid: Cuboid {
            x1: -50,
            x2: 51,
            y1: -50,
            y2: 51,
            z1: -50,
            z2: 51,
        },
        state: false,
    };

    for (target, value) in &actions {
        tree1.set(*target, *value);
    }
    let result1 = tree1.count();

    let mut cuboid2 = Cuboid {
        x1: 0,
        x2: 0,
        y1: 0,
        y2: 0,
        z1: 0,
        z2: 0,
    };
    for (action, _) in &actions {
        cuboid2.x1 = cuboid2.x1.min(action.x1);
        cuboid2.x2 = cuboid2.x2.max(action.x2);
        cuboid2.y1 = cuboid2.y1.min(action.y1);
        cuboid2.y2 = cuboid2.y2.max(action.y2);
        cuboid2.z1 = cuboid2.z1.min(action.z1);
        cuboid2.z2 = cuboid2.z2.max(action.z2);
    }

    let mut tree2 = Tree::Leaf {
        cuboid: cuboid2,
        state: false,
    };

    for (target, value) in &actions {
        tree2.set(*target, *value);
    }
    let result2 = tree2.count();

    (result1, result2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cuboid {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    z1: i64,
    z2: i64,
}

#[derive(Debug)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug)]
enum Tree {
    Leaf {
        // it'd be nice if the cuboid wasn't necessary
        cuboid: Cuboid,
        state: bool,
    },
    // TODO: include spatial information in splitting node.
    // The whole point is to have this information present at a higher level than the leaves.
    Split {
        axis: Axis,
        plane: i64, // specifies the start of the second cuboid
        a: Box<Tree>,
        b: Box<Tree>,
    },
}

impl Tree {
    fn set(&mut self, target: Cuboid, value: bool) {
        match self {
            Tree::Split { axis, plane, a, b } => {
                let (t1, t2) = match axis {
                    Axis::X => (target.x1, target.x2),
                    Axis::Y => (target.y1, target.y2),
                    Axis::Z => (target.z1, target.z2),
                };
                if t1 < *plane {
                    a.set(target, value);
                }
                if t2 >= *plane {
                    b.set(target, value);
                }
            }
            Tree::Leaf {
                cuboid: leaf,
                state,
            } if value != *state => {
                let cut = if target.x1 > leaf.x1 && target.x1 < leaf.x2 {
                    Some((Axis::X, target.x1))
                } else if target.x2 < leaf.x2 && target.x2 > leaf.x1 {
                    Some((Axis::X, target.x2))
                } else if target.y1 > leaf.y1 && target.y1 < leaf.y2 {
                    Some((Axis::Y, target.y1))
                } else if target.y2 < leaf.y2 && target.y2 > leaf.y1 {
                    Some((Axis::Y, target.y2))
                } else if target.z1 > leaf.z1 && target.z1 < leaf.z2 {
                    Some((Axis::Z, target.z1))
                } else if target.z2 < leaf.z2 && target.z2 > leaf.z1 {
                    Some((Axis::Z, target.z2))
                } else {
                    None
                };
                match cut {
                    Some((axis, plane)) => {
                        let (mut cuboid1, mut cuboid2) = (leaf.clone(), leaf.clone());
                        match axis {
                            Axis::X => {
                                cuboid1.x2 = plane;
                                cuboid2.x1 = plane;
                            }
                            Axis::Y => {
                                cuboid1.y2 = plane;
                                cuboid2.y1 = plane;
                            }
                            Axis::Z => {
                                cuboid1.z2 = plane;
                                cuboid2.z1 = plane;
                            }
                        }
                        *self = Tree::Split {
                            axis,
                            plane,
                            a: Box::new(Tree::Leaf {
                                cuboid: cuboid1,
                                state: *state,
                            }),
                            b: Box::new(Tree::Leaf {
                                cuboid: cuboid2,
                                state: *state,
                            }),
                        };
                        self.set(target, value);
                    }
                    None => {
                        if target.x1 <= leaf.x1
                            && target.x2 >= leaf.x2
                            && target.y1 <= leaf.y1
                            && target.y2 >= leaf.y2
                            && target.z1 <= leaf.z1
                            && target.z2 >= leaf.z2
                        {
                            *state = value;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn count(&self) -> i64 {
        match self {
            Tree::Leaf {
                cuboid,
                state: true,
            } => {
                let res =
                    (cuboid.x2 - cuboid.x1) * (cuboid.y2 - cuboid.y1) * (cuboid.z2 - cuboid.z1);
                res
            }
            Tree::Split { a, b, .. } => a.count() + b.count(),
            _ => 0,
        }
    }

    fn print(&self) {
        match self {
            Tree::Leaf {
                cuboid,
                state: true,
            } => println!("{:?}", cuboid),
            Tree::Split { a, b, .. } => {
                a.print();
                b.print()
            }
            _ => {}
        }
    }
}
