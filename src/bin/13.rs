use std::{cmp::Ordering, vec};

use itertools::{EitherOrBoth, Itertools};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(untagged)]
pub enum ListItem {
    List(Vec<ListItem>),
    Number(u32),
}

pub fn parse_input(input: &str) -> Vec<(Vec<ListItem>, Vec<ListItem>)> {
    input
        .split("\n\n")
        .map(|pair| {
            let (first, second) = pair.split_terminator("\n").collect_tuple().unwrap();
            (
                serde_json::from_str(first).unwrap(),
                serde_json::from_str(second).unwrap(),
            )
        })
        .collect_vec()
}

pub fn in_right_order(pair: &(Vec<ListItem>, Vec<ListItem>)) -> Option<bool> {
    for val in pair.0.iter().zip_longest(pair.1.clone()) {
        match val {
            EitherOrBoth::Both(ListItem::Number(a), ListItem::Number(b)) => {
                if *a < b {
                    return Some(true);
                }

                if *a > b {
                    return Some(false);
                }
            }
            EitherOrBoth::Both(ListItem::List(a), ListItem::Number(b)) => {
                if let Some(r) = in_right_order(&(a.to_vec(), vec![ListItem::Number(b)])) {
                    return Some(r);
                }
            }
            EitherOrBoth::Both(ListItem::Number(a), ListItem::List(b)) => {
                if let Some(r) = in_right_order(&(vec![ListItem::Number(*a)], b.to_vec())) {
                    return Some(r);
                }
            }
            EitherOrBoth::Both(ListItem::List(a), ListItem::List(b)) => {
                if let Some(r) = in_right_order(&(a.to_vec(), b.to_vec())) {
                    return Some(r);
                }
            }
            EitherOrBoth::Left(_) => return Some(false),
            EitherOrBoth::Right(_) => return Some(true),
        };
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = parse_input(input);
    Some(
        pairs
            .iter()
            .enumerate()
            .filter_map(|(i, pair)| {
                if in_right_order(pair).unwrap() {
                    Some(i as u32 + 1)
                } else {
                    None
                }
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let inputs = parse_input(input);
    let mut inputs = inputs.iter().flat_map(|(a, b)| vec![a, b]).collect_vec();

    let divider1 = vec![ListItem::List(vec![ListItem::Number(2)])];
    let divider2 = vec![ListItem::List(vec![ListItem::Number(6)])];
    inputs.push(&divider1);
    inputs.push(&divider2);

    inputs.sort_by(|a, b| {
        if in_right_order(&(a.to_vec(), b.to_vec())).unwrap() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let divider1_pos = inputs.iter().position(|x| x == &&divider1).unwrap();
    let divider2_pos = inputs.iter().position(|x| x == &&divider2).unwrap();

    Some(((divider1_pos + 1) * (divider2_pos + 1)) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
