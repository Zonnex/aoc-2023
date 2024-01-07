use crate::{Solution, SolutionPair};

pub fn solve(input: &str) -> SolutionPair {
    let matches = parse_input(input);

    (p1(&matches), p2(&matches))
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| {
            let (_, card) = line.split_once(": ").unwrap();
            let (winning_numbers, actual) = card.split_once(" | ").unwrap();

            // hashset replacement
            let mut wins = [false; 100];

            for number in winning_numbers
                .split_whitespace()
                .filter_map(|n| n.parse::<usize>().ok())
            {
                wins[number] = true;
            }

            actual
                .split_whitespace()
                .filter_map(|n| n.parse::<usize>().ok())
                .filter(|n| wins[*n])
                .count()
        })
        .collect::<Vec<_>>()
}

fn p1(matches: &[usize]) -> Solution {
    let answer = matches.iter().map(|&n| (1 << n) >> 1).sum();

    Solution::Usize(answer)
}

fn p2(matches: &Vec<usize>) -> Solution {
    let mut copies = vec![1; matches.len()];

    for (i, &n) in matches.iter().enumerate() {
        for j in 1..=n {
            copies[i + j] += copies[i];
        }
    }

    let total = copies.iter().sum();

    Solution::Usize(total)
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day04/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(13));
        assert_eq!(p2, Solution::Usize(30));
    }
}
