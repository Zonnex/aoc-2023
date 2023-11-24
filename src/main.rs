mod days;
mod etc;

use days::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25,
};
use etc::solution::Solution;
use std::env;
use std::time::Instant;

pub type SolutionPair = (Solution, Solution);

mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    let days: Vec<u8> = match args.len() {
        0..=1 => (1..=25).collect(),
        2 => {
            let val = args[1].parse().unwrap();
            vec![val]
        }
        _ => (1..=args.iter().last().unwrap().parse().unwrap()).collect(),
    };

    let mut runtime = 0.0;

    for day in days {
        let func = get_day_solver(day);

        let time = Instant::now();
        let input = get_input(day);
        let (p1, p2) = func(input);
        let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;

        println!("\n=== Day {:02} ===", day);
        println!("  · Part 1: {}", p1);
        println!("  · Part 2: {}", p2);
        println!("  · Elapsed: {:.4} ms", elapsed_ms);

        runtime += elapsed_ms;
    }

    println!("Total runtime: {:.4} ms", runtime);
}

fn get_input(day: u8) -> &'static str {
    match day {
        1 => include_str!("../input/day01/real.txt"),
        2 => include_str!("../input/day02/real.txt"),
        3 => include_str!("../input/day03/real.txt"),
        4 => include_str!("../input/day04/real.txt"),
        5 => include_str!("../input/day05/real.txt"),
        6 => include_str!("../input/day06/real.txt"),
        7 => include_str!("../input/day07/real.txt"),
        8 => include_str!("../input/day08/real.txt"),
        9 => include_str!("../input/day09/real.txt"),
        10 => include_str!("../input/day10/real.txt"),
        11 => include_str!("../input/day11/real.txt"),
        12 => include_str!("../input/day12/real.txt"),
        13 => include_str!("../input/day13/real.txt"),
        14 => include_str!("../input/day14/real.txt"),
        15 => include_str!("../input/day15/real.txt"),
        16 => include_str!("../input/day16/real.txt"),
        17 => include_str!("../input/day17/real.txt"),
        18 => include_str!("../input/day18/real.txt"),
        19 => include_str!("../input/day19/real.txt"),
        20 => include_str!("../input/day20/real.txt"),
        21 => include_str!("../input/day21/real.txt"),
        22 => include_str!("../input/day22/real.txt"),
        23 => include_str!("../input/day23/real.txt"),
        24 => include_str!("../input/day24/real.txt"),
        25 => include_str!("../input/day25/real.txt"),
        _ => unreachable!(),
    }
}

fn get_day_solver(day: u8) -> fn(&str) -> SolutionPair {
    match day {
        1 => day01::solve,
        2 => day02::solve,
        3 => day03::solve,
        4 => day04::solve,
        5 => day05::solve,
        6 => day06::solve,
        7 => day07::solve,
        8 => day08::solve,
        9 => day09::solve,
        10 => day10::solve,
        11 => day11::solve,
        12 => day12::solve,
        13 => day13::solve,
        14 => day14::solve,
        15 => day15::solve,
        16 => day16::solve,
        17 => day17::solve,
        18 => day18::solve,
        19 => day19::solve,
        20 => day20::solve,
        21 => day21::solve,
        22 => day22::solve,
        23 => day23::solve,
        24 => day24::solve,
        25 => day25::solve,
        _ => unimplemented!(),
    }
}
