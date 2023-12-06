use std::ops::Range;

use crate::SolutionPair;

type RangeMap = (Range<usize>, Range<usize>);

struct Seeds(Vec<usize>);

#[derive(Debug)]
struct Maps {
    seed_to_soil: Vec<RangeMap>,
    soil_to_fertilizer: Vec<RangeMap>,
    fertilizer_to_water: Vec<RangeMap>,
    water_to_light: Vec<RangeMap>,
    light_to_temperature: Vec<RangeMap>,
    temperature_to_humidity: Vec<RangeMap>,
    humidity_to_location: Vec<RangeMap>,
}

impl Maps {
    fn sort(&mut self) {
        self.seed_to_soil.sort_by(|a, b| a.0.start.cmp(&b.0.start));
        self.soil_to_fertilizer
            .sort_by(|a, b| a.0.start.cmp(&b.0.start));
        self.fertilizer_to_water
            .sort_by(|a, b| a.0.start.cmp(&b.0.start));
        self.water_to_light
            .sort_by(|a, b| a.0.start.cmp(&b.0.start));
        self.light_to_temperature
            .sort_by(|a, b| a.0.start.cmp(&b.0.start));
        self.temperature_to_humidity
            .sort_by(|a, b| a.0.start.cmp(&b.0.start));
    }
}

pub fn solve(input: &str) -> SolutionPair {
    (p1::solve(input), p2::solve(input))
}

fn parse_maps(input: &str) -> Maps {
    let mut maps = Maps {
        seed_to_soil: vec![],
        soil_to_fertilizer: vec![],
        fertilizer_to_water: vec![],
        water_to_light: vec![],
        light_to_temperature: vec![],
        temperature_to_humidity: vec![],
        humidity_to_location: vec![],
    };
    let input = input.replace("\r\n", "\n");

    input.split("\n\n").skip(1).for_each(|part| {
        let (map, map_ranges) = part.split_once(" map:\n").unwrap();
        let vec = match map {
            "seed-to-soil" => &mut maps.seed_to_soil,
            "soil-to-fertilizer" => &mut maps.soil_to_fertilizer,
            "fertilizer-to-water" => &mut maps.fertilizer_to_water,
            "water-to-light" => &mut maps.water_to_light,
            "light-to-temperature" => &mut maps.light_to_temperature,
            "temperature-to-humidity" => &mut maps.temperature_to_humidity,
            "humidity-to-location" => &mut maps.humidity_to_location,
            _ => unreachable!(),
        };

        for line in map_ranges.split("\n") {
            let mut container = [0; 3];
            line.split_whitespace()
                .map(str::parse::<usize>)
                .filter_map(Result::ok)
                .enumerate()
                .for_each(|(i, v)| container[i] = v);

            let [destination, source, length] = container;
            let src_range = source..source + length;
            let dst_range = destination..destination + length;
            let range = (src_range, dst_range);
            vec.push(range);
        }
    });

    maps.sort();
    maps
}

fn map_seed_to_location(seed: usize, maps: &Maps) -> usize {
    fn map_value(value: usize, mappings: &Vec<RangeMap>) -> usize {
        for (src, dst) in mappings {
            if src.contains(&value) {
                return dst.start + (value - src.start);
            }
        }
        value
    }
    let soil = map_value(seed, &maps.seed_to_soil);
    let fertilizer = map_value(soil, &maps.soil_to_fertilizer);
    let water = map_value(fertilizer, &maps.fertilizer_to_water);
    let light = map_value(water, &maps.water_to_light);
    let temperature = map_value(light, &maps.light_to_temperature);
    let humidity = map_value(temperature, &maps.temperature_to_humidity);
    let location = map_value(humidity, &maps.humidity_to_location);
    location
}

fn merge_ranges(ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
    let mut ranges = ranges;
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut merged = vec![];
    let mut current = ranges[0].clone();

    for range in ranges.into_iter().skip(1) {
        if current.end >= range.start {
            current.end = range.end;
        } else {
            merged.push(current);
            current = range;
        }
    }

    merged.push(current);
    merged
}

mod p1 {
    use std::time::Instant;

    use crate::etc::Solution;

    use super::*;

    fn parse_seeds(input: &str) -> Seeds {
        let first_row = input.lines().next().unwrap();

        let (_, seeds) = first_row.split_once(": ").unwrap();
        let seeds = seeds
            .split(" ")
            .map(str::parse::<usize>)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        Seeds(seeds)
    }

    pub(super) fn solve(input: &str) -> Solution {
        let start = Instant::now();

        let Seeds(seeds) = parse_seeds(input);
        let maps = parse_maps(input);
        let min = seeds
            .iter()
            .map(|seed| map_seed_to_location(*seed, &maps))
            .min()
            .unwrap();

        let elapsed_ms = start.elapsed().as_nanos() as f64 / 1_000_000.0;
        println!("  · Elapsed: {:.4} ms", elapsed_ms);

        Solution::USize(min)
    }
}

mod p2 {

    use std::time::Instant;

    use super::*;
    use crate::etc::Solution;

    fn parse_seed_ranges(input: &str) -> Vec<Range<usize>> {
        let first_row = input.lines().next().unwrap();

        let (_, seeds) = first_row.split_once(": ").unwrap();
        let seeds = seeds
            .split(" ")
            .map(str::parse::<usize>)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        let seed_ranges = seeds
            .chunks(2)
            .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
            .collect::<Vec<_>>();

        merge_ranges(seed_ranges)
    }

    pub(super) fn solve(input: &str) -> Solution {
        let start = Instant::now();
        let seed_ranges = parse_seed_ranges(input);
        let maps = parse_maps(input);

        let answer = seed_ranges
            .into_iter()
            .map(|r| {
                (r.start..r.end)
                    .into_iter()
                    .map(|seed| map_seed_to_location(seed, &maps))
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap();

        let elapsed_ms = start.elapsed().as_nanos() as f64 / 1_000_000.0;
        println!("  · Elapsed: {:.4} ms", elapsed_ms);
        Solution::USize(answer)
    }
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day05/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::USize(35));
        assert_eq!(p2, Solution::USize(46));
    }
}
