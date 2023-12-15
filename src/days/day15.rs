use crate::{Solution, SolutionPair};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Equal(u8),
    Minus,
}

#[derive(Debug, Clone)]
struct Label {
    label: Vec<u8>,
    instruction: Instruction,
}

impl Label {
    fn parse(s: &str) -> Self {
        match s.strip_suffix('-') {
            Some(label) => Self {
                instruction: Instruction::Minus,
                label: label.as_bytes().to_vec(),
            },
            None => {
                let (label, focal_length) = s.split_once('=').unwrap();
                let focal_length = focal_length.parse().unwrap();
                Self {
                    instruction: Instruction::Equal(focal_length),
                    label: label.as_bytes().to_vec(),
                }
            }
        }
    }

    fn focusing_power(&self, box_number: usize, lens_number: usize) -> usize {
        match self.instruction {
            Instruction::Equal(focal_length) => {
                (box_number + 1) * (lens_number + 1) * focal_length as usize
            }
            Instruction::Minus => 0,
        }
    }
}

fn hash(input: &[u8]) -> usize {
    let mut current: usize = 0;
    for c in input {
        current += *c as usize;
        current *= 17;
        current %= 256;
    }
    current
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.split(',').map(|s| s.as_bytes().to_vec()).collect()
}

pub fn solve(input: &str) -> SolutionPair {
    (p1(input), p2(input))
}

fn p1(input: &str) -> Solution {
    let input = parse(input);
    let sum = input.iter().map(|s| hash(s)).sum();

    Solution::Usize(sum)
}

fn p2(input: &str) -> Solution {
    const V: Vec<Label> = vec![];
    let mut buckets = vec![V; 256];
    let lenses = input.split(',').map(Label::parse).collect::<Vec<_>>();

    for lens in lenses {
        let bucket = &mut buckets[hash(&lens.label)];

        match lens.instruction {
            Instruction::Minus => bucket.retain(|l| l.label != lens.label),
            Instruction::Equal(_) => {
                if let Some(l) = bucket.iter_mut().find(|l| l.label == lens.label) {
                    l.instruction = lens.instruction;
                } else {
                    bucket.push(lens);
                }
            }
        }
    }

    let result = calculate_focusing_power(buckets);

    Solution::Usize(result)
}

fn calculate_focusing_power(buckets: Vec<Vec<Label>>) -> usize {
    buckets
        .into_iter()
        .enumerate()
        .map(|(box_number, bucket)| {
            bucket
                .into_iter()
                .enumerate()
                .map(|(lens_number, lens)| lens.focusing_power(box_number, lens_number))
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day15/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(1320));
        assert_eq!(p2, Solution::Usize(145));
    }
}
