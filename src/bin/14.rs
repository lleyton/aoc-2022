use std::collections::HashMap;

use itertools::Itertools;

pub fn parse_input(input: &str) -> Vec<Vec<(i32, i32)>> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| {
                    pair.split(",")
                        .map(|num| num.parse().unwrap())
                        .collect_tuple::<(i32, i32)>()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Item {
    Rock,
    Sand,
}

pub fn build_map(lines: &Vec<Vec<(i32, i32)>>) -> HashMap<(i32, i32), Item> {
    lines
        .iter()
        .flat_map(|line| {
            line.iter()
                .enumerate()
                .skip(1)
                .flat_map(|(i, point)| {
                    let prev = line[i - 1];

                    let right = prev.0 == point.0;
                    let (start, end) = if right && prev.1 < point.1 {
                        (prev.1, point.1)
                    } else if right && prev.1 >= point.1 {
                        (point.1, prev.1)
                    } else if prev.0 < point.0 {
                        (prev.0, point.0)
                    } else {
                        (point.0, prev.0)
                    };

                    let change = start..=end;

                    change
                        .map(|val| if right { (prev.0, val) } else { (val, prev.1) })
                        .collect_vec()
                })
                .map(|point| (point, Item::Rock))
                .collect_vec()
        })
        .collect()
}

// The bool in the return value is if the sand fell into the A B Y S S
fn drop_sand(map: &HashMap<(i32, i32), Item>) -> (HashMap<(i32, i32), Item>, bool) {
    let mut new = map.clone();
    let mut sand = (500, 0);

    while sand.1 < 1000 {
        let below = (sand.0, sand.1 + 1);
        let below_left = (sand.0 - 1, sand.1 + 1);
        let below_right = (sand.0 + 1, sand.1 + 1);

        if !map.contains_key(&below) {
            sand = below;
        } else if !map.contains_key(&below_left) {
            sand = below_left;
        } else if !map.contains_key(&below_right) {
            sand = below_right;
        } else {
            new.insert(sand, Item::Sand);
            return (new, false);
        }
    }

    (new, true)
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = parse_input(input);
    let mut map = build_map(&lines);

    let mut amount = 0;
    loop {
        let (new_map, fell) = drop_sand(&map);
        map = new_map;

        if fell {
            break;
        }

        amount += 1;
    }

    Some(amount)
}

fn drop_sand_2(map: &HashMap<(i32, i32), Item>, floor: i32) -> HashMap<(i32, i32), Item> {
    let mut new = map.clone();
    let mut sand = (500, 0);

    loop {
        let below = (sand.0, sand.1 + 1);
        let below_left = (sand.0 - 1, sand.1 + 1);
        let below_right = (sand.0 + 1, sand.1 + 1);

        if sand.1 == floor - 1 {
            new.insert(sand, Item::Sand);
            return new;
        } else if !map.contains_key(&below) {
            sand = below;
        } else if !map.contains_key(&below_left) {
            sand = below_left;
        } else if !map.contains_key(&below_right) {
            sand = below_right;
        } else {
            new.insert(sand, Item::Sand);
            return new;
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = parse_input(input);
    let mut map = build_map(&lines);

    let mut points = map
        .clone()
        .into_iter()
        .map(|(point, _)| point)
        .collect_vec();
    points.sort_by(|a, b| a.1.cmp(&b.1));

    let floor = points.last().unwrap().1 + 2;

    let mut amount = 0;
    loop {
        map = drop_sand_2(&map, floor);
        amount += 1;

        if map.contains_key(&(500, 0)) {
            break;
        }
    }

    Some(amount)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
