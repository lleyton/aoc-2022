use std::cell::RefCell;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        match s {
            "+" => Operation::Add,
            "-" => Operation::Sub,
            "*" => Operation::Mul,
            "/" => Operation::Div,
            _ => panic!("Unknown operation: {}", s),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Operand {
    Literal(u128),
    Old,
}

impl From<&str> for Operand {
    fn from(s: &str) -> Self {
        match s {
            "old" => Operand::Old,
            _ => Operand::Literal(s.parse().unwrap()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Test {
    Divisible(u128),
}

impl From<&str> for Test {
    fn from(s: &str) -> Self {
        match s {
            _ if s.starts_with("divisible by ") => {
                Test::Divisible(s.strip_prefix("divisible by ").unwrap().parse().unwrap())
            }
            _ => panic!("Unknown test: {}", s),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Action {
    Throw(usize),
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        match s {
            _ if s.starts_with("throw to monkey ") => {
                Action::Throw(s.strip_prefix("throw to monkey ").unwrap().parse().unwrap())
            }
            _ => panic!("Unknown action: {}", s),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Monkey {
    // We don't really need this num, but it's passed in the input... so why not
    number: usize,
    starting_items: Vec<u128>,
    operation: (Operation, Operand, Operand),
    test: Test,
    if_true: Action,
    if_false: Action,
    // How many times has this monkey inspected an item
    inspect_count: u128,
}

pub fn parse_input(input: &str) -> Vec<Monkey> {
    let monkey_blocks = input.split("\n\n").collect_vec();
    monkey_blocks
        .iter()
        .map(|block| {
            let trimmed = block.lines().map(|line| line.trim()).collect_vec();
            let number: usize = trimmed[0]
                .strip_prefix("Monkey ")
                .unwrap()
                .strip_suffix(":")
                .unwrap()
                .parse()
                .unwrap();

            let starting_items = trimmed[1]
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(", ")
                .map(|num| num.parse::<u128>().unwrap())
                .collect_vec();

            let operation = {
                let stripped = trimmed[2]
                    .strip_prefix("Operation: new = ")
                    .unwrap()
                    .split(" ")
                    .collect_vec();

                (
                    Operation::from(stripped[1]),
                    Operand::from(stripped[0]),
                    Operand::from(stripped[2]),
                )
            };

            let test = {
                let stripped = trimmed[3].strip_prefix("Test: ").unwrap();

                Test::from(stripped)
            };

            let if_true = {
                let stripped = trimmed[4].strip_prefix("If true: ").unwrap();

                Action::from(stripped)
            };

            let if_false = {
                let stripped = trimmed[5].strip_prefix("If false: ").unwrap();

                Action::from(stripped)
            };

            Monkey {
                number,
                test,
                if_false,
                if_true,
                operation,
                starting_items,
                inspect_count: 0,
            }
        })
        .collect_vec()
}

pub fn perform_operation(operation: (Operation, Operand, Operand), old: u128) -> u128 {
    let operand_1 = match operation.1 {
        Operand::Literal(num) => num,
        Operand::Old => old,
    };

    let operand_2 = match operation.2 {
        Operand::Literal(num) => num,
        Operand::Old => old,
    };

    operand_1.checked_mul(operand_2).unwrap_or(old);

    match operation.0 {
        Operation::Add => operand_1 + operand_2,
        Operation::Sub => operand_1 - operand_2,
        Operation::Mul => operand_1 * operand_2,
        Operation::Div => operand_1 / operand_2,
    }
}

pub fn perform_test(test: Test, worry: u128) -> bool {
    match test {
        Test::Divisible(divisor) => worry % divisor == 0,
    }
}

pub fn run_round(monkeys: &Vec<Monkey>) -> Vec<Monkey> {
    // STFU Rust's borrow checker
    let new_monkeys = monkeys
        .iter()
        .map(|monkey| RefCell::new(monkey.clone()))
        .collect_vec();

    new_monkeys.iter().for_each(|monkey| {
        let mut monkey = monkey.borrow_mut();
        monkey.starting_items.clone().iter().for_each(|item| {
            monkey.inspect_count += 1;
            // We divide by 3... because the monkey gets bored quick.
            let new_worry = perform_operation(monkey.operation, *item) / 3;
            let test_result = perform_test(monkey.test, new_worry);

            let action = if test_result {
                monkey.if_true
            } else {
                monkey.if_false
            };

            match action {
                Action::Throw(next_monkey_index) => {
                    let item_index = monkey
                        .starting_items
                        .iter()
                        .position(|i| i == item)
                        .unwrap();
                    monkey.starting_items.remove(item_index);

                    new_monkeys[next_monkey_index]
                        .borrow_mut()
                        .starting_items
                        .push(new_worry);
                }
            }
        });
    });

    // Make Rust happy again
    new_monkeys
        .iter()
        .map(|monkey| monkey.clone().into_inner())
        .collect_vec()
}

pub fn run_round_2(monkeys: &Vec<Monkey>) -> Vec<Monkey> {
    let wrap = monkeys
        .iter()
        .map(|m| match m.test {
            Test::Divisible(divisor) => divisor,
        })
        .reduce(|a, b| a * b)
        .unwrap();

    // STFU Rust's borrow checker
    let new_monkeys = monkeys
        .iter()
        .map(|monkey| RefCell::new(monkey.clone()))
        .collect_vec();

    new_monkeys.iter().for_each(|monkey| {
        let mut monkey = monkey.borrow_mut();
        monkey.starting_items.clone().iter().for_each(|item| {
            monkey.inspect_count += 1;
            // Wrap the result around
            let new_worry = perform_operation(monkey.operation, *item) % wrap;
            let test_result = perform_test(monkey.test, new_worry);

            let action = if test_result {
                monkey.if_true
            } else {
                monkey.if_false
            };

            match action {
                Action::Throw(next_monkey_index) => {
                    let item_index = monkey
                        .starting_items
                        .iter()
                        .position(|i| i == item)
                        .unwrap();
                    monkey.starting_items.remove(item_index);

                    new_monkeys[next_monkey_index]
                        .borrow_mut()
                        .starting_items
                        .push(new_worry);
                }
            }
        });
    });

    // Make Rust happy again
    new_monkeys
        .iter()
        .map(|monkey| monkey.clone().into_inner())
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u128> {
    let monkeys = parse_input(input);
    let mut final_state = (0..20).fold(monkeys, |state, _| run_round(&state));
    final_state.sort_by(|a, b| a.inspect_count.cmp(&b.inspect_count));

    Some(
        final_state.last().unwrap().inspect_count
            * final_state[final_state.len() - 2].inspect_count,
    )
}

pub fn part_two(input: &str) -> Option<u128> {
    let monkeys = parse_input(input);
    println!("{:#?}", run_round_2(&monkeys));
    let mut final_state = (0..10000).fold(monkeys, |state, _| run_round_2(&state));
    final_state.sort_by(|a, b| a.inspect_count.cmp(&b.inspect_count));

    Some(
        final_state.last().unwrap().inspect_count
            * final_state[final_state.len() - 2].inspect_count,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
