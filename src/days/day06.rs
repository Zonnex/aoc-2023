use crate::{Solution, SolutionPair};

pub fn solve(input: &str) -> SolutionPair {
    (p1(input), p2(input))
}

fn p1(input: &str) -> Solution {
    fn parse_line(line: &str) -> Vec<usize> {
        line.split_whitespace()
            .skip(1)
            .map(str::parse::<usize>)
            .filter_map(Result::ok)
            .collect()
    }
    let (l1, l2) = input.split_once('\n').unwrap();
    let times = parse_line(l1);
    let records = parse_line(l2);

    times
        .into_iter()
        .zip(records)
        .map(|(t, d)| f(t, d))
        .product::<usize>()
        .into()
}

fn p2(input: &str) -> Solution {
    fn parse_line(line: &str) -> usize {
        line.split_whitespace()
            .skip(1)
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    }
    let (l1, l2) = input.split_once('\n').unwrap();
    let times = parse_line(l1);
    let records = parse_line(l2);

    f(times, records).into()
}

fn f(t: usize, d: usize) -> usize {
    let (t, d) = (t as f64, d as f64);
    let center = t / 2.0;
    let diff = ((t / 2.0).powi(2) - d).sqrt();

    let root1 = match center - diff {
        x if x.fract() == 0.0 => x + 1.0,
        x => x.ceil(),
    } as usize;

    let root2 = match center + diff {
        x if x.fract() == 0.0 => x - 1.0,
        x => x.floor(),
    } as usize;
    
    let c1 = root1;
    let c2 = root2;
    (c1..=c2).count()
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day06/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(288));
        assert_eq!(p2, Solution::Usize(71503));
    }
}
