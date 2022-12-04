pub fn parse_input(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let arr = line
                .split(',')
                .map(|range| {
                    let range = range.split('-').collect::<Vec<&str>>();
                    let begin: u32 = range[0].parse().unwrap();
                    let end: u32 = range[1].parse().unwrap();

                    (begin..=end).collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>();

            (arr[0].clone(), arr[1].clone())
        })
        .collect()
}

fn find_overlapping_pairs(input: &Vec<(Vec<u32>, Vec<u32>)>) -> Vec<&(Vec<u32>, Vec<u32>)> {
    input
        .iter()
        .filter(|pair| {
            pair.0.iter().all(|x| pair.1.contains(x)) || pair.1.iter().all(|x| pair.0.contains(x))
        })
        .collect()
}

fn find_overlapping_any(input: &Vec<(Vec<u32>, Vec<u32>)>) -> Vec<&(Vec<u32>, Vec<u32>)> {
    input
        .iter()
        .filter(|pair| {
            pair.0.iter().any(|x| pair.1.contains(x)) || pair.1.iter().any(|x| pair.0.contains(x))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed = parse_input(input);
    let overlapping = find_overlapping_pairs(&parsed);

    Some(overlapping.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed = parse_input(input);
    let overlapping = find_overlapping_any(&parsed);

    Some(overlapping.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
