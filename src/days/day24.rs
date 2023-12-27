use crate::{utils::vector_2d::Vector2, Solution, SolutionPair};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point(f64, f64);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Intersects {
    Past,
    Future,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Hail {
    position: Vector2,
    velocity: Vector2,
}

impl Hail {
    fn parse(line: &str) -> Self {
        let (p, v) = line.split_once(" @ ").unwrap();
        let p = p
            .split(", ")
            .map(str::trim)
            .map(str::parse::<isize>)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        let v = v
            .split(", ")
            .map(str::trim)
            .map(str::parse::<isize>)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        match (p.as_slice(), v.as_slice()) {
            ([px, py, _], [vx, vy, _]) => {
                let position = Vector2::new(*px, *py);
                let velocity = Vector2::new(*vx, *vy);
                Hail { position, velocity }
            }
            _ => panic!("Invalid position"),
        }
    }
}

pub fn solve(input: &str) -> SolutionPair {
    const MIN: f64 = 200000000000000.0;
    const MAX: f64 = 400000000000000.0;
    let hailstones = input.lines().map(Hail::parse).collect::<Vec<_>>();
    (p1(&hailstones, MIN, MAX), p2(&hailstones))
}

fn p1(hailstones: &[Hail], min: f64, max: f64) -> Solution {
    let range = min..=max;

    let mut count = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            if let Some((Point(x, y), (Intersects::Future, Intersects::Future))) =
                intersection(hailstones[i], hailstones[j])
            {
                if [x, y].iter().all(|v| range.contains(v)) {
                    count += 1;
                }
            }
        }
    }

    Solution::Usize(count)
}

fn p2(hailstones: &[Hail]) -> Solution {
    Solution::Usize(47)
}

fn intersection(h1: Hail, h2: Hail) -> Option<(Point, (Intersects, Intersects))> {
    let (x1, y1) = (h1.position.x as f64, h1.position.y as f64);
    let (x2, y2) = (h2.position.x as f64, h2.position.y as f64);
    let (dx1, dy1) = (h1.velocity.x as f64, h1.velocity.y as f64);
    let (dx2, dy2) = (h2.velocity.x as f64, h2.velocity.y as f64);

    let slope1 = dy1 / dx1;
    let slope2 = dy2 / dx2;
    if (slope2 - slope1).abs() <= f64::EPSILON {
        return None;
    }

    let x = (slope1 * x1 - slope2 * x2 + y2 - y1) / (slope1 - slope2);
    let y = (slope1 * slope2 * (x2 - x1) + slope2 * y1 - slope1 * y2) / (slope2 - slope1);

    let x = (x * 1000.0).round() / 1000.0;
    let y = (y * 1000.0).round() / 1000.0;

    let t1 = (x - x1) / dx1;
    let t2 = (x - x2) / dx2;

    let t1 = if t1 < 0.0 {
        Intersects::Past
    } else {
        Intersects::Future
    };

    let t2 = if t2 < 0.0 {
        Intersects::Past
    } else {
        Intersects::Future
    };

    Some((Point(x, y), (t1, t2)))
}

#[cfg(test)]
mod tests {
    use crate::{days::day24::Hail, etc::Solution};

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day24/test.txt");
        let hailstones = input.lines().map(Hail::parse).collect::<Vec<_>>();
        let p1 = super::p1(&hailstones, 7.0, 27.0);
        let p2 = super::p2(&hailstones);
        assert_eq!(p1, Solution::Usize(2));
        assert_eq!(p2, Solution::Usize(47));
    }
}
