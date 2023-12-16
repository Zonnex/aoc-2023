use std::{collections::{HashMap, HashSet}, fmt::Display};

use crate::{utils::vector_2d::*, Solution, SolutionPair};

struct Map {
    grid: HashMap<Vector2, u8>,
    size: Vector2,
}

impl Map {
    fn parse(input: &str) -> Self {
        let (mut x, mut y) = (0, 0);
        let grid = input
            .lines()
            .rev()
            .enumerate()
            .flat_map(|(y, line)| {
                line.bytes()
                    .enumerate()
                    .map(move |(x, c)| (Vector2::new_usize(x, y), c))
            })
            .inspect(|(pos, _)| {
                x = x.max(pos.x);
                y = y.max(pos.y);
            })
            .collect();
        Self {
            grid,
            size: Vector2::new(x, y)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Light {
    position: Vector2,
    direction: Vector2,
}

impl Display for Light {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.position, self.direction)
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let map = Map::parse(input);
    
    (p1(&map), p2(&map))
}

fn p1(map: &Map) -> Solution {
    let light = Light {
        position: Vector2::new(0, map.size.y),
        direction: E,
    };
    Solution::Usize(reflect(map, light))
}

fn reflect(map: &Map, light: Light) -> usize {
    let mut visits = HashSet::new();
    visits.insert(light);
    let mut lights = vec![light];

    while let Some(light) = lights.pop() {
        let new_lights = move_light(map, light);
        for light in new_lights {
            if map.grid.contains_key(&light.position) && visits.insert(light){
                lights.push(light)
            }
        }
    }
    let energized = visits.iter().map(|l| l.position).collect::<HashSet<_>>();
    energized.len()
}

fn p2(map: &Map) -> Solution {
    let mut results = vec![];

    for y in 0..map.size.y {
        let light = Light {
            position: Vector2::new(0, y),
            direction: E,
        };
        results.push(reflect(map, light));
    }

    for y in 0..map.size.y {
        let light = Light {
            position: Vector2::new(map.size.x, y),
            direction: W,
        };
        results.push(reflect(map, light));
    }

    for x in 0..map.size.x {
        let light = Light {
            position: Vector2::new(x, 0),
            direction: N,
        };
        results.push(reflect(map, light));
    }

    for x in 0..map.size.x {
        let light = Light {
            position: Vector2::new(x, map.size.y),
            direction: S,
        };
        results.push(reflect(map, light));
    }
    

    Solution::Usize(results.into_iter().max().unwrap())
}

fn move_light(map: &Map, light: Light) -> Vec<Light> {
    let shape = map.grid.get(&light.position);

    match shape {
        None => return vec![],
        Some(shape) => match light.direction {
            E => move_east(shape, light),
            W => move_west(shape, light),
            N => move_north(shape, light),
            S => move_south(shape, light),
            _ => unreachable!("Invalid direction"),
        },
    }
}

fn move_south(shape: &u8, light: Light) -> Vec<Light> {
    match shape {
        b'-' => vec![
            Light {
                position: light.position + E,
                direction: E,
            },
            Light {
                position: light.position + W,
                direction: W,
            },
        ],
        b'\\' => vec![Light {
            position: light.position + E,
            direction: E,
        }],
        b'/' => vec![Light {
            position: light.position + W,
            direction: W,
        }],
        b'|' => vec![Light {
            position: light.position + S,
            direction: S,
        }],
        b'.' => vec![Light {
            position: light.position + S,
            direction: S,
        }],
        _ => vec![],
    }
}

fn move_north(shape: &u8, light: Light) -> Vec<Light> {
    match shape {
        b'-' => vec![
            Light {
                position: light.position + E,
                direction: E,
            },
            Light {
                position: light.position + W,
                direction: W,
            },
        ],
        b'\\' => vec![Light {
            position: light.position + W,
            direction: W,
        }],
        b'/' => vec![Light {
            position: light.position + E,
            direction: E,
        }],
        b'|' => vec![Light {
            position: light.position + N,
            direction: N,
        }],
        b'.' => vec![Light {
            position: light.position + N,
            direction: N,
        }],
        _ => vec![],
    }
}

fn move_west(shape: &u8, light: Light) -> Vec<Light> {
    match shape {
        b'|' => vec![
            Light {
                position: light.position + N,
                direction: N,
            },
            Light {
                position: light.position + S,
                direction: S,
            },
        ],
        b'\\' => vec![Light {
            position: light.position + N,
            direction: N,
        }],
        b'/' => vec![Light {
            position: light.position + S,
            direction: S,
        }],
        b'-' => vec![Light {
            position: light.position + W,
            direction: W,
        }],
        b'.' => vec![Light {
            position: light.position + W,
            direction: W,
        }],
        _ => vec![],
    }
}

fn move_east(shape: &u8, light: Light) -> Vec<Light> {
    match shape {
        b'|' => vec![
            Light {
                position: light.position + N,
                direction: N,
            },
            Light {
                position: light.position + S,
                direction: S,
            },
        ],
        b'\\' => vec![Light {
            position: light.position + S,
            direction: S,
        }],
        b'/' => vec![Light {
            position: light.position + N,
            direction: N,
        }],
        b'-' => vec![Light {
            position: light.position + E,
            direction: E,
        }],
        b'.' => vec![Light {
            position: light.position + E,
            direction: E,
        }],
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day16/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(46));
        assert_eq!(p2, Solution::Usize(51));
    }
}
