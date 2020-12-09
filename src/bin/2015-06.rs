use lazy_static::lazy_static;
use regex::Regex;

const PROBLEM_NAME: &str = "2015-06";

lazy_static! {
    static ref COMMAND: Regex = Regex::new(r#"(?P<action>turn on|turn off|toggle) (?P<x0>\d+),(?P<y0>\d+) through (?P<x1>\d+),(?P<y1>\d+)"#).unwrap();
}

/// An action to perform to the lights.
#[derive(Clone, Debug, PartialEq)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

impl std::str::FromStr for Action {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "turn on" => Ok(Action::TurnOn),
            "turn off" => Ok(Action::TurnOff),
            "toggle" => Ok(Action::Toggle),
            _ => Err("unknown action"),
        }
    }
}

/// A command to perform on the lights, including which action to perform and
/// which lights it should be performed on.
#[derive(Clone, Debug, PartialEq)]
struct Command {
    action: Action,
    from: (usize, usize),
    to: (usize, usize),
}

impl std::str::FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = COMMAND.captures(&s).ok_or("line was not valid command")?;

        let action: Action = captures["action"].parse()?;

        let x0: usize = captures["x0"].parse().map_err(|_| "invalid x0")?;
        let y0: usize = captures["y0"].parse().map_err(|_| "invalid y0")?;
        let x1: usize = captures["x1"].parse().map_err(|_| "invalid x1")?;
        let y1: usize = captures["y1"].parse().map_err(|_| "invalid y1")?;

        Ok(Command {
            action,
            from: (x0, y0),
            to: (x1, y1),
        })
    }
}

fn main() {
    advent_of_code::init();

    let input = advent_of_code::load_input(PROBLEM_NAME);
    let commands: Vec<Command> = advent_of_code::utils::decode_line(&input);

    let mut lights: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    commands
        .iter()
        .for_each(|command| apply_command_1(&mut lights, command));
    let on = lights.iter().flatten().filter(|light| **light > 0).count();
    log::info!("Part 1 = {}", on);

    let mut lights: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    commands
        .iter()
        .for_each(|command| apply_command_2(&mut lights, command));
    let total_brightness: usize = lights.iter().flatten().sum();
    log::info!("Part 2 = {}", total_brightness);
}

/// Apply the initial actions for the set of commands where a light can only
/// be in a binary position of on/off or 1/0.
fn apply_command_1(lights: &mut [Vec<usize>], command: &Command) {
    for x in command.from.0..=command.to.0 {
        for y in command.from.1..=command.to.1 {
            let cell = &mut lights[x][y];

            match command.action {
                Action::TurnOn => *cell = 1,
                Action::TurnOff => *cell = 0,
                Action::Toggle => *cell = if *cell == 1 { 0 } else { 1 },
            }
        }
    }
}

/// Apply the additional actions for the set of commands where the total
/// brightness changes based on the command. It has a minimum brightness of 0
/// and toggling it increases the brightness by 2.
fn apply_command_2(lights: &mut [Vec<usize>], command: &Command) {
    for x in command.from.0..=command.to.0 {
        for y in command.from.1..=command.to.1 {
            let cell = &mut lights[x][y];

            match command.action {
                Action::TurnOn => *cell += 1,
                // It's possible to subtract from an already off light, so we
                // need to make sure it doesn't go below zero.
                Action::TurnOff => *cell = cell.saturating_sub(1),
                Action::Toggle => *cell += 2,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_from_str() {
        let input: Action = "turn on".parse().unwrap();
        assert_eq!(input, Action::TurnOn);
    }

    #[test]
    fn test_command_from_str() {
        let input: Command = "toggle 1,2 through 999,998".parse().unwrap();
        assert_eq!(
            input,
            Command {
                action: Action::Toggle,
                from: (1, 2),
                to: (999, 998),
            }
        )
    }

    #[test]
    fn test_apply_command_1() {
        let input: Command = "toggle 1,2 through 3,4".parse().unwrap();
        let mut lights = vec![vec![0; 1000]; 1000];
        apply_command_1(&mut lights, &input);
        assert_eq!(lights[1][2], 1);
    }

    #[test]
    fn test_apply_command_2() {
        let input: Command = "toggle 1,2 through 3,4".parse().unwrap();
        let mut lights = vec![vec![0; 1000]; 1000];
        apply_command_2(&mut lights, &input);
        assert_eq!(lights[1][2], 2);
    }
}
