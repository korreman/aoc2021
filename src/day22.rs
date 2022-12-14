pub fn run(input: &str) -> (i64, i64) {
    // Input space is too large to store in an array.
    // Seems like you could arrange cuboids into a tree structure.
    // Like a binary tree or an octuple tree.

    // TODO:
    // 1. Replace recursive ADT and recursion with array representation and loop+stack.
    // 2. Attempt to simplify tree structure.
    //    - Potentially possible to remove cuboid from leaves.
    //      The main point of this would be to remove the need for additional bounds checks.
    //    - Alternatively, use only cuboids?
    //    - Point is, I think that a bunch of superfluous bounds checks are being performed.

    // Parse
    let actions: Vec<(Cuboid, bool)> = input
        .lines()
        .map(|line| {
            let value = line.starts_with("on");
            let nums: Vec<i32> = line[3..]
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

    let mut cuboid = Cuboid {
        x1: 0,
        x2: 0,
        y1: 0,
        y2: 0,
        z1: 0,
        z2: 0,
    };
    for (action, _) in &actions {
        cuboid.x1 = cuboid.x1.min(action.x1);
        cuboid.x2 = cuboid.x2.max(action.x2);
        cuboid.y1 = cuboid.y1.min(action.y1);
        cuboid.y2 = cuboid.y2.max(action.y2);
        cuboid.z1 = cuboid.z1.min(action.z1);
        cuboid.z2 = cuboid.z2.max(action.z2);
    }

    let mut tree2 = Tree::Leaf {
        cuboid,
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
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
}

impl Cuboid {
    fn get_cut(&self, other: Cuboid) -> Option<(Axis, i32)> {
        if other.x1 > self.x1 && other.x1 < self.x2 {
            Some((Axis::X, other.x1))
        } else if other.x2 < self.x2 && other.x2 > self.x1 {
            Some((Axis::X, other.x2))
        } else if other.y1 > self.y1 && other.y1 < self.y2 {
            Some((Axis::Y, other.y1))
        } else if other.y2 < self.y2 && other.y2 > self.y1 {
            Some((Axis::Y, other.y2))
        } else if other.z1 > self.z1 && other.z1 < self.z2 {
            Some((Axis::Z, other.z1))
        } else if other.z2 < self.z2 && other.z2 > self.z1 {
            Some((Axis::Z, other.z2))
        } else {
            None
        }
    }
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
    Split {
        axis: Axis,
        plane: i32,
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
            } if value != *state => match leaf.get_cut(target) {
                Some((axis, plane)) => {
                    let (mut cuboid_a, mut cuboid_b) = (leaf.clone(), leaf.clone());
                    match axis {
                        Axis::X => {
                            cuboid_a.x2 = plane;
                            cuboid_b.x1 = plane
                        }
                        Axis::Y => {
                            cuboid_a.y2 = plane;
                            cuboid_b.y1 = plane
                        }
                        Axis::Z => {
                            cuboid_a.z2 = plane;
                            cuboid_b.z1 = plane
                        }
                    }
                    *self = Tree::Split {
                        axis,
                        plane,
                        a: Box::new(Tree::Leaf {
                            cuboid: cuboid_a,
                            state: *state,
                        }),
                        b: Box::new(Tree::Leaf {
                            cuboid: cuboid_b,
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
            },
            _ => {}
        }
    }

    fn count(&self) -> i64 {
        match self {
            Tree::Leaf {
                cuboid,
                state: true,
            } => {
                (cuboid.x2 as i64 - cuboid.x1 as i64)
                    * (cuboid.y2 as i64 - cuboid.y1 as i64)
                    * (cuboid.z2 as i64 - cuboid.z1 as i64)
            }
            Tree::Split { a, b, .. } => a.count() + b.count(),
            _ => 0,
        }
    }
}
