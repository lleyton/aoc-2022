use itertools::Itertools;

pub fn parse_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed = parse_input(input);

    for (i, _) in parsed.iter().enumerate() {
        if i < 4 {
            continue;
        }

        let sequence = &parsed[(i - 4)..i];
        let unique = sequence.iter().unique().collect::<Vec<&char>>().len();

        if unique == 4 {
            return Some(i as u32);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed = parse_input(input);

    for (i, _) in parsed.iter().enumerate() {
        if i < 14 {
            continue;
        }

        let sequence = &parsed[(i - 14)..i];
        let unique = sequence.iter().unique().collect::<Vec<&char>>().len();

        if unique == 14 {
            return Some(i as u32);
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
