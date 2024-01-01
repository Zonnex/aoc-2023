use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Binary,
    vec,
};

use itertools::Itertools;

use crate::{utils::vector_3d::Vector3, Solution, SolutionPair};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Brick {
    id: usize,
    from: Vector3,
    to: Vector3,
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left = (self.from.z, self.from.x, self.from.y);
        let right = (other.from.z, other.from.x, other.from.y);

        left.cmp(&right)
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Brick {
    fn move_to(&mut self, height: usize) {
        let height = height as isize;
        let diff = height - self.from.z;

        self.from.z += diff;
        self.to.z += diff;
    }
}

#[derive(Debug, Clone)]
struct Bricks {
    bricks: Vec<Brick>,
    width: usize,
    height: usize,
    depth: usize,
}

impl Bricks {
    fn parse(input: &str) -> Self {
        let (mut width, mut depth, mut height) = (0, 0, 0);
        let mut bricks = input
            .lines()
            .map(|line| {
                let (from, to) = line.split_once('~').unwrap();
                let (x, y, z) = from
                    .split(',')
                    .filter_map(|d| d.parse().ok())
                    .collect_tuple()
                    .unwrap();

                let from = Vector3::new(x, y, z);

                let (x, y, z) = to
                    .split(',')
                    .filter_map(|d| d.parse::<usize>().ok())
                    .collect_tuple()
                    .unwrap();

                let to = Vector3::new_usize(x, y, z);

                let (from, to) = if from.z <= to.z {
                    (from, to)
                } else {
                    (to, from)
                };
                Brick { id: 0, from, to }
            })
            .collect::<Vec<_>>();

        bricks.sort_by(|a, b| a.cmp(b));

        let bricks = bricks
            .into_iter()
            .enumerate()
            .map(|(id, mut b)| {
                b.id = id;
                b
            })
            .rev()
            .inspect(|b| {
                width = width.max(b.from.x).max(b.to.x);
                depth = depth.max(b.from.y).max(b.to.y);
                height = height.max(b.from.z).max(b.to.z);
            })
            .collect();

        Self {
            bricks,
            width: (width + 1) as usize,
            depth: (depth + 1) as usize,
            height: (height + 1) as usize,
        }
    }
}

struct HeightMap {
    map: Vec<usize>,
    width: usize,
    depth: usize,
}

impl HeightMap {
    fn new(width: usize, depth: usize) -> Self {
        Self {
            map: vec![0; width * depth],
            width,
            depth,
        }
    }

    fn get(&self, x: usize, y: usize) -> usize {
        self.map[y * self.width + x]
    }

    fn get_height_for_brick(&self, brick: Brick) -> usize {
        let mut max_height = 0;
        for x in brick.from.x..=brick.to.x {
            for y in brick.from.y..=brick.to.y {
                let height = self.get(x as usize, y as usize);
                max_height = max_height.max(height);
            }
        }
        // this is the lowest point we can place the brick
        max_height + 1
    }

    fn place_brick(&mut self, brick: Brick) {
        for x in brick.from.x..=brick.to.x {
            for y in brick.from.y..=brick.to.y {
                for z in brick.from.z..=brick.to.z {
                    let x = x as usize;
                    let y = y as usize;

                    self.map[y * self.width + x] = z as usize;
                }
            }
        }
    }
}

struct Tower {
    bricks: Vec<Brick>,
    tower: Vec<Vec<Vec<Option<usize>>>>,
    height_map: HeightMap,
    dependency_map: HashMap<usize, HashSet<usize>>,
    foundation_map: HashMap<usize, HashSet<usize>>,
}

impl Tower {
    fn new(width: usize, depth: usize, height: usize) -> Self {
        Self {
            bricks: Vec::new(),
            tower: vec![vec![vec![None; height]; depth]; width],
            height_map: HeightMap::new(width, depth),
            dependency_map: HashMap::new(),
            foundation_map: HashMap::new(),
        }
    }

    fn place_brick(&mut self, mut brick: Brick) {
        self.bricks.push(brick);
        let height = self.height_map.get_height_for_brick(brick);
        brick.move_to(height);
        self.height_map.place_brick(brick);

        for x in brick.from.x..=brick.to.x {
            for y in brick.from.y..=brick.to.y {
                for z in brick.from.z..=brick.to.z {
                    let x = x as usize;
                    let y = y as usize;
                    let z = z as usize;

                    debug_assert_eq!(self.tower[x][y][z], None);

                    self.tower[x][y][z] = Some(brick.id);

                    if let Some(id) = self.tower[x][y][z - 1] {
                        if id != brick.id {
                            self.dependency_map
                                .entry(brick.id)
                                .or_insert_with(HashSet::new)
                                .insert(id);

                            self.foundation_map
                                .entry(id)
                                .or_insert_with(HashSet::new)
                                .insert(brick.id);
                        }
                    }
                }
            }
        }
    }

    fn print(&self) {
        let height = self.tower[0][0].len();
        let depth = self.tower[0].len();
        let width = self.tower.len();

        for z in (0..height).rev() {
            for y in 0..depth {
                for x in 0..width {
                    print!(
                        "{}",
                        match self.tower[x][y][z] {
                            Some(id) => format!("{:02}", id),
                            None => "..".to_string(),
                        }
                    );
                }
                println!();
            }
            println!("-----------------");
        }

        for (brick, rests_on) in &self.dependency_map {
            println!("{} -> {:?}", brick, rests_on);
        }
    }

    fn disintegratable_bricks(&self) -> Vec<&Brick> {
        let invalid = self
            .dependency_map
            .values()
            .filter(|s| s.len() == 1)
            .flatten()
            .collect::<HashSet<_>>();

        self.bricks
            .iter()
            .filter(|b| !invalid.contains(&b.id))
            .collect::<Vec<_>>()
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let mut bricks = Bricks::parse(input);
    let mut tower = Tower::new(bricks.width, bricks.depth, bricks.height);

    while let Some(brick) = bricks.bricks.pop() {
        tower.place_brick(brick);
    }
    let p1 = tower.disintegratable_bricks();
    let rest = tower
        .bricks
        .iter()
        .filter(|b| !p1.contains(b))
        .collect::<Vec<_>>();

    (Solution::Usize(p1.len()), p2(&tower, &rest))
}

fn p2(tower: &Tower, bricks: &[&Brick]) -> Solution {
    let mut results = HashMap::new();

    for brick in bricks.iter().rev() {
        let count = count_chain(tower, &brick);
        results.insert(brick.id, count);
    }

    Solution::Usize(results.values().sum())
}

fn count_chain(tower: &Tower, brick: &Brick) -> usize {
    let mut count = 0;
    let mut fallen = HashSet::new();
    let mut queue = VecDeque::new();
    fallen.insert(brick.id);

    for brick in tower.foundation_map.get(&brick.id).unwrap() {
        queue.push_back(&tower.bricks[*brick]);
    }

    while let Some(brick) = queue.pop_front() {
        if fallen.contains(&brick.id) {
            continue;
        }
        if let Some(set) = tower.dependency_map.get(&brick.id) {
            if set.iter().all(|id| fallen.contains(id)) {
                fallen.insert(brick.id);
                count += 1;
                
                if let Some(set) = tower.foundation_map.get(&brick.id) {
                    for id in set {
                        queue.push_back(&tower.bricks[*id]);
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day22/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(5));
        assert_eq!(p2, Solution::Usize(7));
    }
}
