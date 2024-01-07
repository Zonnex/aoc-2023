use crate::{Solution, SolutionPair};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector3 {
    x: i128,
    y: i128,
    z: i128,
}

impl Vector3 {
    fn cross(self, other: Self) -> Self {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    // Changes the magnitude (but not direction) of the vector.
    // Prevents numeric overflow.
    fn gcd(self) -> Self {
        let gcd = gcd(self.x.abs(), gcd(self.y.abs(), self.z.abs()));
        Vector3 {
            x: self.x / gcd,
            y: self.y / gcd,
            z: self.z / gcd,
        }
    }

    fn sum(self) -> i128 {
        self.x + self.y + self.z
    }
}

fn gcd(abs_1: i128, abs_2: i128) -> i128 {
    if abs_1 == 0 {
        abs_2
    } else if abs_2 == 0 {
        abs_1
    } else {
        let mut a = abs_1;
        let mut b = abs_2;
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }
}

impl std::ops::Mul<i128> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: i128) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point(f64, f64);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Intersects {
    Past,
    Future,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Hail {
    position: Vector3,
    velocity: Vector3,
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
            ([px, py, pz], [vx, vy, vz]) => {
                let position = Vector3 {
                    x: *px as i128,
                    y: *py as i128,
                    z: *pz as i128,
                };
                let velocity = Vector3 {
                    x: *vx as i128,
                    y: *vy as i128,
                    z: *vz as i128,
                };
                Hail { position, velocity }
            }
            _ => panic!("Invalid position"),
        }
    }
}

pub fn solve(input: &str) -> SolutionPair {
    const MIN: f64 = 200_000_000_000_000.0;
    const MAX: f64 = 400_000_000_000_000.0;
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
    // take 3 hailstones, any 3
    let a = hailstones[0];
    let b = hailstones[1];
    let c = hailstones[2];

    let (p0, v0) = (a.position, a.velocity);
    let (p1, v1) = (b.position, b.velocity);
    let (p2, v2) = (c.position, c.velocity);

    // compute relative positions and velocities
    // this makes calculations go through origin
    let p3 = p1 - p0;
    let p4 = p2 - p0;
    let v3 = v1 - v0;
    let v4 = v2 - v0;

    let q = v3.cross(p3).gcd();
    let r = v4.cross(p4).gcd();
    let direction = q.cross(r).gcd();

    let t1 = (p3.y * direction.x - p3.x * direction.y) / (v3.x * direction.y - v3.y * direction.x);
    let t2 = (p4.y * direction.x - p4.x * direction.y) / (v4.x * direction.y - v4.y * direction.x);
    let dt = t2 - t1;
    let st = t2 * t1;

    let a = t2 * (p0 + p3).sum();
    let b = t1 * (p0 + p4).sum();
    let c = st * (v3 - v4).sum();
    let answer = (a - b + c) / dt;

    Solution::I128(answer)
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
        assert_eq!(p2, Solution::I128(47));
    }
}
