use std::{ops::Index, cell::RefCell};

use enum_as_inner::EnumAsInner;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumAsInner)]
pub enum Entry {
    File(u32, String),
    Directory(Vec<Entry>, String),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumAsInner)]
pub enum Command {
    List(Vec<Entry>),
    Change(String),
}

pub fn parse_input(input: &str)  -> Vec<Command> {
    let lines = input.lines().collect::<Vec<&str>>();
    let grouped = lines.iter().enumerate().collect::<Vec<(usize, &&str)>>()
        .split_inclusive(|(index, _)| {
            if *index == lines.len() - 1 {
                return true;
            }

            let next_is_command = lines[index + 1].starts_with("$");

            next_is_command
        })
        .map(|group| {
            group.iter().map(|(_, &line)| line.to_string()).collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    grouped.iter().map(|command| {
        match command[0].as_str() {
            "$ ls" => {
                let entries = command[1..].iter().map(|entry| {
                    if entry.starts_with("dir") {
                        return Entry::Directory(
                            Vec::new(),
                            entry.strip_prefix("dir ").unwrap().to_string(),
                        );
                    }

                    let file_entry = entry.split(" ").collect::<Vec<&str>>();

                    Entry::File(
                        file_entry[0].parse().unwrap(),
                        file_entry[1].to_string()
                    )
                }).collect::<Vec<Entry>>();

                Command::List(entries)
            },
            _ if command[0].starts_with("$ cd") => {
                Command::Change(command[0].strip_prefix("$ cd ").unwrap().to_string())
            }
            _ => todo!(),
        }
    }).collect::<Vec<Command>>()
}

fn get_nested_entry<'a>(entry: &'a mut Entry, stack: &'a [String]) -> &'a mut Entry {
    if stack.len() == 0 {
        return entry;
    }

    let (entries, _) = entry.as_directory_mut().unwrap();

    for entry in entries {
        match entry {
            Entry::Directory(_, name) if name == stack.first().unwrap() => {
                let rest = &stack[1..];
                return get_nested_entry(entry, rest);
            }
            _ => {}
        }
    }

    unreachable!()
}

fn build_filesystem(commands: &Vec<Command>) -> Entry {
    let mut fs = Entry::Directory(Vec::new(), "/".to_string());
    let mut stack: Vec<String> = vec![];

    for command in commands {
        match &command {
            Command::Change(dir) => {
                if dir == ".." {
                    stack.pop();
                } else if dir == "/" {
                    stack = vec![];
                } else {
                    stack.push(dir.clone());
                }
            },
            Command::List(entries) => {
                let (current_entries, _) = get_nested_entry(&mut fs, stack.as_slice()).as_directory_mut().unwrap();

                for entry in entries {
                    match entry {
                        Entry::File(size, name) => {
                            current_entries.push(Entry::File(*size, name.clone()));
                        },
                        Entry::Directory(_, name) => {
                            current_entries.push(Entry::Directory(vec![], name.clone()));
                        }
                    }
                }
            }
        }
    }

    fs
}

fn total_under(node: &Entry, under: u32, results: &mut Vec<u32>) -> u32 {
    let (entries, _) = node.as_directory().unwrap();
    let mut local_size = 0;

    for entry in entries {
        match entry {
            Entry::File(size, _) => {
                local_size += size;
            },
            Entry::Directory(_, _) => {
                local_size += total_under(entry, under, results);
            }
        }
    }

    if local_size < under {
        results.push(local_size)
    }

    local_size
}

fn total(node: &Entry, results: &mut Vec<u32>) -> u32 {
    let (entries, _) = node.as_directory().unwrap();
    let mut local_size = 0;

    for entry in entries {
        match entry {
            Entry::File(size, _) => {
                local_size += size;
            },
            Entry::Directory(_, _) => {
                local_size += total(entry, results);
            }
        }
    }

    results.push(local_size);

    local_size
}


pub fn part_one(input: &str) -> Option<u32> {
    let commands = parse_input(input);
    let filesystem = build_filesystem(&commands);

    let mut results = vec![];
    total_under(&filesystem, 100000, &mut results);


    Some(results.iter().sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let commands = parse_input(input);
    let filesystem = build_filesystem(&commands);


    let mut results = vec![];
    let root_size = total(&filesystem, &mut results);

    let current_free = 70000000 - root_size;
    let need_to_free = 30000000 - current_free;

    results.sort();

    Some(*results.iter().find(|&&size| size >= need_to_free).unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
