extern crate common;

use common::error::CommonError;
use common::file_io;

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

fn main() -> Result<(), CommonError> {
    let numbers = file_io::read_file_as_numbers("input")?;

    let count = count_greater(&numbers);
    println!("Result #1: {}", count);

    let count_windows = count_windows(&numbers);
    println!("Result #2: {}", count_windows);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::file_io::transform_iter;

    const TEST_DATA: &'static str = r"199
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
        let numbers = transform_iter(TEST_DATA.split("\n"), |s| s.parse::<i64>()).unwrap();
        let count = count_greater(&numbers);
        assert_eq!(count, 7);
    }

    #[test]
    fn test_count_windows() {
        let numbers = transform_iter(TEST_DATA.split("\n"), |s| s.parse::<i64>()).unwrap();
        let count = count_windows(&numbers);
        assert_eq!(count, 5)
    }
}
