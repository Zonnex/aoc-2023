use crate::{Solution, SolutionPair};

pub fn solve(_input: &str) -> SolutionPair {
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day20/test.txt");
        let (_p1, _p2) = super::solve(input);
        // assert_eq!(p1, expected);
        // assert_eq!(p2, expected);
    }
}
