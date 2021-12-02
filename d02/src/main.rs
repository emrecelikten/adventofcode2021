extern crate common;

use common::error::CommonError;
use common::file_io;
use std::str::FromStr;

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
        let splitted: Vec<&str> = s.split(" ").collect();
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

fn main() -> Result<(), CommonError> {
    let lines = file_io::get_lines_iterator("input")?;
    let commands = file_io::transform_lines(lines, |s| Command::from_str(s))?;
    let state_one = execute_commands_one(&commands);
    println!("Result #1: {}", state_one.depth * state_one.distance);

    let state_two = execute_commands_two(&commands);
    println!("Result #2: {}", state_two.depth * state_two.distance);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &'static str = r"forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_execute_commands_one() {
        let commands =
            file_io::transform_iter(TEST_DATA.split("\n"), |x| Command::from_str(x)).unwrap();
        let result = execute_commands_one(&commands);
        assert_eq!(result.depth * result.distance, 150);
    }

    #[test]
    fn test_execute_commands_two() {
        let commands =
            file_io::transform_iter(TEST_DATA.split("\n"), |x| Command::from_str(x)).unwrap();
        let result = execute_commands_two(&commands);
        assert_eq!(result.depth * result.distance, 900);
    }
}
