#![allow(dead_code)]
use crate::common::error::CommonError;

struct State {
    distance: i64,
    depth: i64,
    aim: i64,
}

impl State {
    fn new() -> Self {
        State {
            distance: 0,
            depth: 0,
            aim: 0,
        }
    }
}

enum Command {
    Forward(i64),
    Up(i64),
    Down(i64),
}

impl std::str::FromStr for Command {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split(' ').collect();
        if splitted.len() != 2 {
            return Err(CommonError::Parse(
                "Malformed input, expected two elements in the line.",
            ));
        }

        if let Ok(num) = splitted[1].parse::<i64>() {
            match splitted[0] {
                "forward" => Ok(Command::Forward(num)),
                "up" => Ok(Command::Up(num)),
                "down" => Ok(Command::Down(num)),
                _ => Err(CommonError::Parse("Unknown command in the line.")),
            }
        } else {
            Err(CommonError::Parse(
                "Second element in line is not a number.",
            ))
        }
    }
}

fn execute_commands_one(commands: &[Command]) -> State {
    let mut state = State::new();

    for command in commands {
        match command {
            Command::Forward(num) => {
                state.distance += num;
            }
            Command::Up(num) => {
                state.depth -= num;
            }
            Command::Down(num) => {
                state.depth += num;
            }
        }
    }

    state
}

fn execute_commands_two(commands: &[Command]) -> State {
    let mut state = State::new();

    for command in commands {
        match command {
            Command::Forward(num) => {
                state.distance += num;
                state.depth += num * state.aim;
            }
            Command::Up(num) => {
                state.aim -= num;
            }
            Command::Down(num) => {
                state.aim += num;
            }
        }
    }

    state
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{file_io, parse};

    const TEST_DATA: &str = r"forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_execute_commands_one() {
        let commands = parse::transform_iter(TEST_DATA.split('\n'), |s| s.parse()).unwrap();
        let result = execute_commands_one(&commands);
        assert_eq!(result.depth * result.distance, 150);
    }

    #[test]
    fn test_execute_commands_two() {
        let commands = parse::transform_iter(TEST_DATA.split('\n'), |s| s.parse()).unwrap();
        let result = execute_commands_two(&commands);
        assert_eq!(result.depth * result.distance, 900);
    }

    #[test]
    fn test_d02() {
        let commands = file_io::read_lines_as_structs("inputs/d02").unwrap();
        let state_one = execute_commands_one(&commands);
        println!("Day 02 result #1: {}", state_one.depth * state_one.distance);

        let state_two = execute_commands_two(&commands);
        println!("Day 02 result #2: {}", state_two.depth * state_two.distance);
    }
}
