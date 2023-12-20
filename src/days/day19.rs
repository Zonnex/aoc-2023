use std::collections::HashMap;

use regex::Regex;

use crate::{Solution, SolutionPair};

#[derive(Debug)]
struct Rule<'a> {
    category: Category,
    comparison: Comparison,
    dest: &'a str,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Comparison {
    Gt(usize),
    Lt(usize),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    dest: &'a str,
}

impl<'a> Workflow<'a> {
    fn parse(rules: &'a str) -> Self {
        let rule_regex = Regex::new(
            r"(?<rule>(?<category>[xmas])(?<comparison>[<>])(?<value>\d+):(?<dest>[a-zAR]+))",
        )
        .unwrap();

        let rules = rules.split(',').collect::<Vec<_>>();
        let (dest, rules) = rules.split_last().unwrap();

        let rules = rules
            .iter()
            .map(|rule| {
                let captures = rule_regex.captures(rule).unwrap();

                let category = match captures.name("category").unwrap().as_str() {
                    "x" => Category::X,
                    "m" => Category::M,
                    "a" => Category::A,
                    "s" => Category::S,
                    _ => unreachable!("Invalid input for Category"),
                };

                let value = captures
                    .name("value")
                    .map(|v| v.as_str().parse().unwrap())
                    .unwrap();

                let comparison = match captures.name("comparison").unwrap().as_str() {
                    ">" => Comparison::Gt(value),
                    "<" => Comparison::Lt(value),
                    _ => unreachable!("Invalid input for Comparison"),
                };

                let dest = captures.name("dest").unwrap().as_str();

                Rule {
                    category,
                    comparison,
                    dest,
                }
            })
            .collect::<Vec<_>>();

        Self {
            rules,
            dest: dest.trim(),
        }
    }

    fn run_rules(&self, part: &Part) -> &str {
        for rule in &self.rules {
            match (rule.category, rule.comparison, rule.dest) {
                (Category::X, Comparison::Gt(v), dest) => {
                    if part.x > v {
                        return dest;
                    }
                }
                (Category::X, Comparison::Lt(v), dest) => {
                    if part.x < v {
                        return dest;
                    }
                }
                (Category::M, Comparison::Gt(v), dest) => {
                    if part.m > v {
                        return dest;
                    }
                }
                (Category::M, Comparison::Lt(v), dest) => {
                    if part.m < v {
                        return dest;
                    }
                }
                (Category::A, Comparison::Gt(v), dest) => {
                    if part.a > v {
                        return dest;
                    }
                }
                (Category::A, Comparison::Lt(v), dest) => {
                    if part.a < v {
                        return dest;
                    }
                }
                (Category::S, Comparison::Gt(v), dest) => {
                    if part.s > v {
                        return dest;
                    }
                }
                (Category::S, Comparison::Lt(v), dest) => {
                    if part.s < v {
                        return dest;
                    }
                }
            }
        }

        self.dest
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn parse(input: &str) -> Self {
        let xmas = input
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .filter_map(|p| p.split_once('='))
            .filter_map(|(_, v)| v.parse().ok())
            .collect::<Vec<_>>();

        match xmas.as_slice() {
            [p0, p1, p2, p3] => Self {
                x: *p0,
                m: *p1,
                a: *p2,
                s: *p3,
            },
            _ => unreachable!("Invalid input"),
        }
    }

    fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let (parts, workflows) = parse(input);
    (p1(&parts, &workflows), p2(&workflows))
}

fn p1(parts: &[Part], workflows: &HashMap<&str, Workflow>) -> Solution {
    let start = workflows.get("in").unwrap();

    let mut accepted_rating = 0;

    for part in parts {
        let mut current = start;
        loop {
            let next = current.run_rules(part);
            if next == "A" {
                accepted_rating += part.rating();
                break;
            }
            if next == "R" {
                break;
            }
            current = workflows.get(next).unwrap();
        }
    }
    Solution::Usize(accepted_rating)
}

#[derive(Clone, Copy, Debug)]
struct Ranges {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl Ranges {
    fn split_gt(self, category: Category, value: usize) -> (Self, Self) {
        match category {
            Category::X => (
                Self {
                    x: (self.x.0, value),
                    m: self.m,
                    a: self.a,
                    s: self.s,
                },
                Self {
                    x: (value + 1, self.x.1),
                    m: self.m,
                    a: self.a,
                    s: self.s,
                },
            ),
            Category::M => (
                Self {
                    x: self.x,
                    m: (self.m.0, value),
                    a: self.a,
                    s: self.s,
                },
                Self {
                    x: self.x,
                    m: (value + 1, self.m.1),
                    a: self.a,
                    s: self.s,
                },
            ),
            Category::A => (
                Self {
                    x: self.x,
                    m: self.m,
                    a: (self.a.0, value),
                    s: self.s,
                },
                Self {
                    x: self.x,
                    m: self.m,
                    a: (value + 1, self.a.1),
                    s: self.s,
                },
            ),
            Category::S => (
                Self {
                    x: self.x,
                    m: self.m,
                    a: self.a,
                    s: (self.s.0, value),
                },
                Self {
                    x: self.x,
                    m: self.m,
                    a: self.a,
                    s: (value + 1, self.s.1),
                },
            ),
        }
    }

