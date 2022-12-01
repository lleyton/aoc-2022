fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|line| line.parse::<u32>().ok())
        .collect::<Vec<Option<u32>>>()
        .split(|num| num.is_none())
        .map(|part| part.iter().map(|num| num.unwrap()).collect())
        .collect()
}

fn get_total_counts(input: Vec<Vec<u32>>) -> Vec<u32> {
    input.iter().map(|part| part.iter().sum::<u32>()).collect::<Vec<u32>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let numbers = parse_input(input);
    let mut totals = get_total_counts(numbers);
    totals.sort();

    totals.last().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = parse_input(input);
    let mut totals = get_total_counts(numbers);
    totals.sort();
    totals.reverse();

    let (top3, _) = totals.split_at(3);

    Some(top3.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
