#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Choice {
    fn from(chr: char) -> Self {
        match chr {
            'A' | 'X' => Choice::Rock,
            'B' | 'Y' => Choice::Paper,
            'C' | 'Z' => Choice::Scissors,
            _ => panic!("Invalid choice"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum GameResult {
    Win,
    Lose,
    Draw,
}

impl From<char> for GameResult {
    fn from(chr: char) -> Self {
        match chr {
            'X' => GameResult::Lose,
            'Z' => GameResult::Win,
            'Y' => GameResult::Draw,
            _ => panic!("Invalid game result"),
        }
    }
}

pub fn parse_input(input: &str) -> Vec<(Choice, Choice)> {
    input
        .lines()
        .map(|line| {
            let instruction = line.chars().collect::<Vec<char>>();

            (instruction[0].into(), instruction[2].into())
        })
        .collect()
}

pub fn parse_input_by_strategy(input: &str) -> Vec<(Choice, GameResult)> {
    input
        .lines()
        .map(|line| {
            let instruction = line.chars().collect::<Vec<char>>();

            (instruction[0].into(), instruction[2].into())
        })
        .collect()
}

pub fn evalulate_game(game: &(Choice, Choice)) -> u32 {
    let score_by_type = match game.1 {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    let score_by_match = match game {
        (Choice::Scissors, Choice::Rock) => 6,
        (Choice::Rock, Choice::Paper) => 6,
        (Choice::Paper, Choice::Scissors) => 6,
        _ if game.0 == game.1 => 3,
        _ => 0,
    };

    score_by_type + score_by_match
}

pub fn get_game_by_strategy(strategy: &(Choice, GameResult)) -> (Choice, Choice) {
    match strategy {
        (Choice::Paper, GameResult::Win) => (strategy.0, Choice::Scissors),
        (Choice::Rock, GameResult::Win) => (strategy.0, Choice::Paper),
        (Choice::Scissors, GameResult::Win) => (strategy.0, Choice::Rock),
        (Choice::Paper, GameResult::Lose) => (strategy.0, Choice::Rock),
        (Choice::Rock, GameResult::Lose) => (strategy.0, Choice::Scissors),
        (Choice::Scissors, GameResult::Lose) => (strategy.0, Choice::Paper),
        (_, GameResult::Draw) => (strategy.0, strategy.0),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let choices = parse_input(input);

    Some(choices.iter().map(evalulate_game).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let choices = parse_input_by_strategy(input);
    Some(
        choices
            .iter()
            .map(get_game_by_strategy)
            .map(|game| evalulate_game(&game))
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
