use std::collections::{HashMap, VecDeque};

use crate::{Solution, SolutionPair};

pub fn solve(input: &str) -> SolutionPair {
    let config = parse_input(input);
    (p1(&config), p2(&config))
}

fn p1(config: &HashMap<&str, (Module, Vec<&str>)>) -> Solution {
    Solution::Usize(simulate(config, 1000))
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Module {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum State {
    On,
    Off,
}

fn simulate(config: &HashMap<&str, (Module, Vec<&str>)>, rounds: usize) -> usize {
    let mut conjunctions = config
        .iter()
        .filter_map(|(name, (t, _))| {
            if *t == Module::Conjunction {
                Some((name, HashMap::new()))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    let mut flip_flops = config
        .iter()
        .filter_map(|(name, (t, _))| {
            if *t == Module::FlipFlop {
                Some((name, State::Off))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    for (name, (_, dests)) in config.iter() {
        for dest in dests {
            if conjunctions.contains_key(dest) {
                let vec = conjunctions.get_mut(dest).unwrap();
                vec.insert(name, Pulse::Low);
            }
        }
    }

    // println!("Conjunctions: {:?}", conjunctions);
    // println!("Flip-flops: {:?}", flip_flops);

    let mut queue = VecDeque::new();
    let (mut low, mut high) = (0, 0);

    let (_, dests) = config.get("broadcaster").unwrap();
    for _round in 0..rounds {
        low += 1; // button -> broadcaster is low pulse
        for dest in dests {
            queue.push_back(("broadcaster", dest, Pulse::Low));
        }
        // if round % 100 == 0 {
        //     println!("Round {}. High: {}. Low: {}", round, high, low);
        // }
        while let Some((from, current, pulse)) = queue.pop_front() {
            if pulse == Pulse::High {
                high += 1;
            } else {
                low += 1;
            }
            match config.get(current) {
                None => continue,
                Some((t, dests)) => {
                    match t {
                        Module::FlipFlop => {
                            /*
                                Flip-flop modules (prefix %) are either on or off; they are initially off.
                                If a flip-flop module receives a high pulse, it is ignored and nothing happens.
                                However, if a flip-flop module receives a low pulse, it flips between on and off.
                                If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.
                            */
                            let state = flip_flops.get_mut(current).unwrap();
                            if let Pulse::Low = pulse {
                                *state = match state {
                                    State::On => State::Off,
                                    State::Off => State::On,
                                };
                                if *state == State::On {
                                    for dest in dests {
                                        queue.push_back((current, dest, Pulse::High));
                                    }
                                } else {
                                    for dest in dests {
                                        queue.push_back((current, dest, Pulse::Low));
                                    }
                                }
                            }
                        }
                        Module::Conjunction => {
                            /*
                                Conjunction modules (prefix &) remember the type of the most recent pulse received from each of their connected input modules;
                                they initially default to remembering a low pulse for each input.
                                When a pulse is received, the conjunction module first updates its memory for that input.
                                Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
                            */
                            let inputs = conjunctions.get_mut(current).unwrap();
        
                            // update memory
                            *inputs.get_mut(&from).unwrap_or_else(|| panic!("tried to get input for {} from {}", current, from)) = pulse;
        
                            // check if all inputs are high
                            if inputs.values().all(|p| *p == Pulse::High) {
                                for dest in dests {
                                    queue.push_back((current, dest, Pulse::Low));
                                }
                            } else {
                                for dest in dests {
                                    queue.push_back((current, dest, Pulse::High));
                                }
                            }
                        }
                        _ => unreachable!("Unknown module type"),
                    }
                }
            }
        }
    }

    low * high
}

fn p2(_config: &HashMap<&str, (Module, Vec<&str>)>) -> Solution {
    // determine when rx gets sent a low pulse
    // rx gets sent a low pulse when mg remembers all high pulses
    // mg remembers all high pulses when mg receives a low pulse from each of its inputs

    // the inputs to mg in turn are conjunctions, and they need to send a low pulse each
    // conjunctions send a low pulse when they receive a high pulse from each of their inputs

    // build a network of nodes and calculate the cycle of each.
    // using this we should be able to determine when the cycle of mg is complete

    Solution::Usize(0)
}

fn parse_input(input: &str) -> HashMap<&str, (Module, Vec<&str>)> {
    let mut config = HashMap::new();

    for line in input.lines() {
        let (from, to) = line.split_once(" -> ").unwrap();

        let dests = to.split(", ").collect::<Vec<_>>();
        if from == "broadcaster" {
            config.insert(from, (Module::Broadcaster, dests));
        } else {
            let (t, name) = from.split_at(1);
            let t = match t {
                "%" => Module::FlipFlop,
                "&" => Module::Conjunction,
                _ => panic!("Unknown module type"),
            };
            config.insert(name, (t, dests));
        }
    }

    config
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day20/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(11687500));
        assert_eq!(p2, Solution::Usize(0));
    }
}
