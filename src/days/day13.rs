use crate::{Solution, SolutionPair};

struct Valley(Vec<Vec<u8>>);

impl Valley {
    fn reflection_indices<F>(&self, compare: F, size: usize, expected_diffs: usize) -> Option<(usize, usize)>
    where
        F: Fn(usize, usize) -> usize,
    {
        let check_outwards = |mut l: usize, mut r: usize| {
            let mut diffs = 0;
            while l > 0 && r < size - 1 {
                l -= 1;
                r += 1;
                diffs += compare(l, r);
            }
            diffs
        };

        let mut l = 0;
        let mut r = 1;
        while l < size - 1 {
            if compare(l, r) + check_outwards(l, r) == expected_diffs {
                return Some((l, r));
            }
            l += 1;
            r += 1;
        }
        None
    }

    fn reflection_row_indices(&self, expected_diffs: usize) -> Option<(usize, usize)> {
        let Valley(matrix) = self;
        let count_diffs = |i: usize, j: usize| {
            matrix[i]
                .iter()
                .zip(matrix[j].iter())
                .filter(|(&a, &b)| a != b)
                .count()
        };
        self.reflection_indices(count_diffs, matrix.len(), expected_diffs)
    }

    fn reflection_column_indices(&self, expected_diffs: usize) -> Option<(usize, usize)> {
        let Valley(matrix) = self;
        let count_diffs = |j: usize, k: usize| {
            matrix
                .iter()
                .map(|row| row[j])
                .zip(matrix.iter().map(|row| row[k]))
                .filter(|(a, b)| a != b)
                .count()
        };
        self.reflection_indices(count_diffs, matrix[0].len(), expected_diffs)
    }

    fn fold(&self, diffs: usize, acc: (usize, usize)) -> (usize, usize) {
        let (mut columns, mut rows) = acc;

        if let Some((_, y)) = self.reflection_row_indices(diffs) {
            rows += y;
        } else if let Some((_, x)) = self.reflection_column_indices(diffs) {
            columns += x;
        }

        (columns, rows)
    }
}

struct PuzzleInput {
    valleys: Vec<Valley>,
}

impl PuzzleInput {
    fn parse(input: &str) -> Self {
        let valleys = input
            .split("\n\n")
            .map(|field| {
                let valley = field
                    .lines()
                    .map(|line| line.as_bytes().to_vec())
                    .collect::<Vec<_>>();
                Valley(valley)
            })
            .collect::<Vec<_>>();

        PuzzleInput { valleys }
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let input = PuzzleInput::parse(input);
    (p1(&input), p2(input))
}

fn p1(input: &PuzzleInput) -> Solution {
    let (columns, rows) = input
        .valleys
        .iter()
        .fold((0, 0), |acc, valley| valley.fold(0, acc));

    Solution::Usize(columns + rows * 100)
}

fn p2(input: PuzzleInput) -> Solution {
    let (columns, rows) = input
        .valleys
        .iter()
        .fold((0, 0), |acc, valley| valley.fold(1, acc));

    Solution::Usize(columns + rows * 100)
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day13/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(405));
        assert_eq!(p2, Solution::Usize(400));
    }
}
