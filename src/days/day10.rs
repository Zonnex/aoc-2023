use std::collections::{HashMap, HashSet, VecDeque};

use crate::{utils::vector_2d::*, Solution, SolutionPair};

#[derive(Debug)]
pub(crate) struct Map {
    tiles: HashMap<Vector2, u8>,
    size: (usize, usize),
}
impl Map {
    fn get(&self, position: Vector2) -> Option<&u8> {
        self.tiles.get(&position)
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let map = parse_map(input);

    let start = map
        .tiles
        .iter()
        .find(|(_, c)| **c == b'S')
        .map(|(pos, _)| *pos)
        .unwrap();

    let (p1, pipe) = p1(&map, start);

    (p1, p2(map, pipe, start))
}

fn parse_map(input: &str) -> Map {
    let mut map = HashMap::new();
    for (y, line) in input.lines().rev().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            map.insert(Vector2::new_usize(x, y), c);
        }
    }
    let max_x = input.lines().next().unwrap().len() - 1;
    let max_y = input.lines().count() - 1;
    Map {
        tiles: map,
        size: (max_x, max_y),
    }
}

fn p1(map: &Map, start: Vector2) -> (Solution, HashSet<Vector2>) {
    let symbol = get_start_symbol(map, start);
    let mut todo = get_exits(symbol)
        .into_iter()
        .map(|d| start + d)
        .collect::<VecDeque<_>>();

    let mut visits = HashSet::new();
    visits.insert(start);

    while let Some(pos) = todo.pop_front() {
        let c = match map.tiles.get(&pos) {
            Some(&c) => c,
            None => continue,
        };

        if !visits.insert(pos) {
            continue;
        }

        get_exits(c)
            .into_iter()
            .map(|d| pos + d)
            .filter(|x| !visits.contains(x))
            .for_each(|x| todo.push_back(x));
    }

    (Solution::from(visits.len().div_ceil(2)), visits)
}

fn get_start_symbol(map: &Map, start: Vector2) -> u8 {
    let results = [N, E, W, S]
        .into_iter()
        .map(|dir| {
            let pos = start + dir;
            if let Some(c) = map.get(pos) {
                let exits = get_exits(*c);
                let inverse = dir.inverse();
                return exits.contains(&inverse);
            }
            false
        })
        .collect::<Vec<_>>();

    match results.as_slice() {
        [true, true, false, false] => b'L', // North and East
        [false, true, false, true] => b'F', // East and South
        [false, false, true, true] => b'7', // South and West
        [true, false, true, false] => b'J', // North and West
        [true, false, false, true] => b'|', // North and South
        [false, true, true, false] => b'-', // East and West
        _ => unreachable!(),
    }
}

fn get_exits(this: u8) -> [Vector2; 2] {
    match this {
        b'|' => [N, S],
        b'-' => [W, E],
        b'L' => [N, E],
        b'F' => [S, E],
        b'7' => [W, S],
        b'J' => [W, N],
        _ => unreachable!("Invalid exit char: {}", this),
    }
}

fn p2(mut map: Map, pipe: HashSet<Vector2>, start: Vector2) -> Solution {
    let symbol = get_start_symbol(&map, start);
    map.tiles.iter_mut().for_each(|(pos, c)| {
        if !pipe.contains(pos) {
            *c = b'.';
        }
    });

    map.tiles.remove(&start);
    map.tiles.insert(start, symbol);

    print_map(&map);

    let mut total = 0usize;
    let (x_max, y_max) = map.size;
    for row in 0..=y_max {
        let mut inside = false;
        for current in 0..=x_max {
            let next = current + 1;
            let current_pos = Vector2::new_usize(current, row);
            let next_pos = Vector2::new_usize(next, row);

            let current_char = map.get(current_pos);
            let next_char = map.get(next_pos);
            match (current_char, next_char) {
                (Some(b'F'), Some(b'J')) => inside = !inside,
                (Some(b'L'), Some(b'7')) => inside = !inside,
                (Some(b'|'), _) => inside = !inside,
                (Some(b'.'), _) => {
                    if inside {
                        total += 1
                    }
                }
                _ => continue,
            }
        }
    }

    /*
        Hitta alla delar av loopen och sätt rätt värde på S-pipen (görs i del 1).
        Byt ut alla bitar som inte är en del av loopen till '.'
        Ta bort alla '-'
        Byt ut alla FJ och L7 till '|'
    */
    // map.into_iter();
    Solution::from(total)
}

fn print_map(map: &Map) {
    let (max_x, max_y) = map.size;
    for row in (0..=max_y).rev() {
        for column in 0..=max_x {
            let pos = Vector2::new_usize(column, row);
            let c = map.tiles.get(&pos).unwrap();
            let c = match c {
                b'F' => '╔',
                b'L' => '╚',
                b'7' => '╗',
                b'|' => '║',
                b'J' => '╝',
                b'-' => '═',
                _ => *c as char,
            };
            print!("{}", c);
        }
        println!()
    }
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day10/real.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(4));
        assert_eq!(p2, Solution::Usize(1));
    }
}
