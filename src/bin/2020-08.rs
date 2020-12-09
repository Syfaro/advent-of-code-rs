use std::collections::HashSet;
use std::convert::TryFrom;

static PROBLEM_NAME: &str = "2020-08";

/// An instruction to execute.
#[derive(Copy, Clone, Debug, PartialEq)]
enum Instruction {
    Nop,
    Acc,
    Jmp,
}

impl std::str::FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(Instruction::Nop),
            "acc" => Ok(Instruction::Acc),
            "jmp" => Ok(Instruction::Jmp),
            _ => Err("unknown instruction"),
        }
    }
}

/// An instruction line with a single parameter.
#[derive(Clone, Debug, PartialEq)]
struct Line {
    instruction: Instruction,
    parameter: i32,
}

impl std::str::FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction: Instruction = s[..3].parse()?;
        let parameter: i32 = s[4..].parse().map_err(|_| "invalid parameter")?;

        Ok(Self {
            instruction,
            parameter,
        })
    }
}

fn main() {
    advent_of_code::init();

    let input = advent_of_code::load_input(PROBLEM_NAME);
    let lines: Vec<Line> = advent_of_code::utils::decode_line(&input);

    let acc = run_until_duplicate(&lines);
    log::info!("Part 1 = {}", acc);

    let acc = flip_until_complete(&lines);
    log::info!("Part 2 = {}", acc);
}

/// Execute instructions until revisiting a line, keeping track of and returning
/// the accumulator when done.
fn run_until_duplicate(lines: &[Line]) -> i32 {
    let mut pos = 0;
    let mut acc = 0;

    let mut visited = HashSet::with_capacity(lines.len());

    // Check if already visited this instruction line. If we have, we're done.
    // Otherwise, evaluate instruction and move to new position.
    loop {
        if visited.contains(&pos) {
            log::debug!("Already visited line {}, exiting", pos);
            break acc;
        }
        visited.insert(pos);

        let line = lines.get(pos).unwrap();
        log::trace!("Evaluating line {}: {:?}", pos, line);

        match line.instruction {
            Instruction::Nop => pos += 1,
            Instruction::Acc => {
                acc += line.parameter;
                pos += 1;
            }
            Instruction::Jmp => pos = usize::try_from(pos as i32 + line.parameter).unwrap(),
        }
        log::trace!("Moving to line {}", pos);
    }
}

/// Attempt to execute instructions until everything has been completed. Keeps
/// track of the accumulator value which is returned if the run completed,
/// otherwise returns 0.
fn will_complete(lines: &[Line]) -> (bool, i32) {
    let mut pos = 0;
    let mut acc = 0;

    let mut visited = HashSet::with_capacity(lines.len());

    // Nearly identical to `run_until_duplicate`, except handles checking if
    // we have completed executing all lines.
    loop {
        if pos >= lines.len() {
            log::debug!("Reached end of instructions");
            break (true, acc);
        }

        if visited.contains(&pos) {
            log::debug!("Already visited line {}, exiting", pos);
            return (false, 0);
        }
        visited.insert(pos);

        let line = lines.get(pos).unwrap();
        log::trace!("Evaluating line {}: {:?}", pos, line);

        match line.instruction {
            Instruction::Nop => pos += 1,
            Instruction::Acc => {
                acc += line.parameter;
                pos += 1;
            }
            Instruction::Jmp => pos = usize::try_from(pos as i32 + line.parameter).unwrap(),
        }
        log::trace!("Moving to line {}", pos);
    }
}

/// Flips every nop and jmp instruction until the program successfully exits.
fn flip_until_complete(lines: &[Line]) -> i32 {
    for index in 0..lines.len() {
        let mut lines = lines.to_vec();

        match lines[index].instruction {
            Instruction::Nop => lines[index].instruction = Instruction::Jmp,
            Instruction::Jmp => lines[index].instruction = Instruction::Nop,
            // If we're not flipping the instruction, we don't need to try it.
            _ => continue,
        }

        log::trace!("Trying to flip instruction on line {}", index);

        let (completed, acc) = will_complete(&lines);
        if completed {
            log::debug!(
                "Flipping instruction on line {} to {:?} completed",
                index,
                lines[index].instruction
            );
            return acc;
        }
    }

    panic!("found no solutions");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_from_str() {
        advent_of_code::init();

        let instr: Instruction = "jmp".parse().unwrap();
        assert_eq!(instr, Instruction::Jmp);
    }

    #[test]
    fn test_line_from_str() {
        advent_of_code::init();

        let line: Line = "acc +3".parse().unwrap();
        assert_eq!(line.instruction, Instruction::Acc);
        assert_eq!(line.parameter, 3);
    }

    #[test]
    fn test_run_until_duplicate() {
        advent_of_code::init();

        let input = "nop 0\njmp -1";
        let lines: Vec<Line> = advent_of_code::utils::decode_line(&input);

        let acc = run_until_duplicate(&lines);
        assert_eq!(acc, 0);
    }

    #[test]
    fn test_will_complete() {
        advent_of_code::init();

        let input = "nop 0";
        let lines: Vec<Line> = advent_of_code::utils::decode_line(&input);

        let (completed, acc) = will_complete(&lines);
        assert_eq!(completed, true);
        assert_eq!(acc, 0);
    }

    #[test]
    fn test_flip_until_complete() {
        advent_of_code::init();

        let input = "nop 0\njmp -1";
        let lines: Vec<Line> = advent_of_code::utils::decode_line(&input);

        let acc = flip_until_complete(&lines);
        assert_eq!(acc, 0);
    }
}
