use std::collections::{HashMap, HashSet};

use crate::{Solution, SolutionPair};

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_graph(input: &str) -> Graph {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    let edge_iterator = input.lines().flat_map(|line| {
        let (from, to) = line.split_once(": ").unwrap();

        let from = std::iter::repeat(from);
        to.split(' ').zip(from)
    });

    for (a, b) in edge_iterator {
        graph.entry(a).or_default().push(b);
        graph.entry(b).or_default().push(a);
    }

    graph
}

pub fn solve(input: &str) -> SolutionPair {
    let graph = parse_graph(input);
    (p1(graph), Solution::None)
}

fn p1(mut graph: Graph<'_>) -> Solution {
    let graph = graph
        .drain()
        .map(|(k, mut edges)| {
            match k {
                "nmz" => edges.retain(|&x| x != "mnl"),
                "mnl" => edges.retain(|&x| x != "nmz"),
                "vgf" => edges.retain(|&x| x != "jpn"),
                "jpn" => edges.retain(|&x| x != "vgf"),
                "fdb" => edges.retain(|&x| x != "txm"),
                "txm" => edges.retain(|&x| x != "fdb"),
                _ => {}
            };

            (k, edges)
        })
        .collect::<Graph>();

    let left = graph.get("nmz").unwrap().first().unwrap();
    let right = graph.get("mnl").unwrap().first().unwrap();

    let left_cluster = count_cluster(&graph, left);
    let right_cluster = count_cluster(&graph, right);

    Solution::Usize(left_cluster * right_cluster)
}

fn count_cluster(graph: &Graph<'_>, start: &str) -> usize {
    let mut cluster = HashSet::new();
    let mut queue = vec![start];

    while let Some(node) = queue.pop() {
        if cluster.contains(node) {
            continue;
        }

        cluster.insert(node);

        for edge in graph.get(node).unwrap() {
            queue.push(edge);
        }
    }

    cluster.len()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day25/test.txt");
        let (_p1, _p2) = super::solve(input);
        // assert_eq!(p1, expected);
        // assert_eq!(p2, expected);
    }
}