    fn split_lt(self, category: Category, value: usize) -> (Self, Self) {
        match category {
            Category::X => (
                Self {
                    x: (self.x.0, value - 1),
                    m: self.m,
                    a: self.a,
                    s: self.s,
                },
                Self {
                    x: (value, self.x.1),
                    m: self.m,
                    a: self.a,
                    s: self.s,
                },
            ),
            Category::M => (
                Self {
                    x: self.x,
                    m: (self.m.0, value - 1),
                    a: self.a,
                    s: self.s,
                },
                Self {
                    x: self.x,
                    m: (value, self.m.1),
                    a: self.a,
                    s: self.s,
                },
            ),
            Category::A => (
                Self {
                    x: self.x,
                    m: self.m,
                    a: (self.a.0, value - 1),
                    s: self.s,
                },
                Self {
                    x: self.x,
                    m: self.m,
                    a: (value, self.a.1),
                    s: self.s,
                },
            ),
            Category::S => (
                Self {
                    x: self.x,
                    m: self.m,
                    a: self.a,
                    s: (self.s.0, value - 1),
                },
                Self {
                    x: self.x,
                    m: self.m,
                    a: self.a,
                    s: (value, self.s.1),
                },
            ),
        }
    }

    fn combinations(&self) -> usize {
        let x = self.x.1 - self.x.0 + 1;
        let m = self.m.1 - self.m.0 + 1;
        let a = self.a.1 - self.a.0 + 1;
        let s = self.s.1 - self.s.0 + 1;
        x * m * a * s
    }

    fn contains(&self, category: Category, v: usize) -> bool {
        match category {
            Category::X => self.x.0 <= v && v <= self.x.1,
            Category::M => self.m.0 <= v && v <= self.m.1,
            Category::A => self.a.0 <= v && v <= self.a.1,
            Category::S => self.s.0 <= v && v <= self.s.1,
        }
    }
}

impl Default for Ranges {
    fn default() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }
}

fn p2(workflows: &HashMap<&str, Workflow>) -> Solution {
    let mut queue = vec![("in", Ranges::default())];
    let mut total = 0;

    while let Some((workflow_key, ranges)) = queue.pop() {
        if workflow_key == "A" {
            total += ranges.combinations();
            continue;
        }
        if workflow_key == "R" {
            continue;
        }
        let workflow = workflows.get(workflow_key).unwrap();
        let mut current = ranges;
        for rule in &workflow.rules {
            match rule.comparison {
                Comparison::Gt(v) => {
                    if current.contains(rule.category, v) {
                        let (smaller, greater) = current.split_gt(rule.category, v);
                        queue.push((rule.dest, greater));
                        current = smaller;
                    }
                }
                Comparison::Lt(v) => {
                    if current.contains(rule.category, v) {
                        let (smaller, greater) = current.split_lt(rule.category, v);
                        queue.push((rule.dest, smaller));
                        current = greater;
                    }
                }
            }
        }
        queue.push((workflow.dest, current));
    }
    Solution::Usize(total)
    // Solution::Usize(167409079868000)
}

fn parse(input: &str) -> (Vec<Part>, HashMap<&str, Workflow>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once('{').unwrap();
            let rest = rest.trim_end_matches('}');
            (name, Workflow::parse(rest))
        })
        .collect::<HashMap<_, _>>();

    let parts = parts.lines().map(Part::parse).collect();

    (parts, workflows)
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day19/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(19114));
        assert_eq!(p2, Solution::Usize(167409079868000));
    }
}
