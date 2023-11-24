use std::{collections::HashMap, ops::RangeInclusive};

use regex::Regex;

use crate::{utils::vector_2d::Vector2, Solution, SolutionPair};

#[derive(Debug, Default)]
struct Grid {
    map: HashMap<Vector2, Entity>,
    part_numbers: Vec<(Vector2, Number)>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy)]
struct Width(usize);

#[derive(Debug, Clone, Copy)]
struct Value(usize);

#[derive(Debug, Clone, Copy)]
struct Number(Value, Width);

impl Number {
    fn border(&self, position: Vector2) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
        let Number(_, Width(width)) = self;

        let y_start = position.y - 1;
        let y_end = position.y + 1;
        let x_start = position.x - 1;
        let x_end = position.x + *width as isize;

        let y_range = y_start..=y_end;
        let x_range = x_start..=x_end;

        (x_range, y_range)
    }

    fn value(self) -> usize {
        let Number(Value(v), _) = self;
        v
    }
}

#[derive(Debug)]
enum Entity {
    Digit(usize),
    Dot,
    Symbol(char),
}

pub fn solve(input: &str) -> SolutionPair {
    let grid = parse_grid(input);
    print_grid(&grid);

    (Solution::USize(p1(&grid)), Solution::USize(p2(&grid)))
}

fn p1(grid: &Grid) -> usize {
    grid.part_numbers
        .iter()
        .map(|(_, Number(Value(v), _))| v)
        .sum()
}

fn p2(grid: &Grid) -> usize {
    let mut gears = HashMap::new();

    for (position, number) in &grid.part_numbers {
        let (x_range, y_range) = number.border(*position);

        for y in y_range {
            for x in x_range.clone() {
                let position = Vector2::new(x, y);
                if let Some(Entity::Symbol(c)) = grid.map.get(&position) {
                    if c == &'*' {
                        gears.entry(position).or_insert_with(Vec::new).push(*number);
                    }
                } else {
                    continue;
                }
            }
        }
    }

    gears
        .into_iter()
        .map(|(_, value)| value)
        .filter(|value| value.len() == 2)
        .map(|gear| gear.into_iter().map(Number::value).product::<usize>())
        .sum()
}

fn parse_grid(input: &str) -> Grid {
    let mut grid = input
        .lines()
        .enumerate()
        .fold(Grid::default(), |mut grid, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let pos = Vector2::new_usize(x, y);
                let entity = match c {
                    '0'..='9' => Entity::Digit(c.to_digit(10).unwrap() as usize),
                    '.' => Entity::Dot,
                    _ => Entity::Symbol(c),
                };

                grid.map.insert(pos, entity);
                grid.width = grid.width.max(x);
                grid.height = grid.height.max(y);
            });

            grid
        });

    grid.part_numbers = find_numbers(input)
        .into_iter()
        .filter(|(position, number)| should_keep_number(&grid, position, number))
        .collect::<Vec<_>>();

    grid
}

fn find_numbers(input: &str) -> Vec<(Vector2, Number)> {
    let re = Regex::new(r"\d+").unwrap();

    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            re.find_iter(line).map(move |m| {
                let position = Vector2::new_usize(m.start(), y);
                (
                    position,
                    Number(
                        Value(m.as_str().parse().unwrap()),
                        Width(m.end() - m.start()),
                    ),
                )
            })
        })
        .collect()
}

fn should_keep_number(grid: &Grid, position: &Vector2, number: &Number) -> bool {
    let (x_range, y_range) = number.border(*position);

    for y in y_range {
        for x in x_range.clone() {
            let pos = Vector2::new(x, y);
            if let Some(entity) = grid.map.get(&pos) {
                match entity {
                    Entity::Symbol(_) => return true,
                    _ => continue,
                }
            }
        }
    }
    false
}

fn print_grid(grid: &Grid) {
    for y in 0..=grid.height {
        for x in 0..=grid.width {
            let pos = Vector2::new_usize(x, y);
            if let Some(n) = grid.map.get(&pos) {
                match n {
                    Entity::Digit(n) => print!("{}", n),
                    Entity::Dot => print!("."),
                    Entity::Symbol(c) => print!("{}", c),
                }
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day03/test.txt");
        let (p1, p2) = super::solve(input);

        assert_eq!(p1, Solution::USize(4361));
        assert_eq!(p2, Solution::USize(467835));
    }
}
