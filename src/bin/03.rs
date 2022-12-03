use std::{
    collections::HashSet
};

// NOTE: this is terrible code, I got frustrated near the end (thanks rustc!) and just decided that I didn't give a shit about code quality
// You've been warned

pub fn parse_input(input: &str) -> Vec<(Vec<char>, Vec<char>)> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);

            (first.chars().collect(), second.chars().collect())
        })
        .collect()
}

pub fn find_shared(input: &(Vec<char>, Vec<char>)) -> Vec<char> {
    let first: HashSet<&char> = HashSet::from_iter(input.0.iter());
    let second: HashSet<&char> = HashSet::from_iter(input.1.iter());

    first.intersection(&second).map(|&&c| c).collect()
}

pub fn get_priority(input: char) -> u32 {
    let mut alphabet = ('a'..='z').collect::<Vec<char>>();
    alphabet.extend('A'..='Z');

    alphabet.iter().position(|&l| l == input).unwrap() as u32 + 1
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed = parse_input(input);
    Some(
        parsed
            .iter()
            .map(find_shared)
            .map(|sack| sack.iter().map(|c| get_priority(*c)).sum::<u32>())
            .sum(),
    )
}

pub fn parse_input_part_2(input: &str) -> Vec<(Vec<char>, Vec<char>, Vec<char>)> {
    let a = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|c| c.iter().map(|&s| s.chars().collect()).collect())
        .collect::<Vec<Vec<Vec<char>>>>();

    let b = a
        .iter()
        .map(|c| (c[0].clone(), c[1].clone(), c[2].clone()))
        .collect::<Vec<_>>();

    b
}

pub fn find_shared_part_2(input: &(Vec<char>, Vec<char>, Vec<char>)) -> Vec<char> {
    let sets = vec![
        HashSet::from_iter(input.0.iter()),
        HashSet::from_iter(input.1.iter()),
        HashSet::from_iter(input.2.iter()),
    ];

    let new = sets
        .iter()
        .skip(1)
        .fold(sets[0].clone(), |a: HashSet<&char>, b| {
            a.intersection(b).cloned().collect()
        });

    new.iter().map(|&&c| c).collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed = parse_input_part_2(input);
    Some(
        parsed
            .iter()
            .map(find_shared_part_2)
            .map(|s| s.iter().map(|c| get_priority(*c)).sum::<u32>())
            .sum::<u32>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
