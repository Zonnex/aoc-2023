use regex::Regex;

use crate::{Solution, SolutionPair};

#[derive(Debug, PartialEq, Default)]
struct Game {
    id: usize,
    red: usize,
    green: usize,
    blue: usize,
}
impl Game {
    fn new(id: usize) -> Game {
        Game {
            id: id + 1,
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let re = Regex::new(r"(\d+)\s+(\w+)").unwrap();
    let games = parse_games(input, re);

    (Solution::USize(p1(&games)), Solution::USize(p2(&games)))
}

fn parse_games(input: &str, re: Regex) -> Vec<Game> {
    fn parse_game(id: usize, line: &str, re: &Regex) -> Game {
        let mut game = Game::new(id);

        for cap in re.captures_iter(line) {
            let count: usize = cap[1].parse().unwrap();
            let color: String = cap[2].to_string();

            match color.as_str() {
                "red" => game.red = game.red.max(count),
                "blue" => game.blue = game.blue.max(count),
                "green" => game.green = game.green.max(count),
                _ => unreachable!("Invalid color: {}", color),
            };
        }

        game
    }

    input
        .lines()
        .enumerate()
        .map(|(i, line)| parse_game(i, line, &re))
        .collect::<Vec<_>>()
}

fn p1(games: &[Game]) -> usize {
    games
        .iter()
        .filter(|game| game.red <= 12 && game.green <= 13 && game.blue <= 14)
        .map(|g| g.id)
        .sum()
}

fn p2(games: &[Game]) -> usize {
    games.iter().map(Game::power).sum()
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day02/test.txt");
        let (p1, p2) = super::solve(input);

        assert_eq!(p1, Solution::USize(8));
        assert_eq!(p2, Solution::USize(2286));
    }
}
