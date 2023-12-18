use crate::{utils::vector_2d::*, Solution, SolutionPair};

#[derive(Debug)]
struct Instruction {
    dir: Vector2,
    steps: usize,
    color: String,
}

impl Instruction {
    fn parse(input: &str) -> Vec<Self> {
        input
            .lines()
            .map(|line| {
                let parts = line.split(' ').collect::<Vec<_>>();

                match parts.as_slice() {
                    [dir, steps, color] => {
                        let color = color[2..color.len() - 1].to_string();
                        Self {
                            steps: steps.parse::<usize>().unwrap(),
                            color: color.to_string(),
                            dir: match *dir {
                                "U" => N,
                                "D" => S,
                                "L" => W,
                                "R" => E,
                                _ => unreachable!("Invalid direction"),
                            },
                        }
                    }
                    _ => unreachable!("Invalid input"),
                }
            })
            .collect()
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let instructions = Instruction::parse(input);
    (p1(&instructions), p2(&instructions))
}

fn p1(instructions: &[Instruction]) -> Solution {
    let polygon = to_polygon_p1(instructions);
    let (border, interior) = border_and_interior(&polygon);
    Solution::Usize(border + interior)
}

fn p2(instructions: &[Instruction]) -> Solution {
    let polygon = to_polygon_p2(instructions);
    let (border, interior) = border_and_interior(&polygon);
    Solution::Usize(border + interior)
}

fn to_polygon_p1(instructions: &[Instruction]) -> Vec<Vector2> {
    let mut polygon = vec![Vector2::new(0, 0)];
    let mut current = Vector2::new(0, 0);
    for instruction in instructions {
        current += instruction.dir * instruction.steps;
        polygon.push(current);
    }
    polygon
}

fn to_polygon_p2(instructions: &[Instruction]) -> Vec<Vector2> {
    let mut polygon = vec![Vector2::new(0, 0)];
    let mut current = Vector2::new(0, 0);
    for instruction in instructions {
        let (steps, dir) = instruction.color.split_at(5);

        let dir = match dir {
            "0" => E,
            "1" => S,
            "2" => W,
            "3" => N,
            _ => unreachable!("Invalid direction"),
        };
        let steps = isize::from_str_radix(steps, 16).unwrap() as usize;

        current += dir * steps;
        polygon.push(current);
    }
    polygon
}

fn area(polygon: &[Vector2]) -> usize {
    let s1 = polygon
        .iter()
        .zip(polygon.iter().cycle().skip(1))
        .map(|(&a, &b)| a.x * b.y)
        .sum::<isize>();

    let s2 = polygon
        .iter()
        .zip(polygon.iter().cycle().skip(1))
        .map(|(&a, &b)| a.y * b.x)
        .sum::<isize>();

    (s1 - s2).unsigned_abs()
}

fn border(polygon: &[Vector2]) -> usize {
    polygon
        .iter()
        .zip(polygon.iter().cycle().skip(1))
        .map(|(&a, &b)| (a - b).manhattan_distance())
        .sum::<usize>()
}

fn border_and_interior(polygon: &[Vector2]) -> (usize, usize) {
    let area = area(polygon) / 2;
    let border = border(polygon);
    let interior = area - (border / 2 - 1);
    (border, interior)
}

#[cfg(test)]
mod tests {
    use crate::{days::day18::*, etc::Solution};
    #[test]
    fn test_shoelace_formula() {
        let polygon = vec![
            Vector2::new(6, 0),
            Vector2::new(6, -5),
            Vector2::new(4, -5),
            Vector2::new(4, -7),
            Vector2::new(6, -7),
            Vector2::new(6, -9),
            Vector2::new(1, -9),
            Vector2::new(1, -7),
            Vector2::new(0, -7),
            Vector2::new(0, -5),
            Vector2::new(2, -5),
            Vector2::new(2, -2),
            Vector2::new(0, -2),
            Vector2::new(0, 0),
        ];
        assert_eq!(border_and_interior(&polygon), (38, 24));
    }

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day18/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(62));
        assert_eq!(p2, Solution::Usize(952_408_144_115));
    }
}
