use std::collections::LinkedList;

type State = Vec<LinkedList<char>>;
type Instructions = Vec<(u32, u32, u32)>;

pub fn parse_input(input: &str) -> (State, Instructions) {
    let lines: Vec<&str> = input.lines().collect();
    let sections = lines.split(|&line| line == "").collect::<Vec<&[&str]>>();

    let initial_section = sections[0];
    let instructions_section = sections[1];

    let mut initial_rows = initial_section
        .iter()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|(i, _)| (i + 1) % 4 != 0)
                .map(|(_, c)| c)
                .collect::<Vec<char>>()
                .chunks(3)
                .map(|chunk| {
                    if chunk[1] == ' ' {
                        None
                    } else {
                        Some(chunk[1])
                    }
                })
                .collect::<Vec<Option<char>>>()
        })
        .collect::<Vec<Vec<Option<char>>>>();

    initial_rows.reverse();

    let column_count = initial_rows[0].len();
    let mut state: Vec<LinkedList<char>> = (0..column_count).map(|_| LinkedList::new()).collect();

    initial_rows.iter().skip(1).for_each(|row| {
        row.iter()
            .enumerate()
            .filter(|(_, item)| item.is_some())
            .for_each(|(i, item)| {
                state[i].push_front(item.unwrap());
            });
    });

    let instructions = instructions_section
        .iter()
        .map(|line| {
            line.split(' ')
                .map(|chr| chr.parse::<u32>())
                .filter(|result| result.is_ok())
                .map(|result| result.unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|arr| (arr[0], arr[1], arr[2]))
        .collect::<Vec<(u32, u32, u32)>>();

    (state, instructions)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut state, instructions) = parse_input(input);

    instructions.iter().for_each(|(amount, from, to)| {
        for _ in 0..*amount {
            let item = state[(from - 1) as usize].pop_front().unwrap();
            state[(to - 1) as usize].push_front(item);
        }
    });

    let chars: String = state.iter().map(|list| *list.front().unwrap()).collect();

    Some(chars)
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut state, instructions) = parse_input(input);

    instructions.iter().for_each(|(amount, from, to)| {
        let mut pulled = (0..*amount)
            .map(|_| state[(from - 1) as usize].pop_front().unwrap())
            .collect::<Vec<char>>();
        pulled.reverse();

        for i in 0..*amount {
            state[(to - 1) as usize].push_front(pulled[i as usize]);
        }
    });

    let chars: String = state.iter().map(|list| *list.front().unwrap()).collect();

    Some(chars)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_owned()));
    }
}
