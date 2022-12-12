use itertools::Itertools;
use pathfinding::prelude::bfs;

pub fn parse_input(input: &str) -> (Vec<Vec<u32>>, (usize, usize), (usize, usize)) {
    let starting = input.find('S').unwrap();
    let ending = input.find('E').unwrap();

    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let starting_pos = map
        .iter()
        .enumerate()
        .find_map(|(line_pos, line)| {
            line.iter()
                .position(|c| *c == 'S')
                .map(|col_pos| (line_pos, col_pos))
        })
        .unwrap();

    let ending_pos = map
        .iter()
        .enumerate()
        .find_map(|(line_pos, line)| {
            line.iter()
                .position(|c| *c == 'E')
                .map(|col_pos| (line_pos, col_pos))
        })
        .unwrap();

    let map = map
        .iter()
        .map(|line| {
            line.iter()
                .map(|&x| {
                    if x == 'S' {
                        0
                    } else if x == 'E' {
                        25
                    } else {
                        x as u32 - 97
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    (map, starting_pos, ending_pos)
}

pub fn search(
    map: &Vec<Vec<u32>>,
    starting: (usize, usize),
    ending: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    bfs(
        &starting,
        |p| {
            let mut next = vec![];

            if p.0 > 0 && map[p.0 - 1][p.1].checked_sub(map[p.0][p.1]).unwrap_or(0) <= 1 {
                next.push((p.0 - 1, p.1));
            }

            if p.0 < map.len() - 1 && map[p.0 + 1][p.1].checked_sub(map[p.0][p.1]).unwrap_or(0) <= 1
            {
                next.push((p.0 + 1, p.1));
            }

            if p.1 > 0 && map[p.0][p.1 - 1].checked_sub(map[p.0][p.1]).unwrap_or(0) <= 1 {
                next.push((p.0, p.1 - 1));
            }

            if p.1 < map[0].len() - 1
                && map[p.0][p.1 + 1].checked_sub(map[p.0][p.1]).unwrap_or(0) <= 1
            {
                next.push((p.0, p.1 + 1));
            }

            next
        },
        |p| *p == ending,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, starting, ending) = parse_input(input);
    let search = search(&map, starting, ending).unwrap();

    Some(search.len() as u32 - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, _, ending) = parse_input(input);

    let candidates = map
        .iter()
        .enumerate()
        .flat_map(|(line_num, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(col_num, &height)| {
                    if height == 0 {
                        Some((line_num, col_num))
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let mut paths = candidates
        .iter()
        .filter_map(|&canidate| search(&map, canidate, ending))
        .collect_vec();
    paths.sort_by(|a, b| a.len().cmp(&b.len()));

    Some(paths.first().unwrap().len() as u32 - 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
