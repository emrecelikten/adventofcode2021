extern crate common;

use common::error::CommonError;
use common::file_io;

fn find_most_common_bit_in_column<S: AsRef<str>>(lines: &[S], column: usize) -> char {
    let sum = lines.iter().fold(0.0, |acc, line| {
        let ch = line.as_ref().chars().nth(column).unwrap();
        acc + ch.to_digit(10).unwrap() as f64
    });

    if (sum / lines.len() as f64).round() == 0.0 {
        '0'
    } else {
        '1'
    }
}

fn inverse_char(ch: char) -> char {
    if ch == '0' {
        '1'
    } else {
        '0'
    }
}

fn calculate_gamma_epsilon_mul<S: AsRef<str>>(lines: &[S]) -> u64 {
    let line_length = lines[0].as_ref().len();
    let most_common = (0..line_length).fold(String::new(), |mut bitvec, column| {
        bitvec.push(find_most_common_bit_in_column(lines, column));
        bitvec
    });

    let gamma = u64::from_str_radix(&most_common, 2).unwrap();
    let inverse = most_common
        .chars()
        .map(|ch| inverse_char(ch))
        .collect::<String>();
    let epsilon = u64::from_str_radix(&inverse, 2).unwrap();

    gamma * epsilon
}

fn sieve_lines<S: AsRef<str>>(lines: &[S], most_common: bool) -> Vec<&S> {
    let mut remaining: Vec<&S> = lines.iter().collect();
    let line_length = lines[0].as_ref().len();

    for column in 0..line_length {
        let mut most_common_col = find_most_common_bit_in_column(&remaining, column);
        if !most_common {
            most_common_col = inverse_char(most_common_col)
        }

        remaining = remaining
            .into_iter()
            .filter(|line| line.as_ref().chars().nth(column).unwrap() == most_common_col)
            .collect();

        if remaining.len() == 1 {
            break;
        }
    }

    remaining
}

fn find_oxygen_coscrubber_rating_mul<S: AsRef<str>>(lines: &[S]) -> u64 {
    let oxygen_remaining = sieve_lines(lines, true);
    assert_eq!(oxygen_remaining.len(), 1);
    let oxygen = u64::from_str_radix(oxygen_remaining[0].as_ref(), 2).unwrap();

    let co_scrubber_remaining = sieve_lines(lines, false);
    assert_eq!(co_scrubber_remaining.len(), 1);
    let co_scrubber = u64::from_str_radix(co_scrubber_remaining[0].as_ref(), 2).unwrap();

    oxygen * co_scrubber
}

fn main() -> Result<(), CommonError> {
    let lines = file_io::read_file_as_strings("input")?;
    let gamma_epsilon_mul = calculate_gamma_epsilon_mul(&lines);
    println!("Result #1: {}", gamma_epsilon_mul);

    let oxygen_coscrubber_mul = find_oxygen_coscrubber_rating_mul(&lines);
    println!("Result #2: {}", oxygen_coscrubber_mul);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &'static str = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_calculate_epsilon_gamma_mul() {
        let lines: Vec<&str> = TEST_DATA.split("\n").collect();
        let result = calculate_gamma_epsilon_mul(&lines);
        assert_eq!(result, 198)
    }

    #[test]
    fn test_find_oxygen_coscrubber_rating_mul() {
        let lines: Vec<&str> = TEST_DATA.split("\n").collect();
        let result = find_oxygen_coscrubber_rating_mul(&lines);
        assert_eq!(result, 230)
    }
}
