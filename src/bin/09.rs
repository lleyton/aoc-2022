use std::iter::repeat;

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => unimplemented!("Invalid move"),
        }
    }
}

type Move = (Direction, usize);

pub fn parse_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let split = line.split(" ").collect::<Vec<&str>>();
            (
                Direction::from(split[0].chars().next().unwrap()),
                split[1].parse::<usize>().unwrap(),
            )
        })
        .collect()
}

pub fn get_direction_list(moves: &Vec<Move>) -> Vec<Direction> {
    moves
        .iter()
        .flat_map(|(direction, amount)| repeat(direction).take(*amount).map(|_| *direction))
        .collect()
}

type Coordinate = (i32, i32);

#[derive(Debug, Clone, Copy)]
pub struct RopeState {
    pub head: Coordinate,
    pub tail: Coordinate,
}

impl Default for RopeState {
    fn default() -> Self {
        RopeState {
            head: (0, 0),
            tail: (0, 0),
        }
    }
}

fn is_adjacent(first: &Coordinate, second: &Coordinate) -> bool {
    if first == second {
        return true;
    }

    let (x1, y1) = first;
    let (x2, y2) = second;

    if (x1 - x2).abs() <= 1 && (y1 - y2).abs() <= 1 {
        return true;
    } else {
        false
    }
}

fn get_next_state(state: &RopeState, direction: Direction) -> RopeState {
    let (x, y) = state.head;
    let new_head = match direction {
        Direction::Down => (x, y - 1),
        Direction::Up => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    };

    let adjacent = is_adjacent(&new_head, &state.tail);
    let diff = (new_head.0 - state.tail.0, new_head.1 - state.tail.1);

    if adjacent {
        RopeState {
            head: new_head,
            tail: state.tail,
        }
    } else {
        RopeState {
            head: new_head,
            tail: (
                state.tail.0 + diff.0.signum(),
                state.tail.1 + diff.1.signum(),
            ),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let moves = parse_input(input);
    let directions = get_direction_list(&moves);
    let initial = RopeState::default();

    let mut states = directions
        .iter()
        .scan(initial, |acc, direction| {
            let next = get_next_state(&acc, *direction);
            *acc = next.clone();
            Some(next)
        })
        .collect::<Vec<RopeState>>();
    states.insert(0, initial);

    Some(states.iter().map(|state| state.tail).unique().count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let moves = parse_input(input);
    let directions = get_direction_list(&moves);
    let initial = repeat(RopeState::default())
        .take(10)
        .collect::<Vec<RopeState>>();

    let mut states = directions
        .iter()
        .scan(initial.clone(), |acc, direction| {
            let first = acc[0];
            let next = get_next_state(&first, *direction);

            let mut updated = acc
                .iter()
                .skip(1)
                .scan(next, |acc2, curr| {
                    let new = RopeState {
                        head: acc2.tail,
                        tail: curr.tail,
                    };

                    let adjacent = is_adjacent(&new.head, &new.tail);

                    if adjacent {
                        *acc2 = new.clone();
                        Some(new)
                    } else {
                        let diff = (new.head.0 - new.tail.0, new.head.1 - new.tail.1);

                        let new = RopeState {
                            head: acc2.tail,
                            tail: (new.tail.0 + diff.0.signum(), new.tail.1 + diff.1.signum()),
                        };

                        *acc2 = new.clone();
                        Some(new)
                    }
                })
                .collect::<Vec<RopeState>>();

            updated.insert(0, next);

            *acc = updated.clone();
            Some(updated)
        })
        .collect::<Vec<Vec<RopeState>>>();
    states.insert(0, initial);

    Some(
        states
            .iter()
            .map(|state| state.iter().last().unwrap().head)
            .unique()
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
