use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

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

fn p1(map: &Map, start: Vector2) -> (Solution, Vec<Vector2>) {
    let symbol = get_start_symbol(map, start);
    let mut todo = get_exits(symbol)
        .into_iter()
        .map(|d| start + d)
        .collect::<Vec<_>>();

    let mut pipe = Vec::new();
    let mut visits = HashSet::new();
    visits.insert(start);
    pipe.push(start);

    while let Some(pos) = todo.pop() {
        let c = match map.tiles.get(&pos) {
            Some(&c) => c,
            None => continue,
        };

        if !visits.insert(pos) {
            continue;
        }
        pipe.push(pos);

        get_exits(c)
            .into_iter()
            .map(|d| pos + d)
            .filter(|x| !visits.contains(x))
            .for_each(|x| todo.push(x));
    }

    (Solution::from(visits.len().div_ceil(2)), pipe)
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

fn get_exits(shape: u8) -> [Vector2; 2] {
    match shape {
        b'|' => [N, S],
        b'-' => [W, E],
        b'L' => [N, E],
        b'F' => [S, E],
        b'7' => [W, S],
        b'J' => [W, N],
        _ => unreachable!("Invalid exit char: {}", shape),
    }
}

fn p2(mut map: Map, pipe: Vec<Vector2>, start: Vector2) -> Solution {
    let symbol = get_start_symbol(&map, start);
    map.tiles.iter_mut().for_each(|(pos, c)| {
        if !pipe.contains(pos) {
            *c = b'.';
        }
    });

    map.tiles.remove(&start);
    map.tiles.insert(start, symbol);

    let mut current = pipe[0];
    let mut inside = HashSet::new();
    let mut pipe_iterator = pipe.into_iter().skip(1);
    while let Some(next) = pipe_iterator.next() {
        let direction = next - current;
        let tiles = get_inside_tiles(&map, current, direction)
            .iter()
            .map(|&d| current + d)
            .filter_map(|p| map.tiles.get(&p).map(|&c| (p, c)))
            .filter(|(p, c)| c == &b'.' && !inside.contains(p))
            .collect::<Vec<_>>();

        for tile in tiles {
            floodfill(&map, &mut inside, tile.0);
        }
        current = next;
    }

    for inside in inside.iter() {
        let tile = map.tiles.get_mut(inside).unwrap();
        *tile = b'0';
    }

    Solution::from(inside.len())
}

fn floodfill(map: &Map, inside: &mut HashSet<Vector2>, start: Vector2) {
    let mut todo = VecDeque::new();

    todo.push_back(start);

    while let Some(pos) = todo.pop_front() {
        let c = match map.tiles.get(&pos) {
            Some(&c) => c,
            None => continue,
        };

        if c == b'.' && inside.insert(pos) {
            todo.push_back(pos + N);
            todo.push_back(pos + E);
            todo.push_back(pos + S);
            todo.push_back(pos + W);
        }
    }
}

fn get_inside_tiles(map: &Map, position: Vector2, direction: Vector2) -> Vec<Vector2> {
    const SW: Vector2 = Vector2 { x: -1, y: -1 };
    const SE: Vector2 = Vector2 { x: 1, y: -1 };
    const NW: Vector2 = Vector2 { x: -1, y: 1 };
    const NE: Vector2 = Vector2 { x: 1, y: 1 };

    let shape = map.tiles.get(&position).unwrap();
    match (shape, direction) {
        (b'|', N) => vec![E],
        (b'|', S) => vec![W],
        (b'-', W) => vec![N],
        (b'-', E) => vec![S],
        (b'L', N) => vec![NE],
        (b'L', E) => vec![W, SW, S],
        (b'F', E) => vec![SE],
        (b'F', S) => vec![W, NW, N],
        (b'7', S) => vec![SW],
        (b'7', W) => vec![E, NE, N],
        (b'J', W) => vec![NW],
        (b'J', N) => vec![E, SE, S],
        _ => unreachable!("Invalid exit char: {}", shape),
    }
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
        let input = include_str!("../../input/day10/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(4));
        assert_eq!(p2, Solution::Usize(1));
    }
}
