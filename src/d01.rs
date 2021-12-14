#![allow(dead_code)]

fn count_greater(numbers: &[i64]) -> u64 {
    let mut prev = numbers[0];
    let mut count = 0;

    for &num in numbers.iter().skip(1) {
        if num > prev {
            count += 1;
        }
        prev = num;
    }

    count
}

fn count_windows(numbers: &[i64]) -> u64 {
    let sums: Vec<i64> = numbers
        .windows(3)
        .map(|triple| triple.iter().sum())
        .collect();

    count_greater(&sums)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{file_io, parse};

    const TEST_DATA: &str = r"199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_count_greater() {
        let numbers = parse::transform_iter(TEST_DATA.split('\n'), |s| s.parse::<i64>()).unwrap();
        let count = count_greater(&numbers);
        assert_eq!(count, 7);
    }

    #[test]
    fn test_count_windows() {
        let numbers = parse::transform_iter(TEST_DATA.split('\n'), |s| s.parse::<i64>()).unwrap();
        let count = count_windows(&numbers);
        assert_eq!(count, 5)
    }

    #[test]
    fn test_d01() {
        let numbers = file_io::read_lines_as_numbers("inputs/d01").unwrap();

        let count = count_greater(&numbers);
        println!("Day 01 result #1: {}", count);

        let count_windows = count_windows(&numbers);
        println!("Day 01 result #2: {}", count_windows);
    }
}
