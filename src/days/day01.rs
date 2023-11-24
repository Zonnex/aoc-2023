use crate::{Solution, SolutionPair};

pub fn solve(input: &str) -> SolutionPair {
    (p1(input), p2(input))
}

fn get_digits(line: &str) -> u32 {
    let mut chars = line.chars().filter_map(|c| c.to_digit(10));

    let first_digit = chars.next().unwrap();
    let last_digit = chars.last().unwrap_or(first_digit);
    let res = 10 * first_digit + last_digit;
    res
}

fn p1(input: &str) -> Solution {
    input.lines().map(get_digits).sum::<u32>().into()
}

fn p2(input: &str) -> Solution {
    input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
        .lines()
        .map(get_digits)
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let p1_input = include_str!("../../input/day01/test_p1.txt");
        let p2_input = include_str!("../../input/day01/test_p2.txt");
        let (p1, p2) = (super::p1(p1_input), super::p2(p2_input));

        println!("p1: {}, p2: {}", p1, p2);
        assert_eq!(p1, Solution::U32(142));
        assert_eq!(p2, Solution::U32(281));
    }
}
