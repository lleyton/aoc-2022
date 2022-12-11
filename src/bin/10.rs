use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let split = line.split(" ").collect_vec();

            match split[0] {
                "noop" => Instruction::Noop,
                "addx" => Instruction::Addx(split[1].parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect_vec()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct VMState {
    pub cycle: usize,
    pub program_counter: usize,
    // How many cycles left before executing the next instruction
    pub pending_cycles: usize,
    pub register_x: i32,
    pub instructions: Vec<Instruction>,
    pub halted: bool,
}

impl VMState {
    pub fn run_cycle(&self) -> VMState {
        let mut new_state = self.clone();
        let current_instruction = new_state.instructions[new_state.program_counter];

        match current_instruction {
            Instruction::Noop => {
                new_state.program_counter += 1;
            }
            Instruction::Addx(operand) if new_state.pending_cycles == 1 => {
                new_state.register_x += operand;
                new_state.program_counter += 1;
                new_state.pending_cycles = 0;
            }
            Instruction::Addx(_) => {
                new_state.pending_cycles = 1;
            }
        }

        new_state.cycle += 1;

        if new_state.program_counter >= new_state.instructions.len() {
            new_state.halted = true;
        }

        new_state
    }

    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            cycle: 0,
            program_counter: 0,
            pending_cycles: 0,
            register_x: 1,
            halted: instructions.len() == 0,
            instructions,
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let instructions = parse_input(input);

    let mut states = Vec::new();
    let mut current_state = VMState::new(instructions);

    states.push(current_state.clone());

    while !current_state.halted {
        current_state = current_state.run_cycle();
        states.push(current_state.clone());
    }

    Some(
        vec![20, 60, 100, 140, 180, 220]
            .iter()
            .map(|cycle| states[cycle - 1].register_x * (*cycle as i32))
            .sum::<i32>(),
    )
}

pub fn draw_frame(states: Vec<VMState>) -> String {
    let frame = (0..6).map(|_| (0..40).collect_vec()).collect_vec();

    let drawed = frame
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(|(col_index, _)| {
                    let current_state = &states[row_index * 40 + col_index];
                    if (col_index + 1) >= current_state.register_x as usize
                        && (col_index + 1) < current_state.register_x as usize + 3
                    {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    drawed
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect_vec()
        .join("\n")
}

pub fn part_two(input: &str) -> Option<String> {
    let instructions = parse_input(input);

    let mut states = Vec::new();
    let mut current_state = VMState::new(instructions);

    states.push(current_state.clone());

    while !current_state.halted {
        current_state = current_state.run_cycle();
        states.push(current_state.clone());
    }

    let frame = draw_frame(states);

    Some(frame)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let out =
"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(out.to_string()));
    }
}
