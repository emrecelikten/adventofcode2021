use std::fs::read_to_string;
use std::io;
use std::str::FromStr;

fn parse_data<S: AsRef<str>>(data: S) -> Vec<u8> {
    data.as_ref()
        .trim()
        .split(",")
        .map(|x| u8::from_str(x).unwrap())
        .collect()
}

fn step(fishes: &mut Vec<u8>) {
    let mut new_fishes = 0;

    for fish in fishes.iter_mut() {
        if *fish == 0 {
            new_fishes += 1;
            *fish = 6;
        } else {
            *fish -= 1;
        }
    }

    fishes.append(&mut vec![8u8; new_fishes]);
}

fn simulate_growth(fishes: &Vec<u8>, days: usize) -> usize {
    let mut cur_fishes = fishes.clone();
    for _ in 0..days {
        step(&mut cur_fishes)
    }

    cur_fishes.len()
}

fn simulate_growth_large(fishes: &Vec<u8>, days: usize) -> usize {
    // Keep an array of counts, e.g. "3, 4, 1" becomes [0, 1, 0, 1, 1, 0, 0, 0, 0]
    // Iterate over these, when a non-zero is reached, move contents to i+6 % 9, add one fish for each
    // to i+8 % 9
    let mut days_to_breed = [0usize; 9];

    for fish in fishes {
        days_to_breed[*fish as usize] += 1;
    }

    for i in 0..days {
        let mi = i % 9;
        days_to_breed[(mi + 7) % 9] += days_to_breed[mi];
    }

    days_to_breed.iter().sum()
}

fn main() -> Result<(), io::Error> {
    let data = read_to_string("input")?;
    let fishes = parse_data(&data);
    let result = simulate_growth(&fishes, 80);
    println!("Result #1; {}", result);

    let result_large = simulate_growth_large(&fishes, 256);
    println!("Result #2; {}", result_large);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &'static str = r"3,4,3,1,2";

    #[test]
    fn test_step() {
        let mut fishes = vec![1, 0];
        step(&mut fishes);
        assert_eq!(fishes, vec![0, 6, 8]);

        step(&mut fishes);
        assert_eq!(fishes, vec![6, 5, 7, 8]);
    }

    #[test]
    fn test_simulate() {
        let fishes = parse_data(&TEST_DATA);
        assert_eq!(fishes, vec![3, 4, 3, 1, 2]);

        let result_after_18 = simulate_growth(&fishes, 18);
        assert_eq!(result_after_18, 26);

        let result_after_80 = simulate_growth(&fishes, 80);
        assert_eq!(result_after_80, 5934);
    }

    #[test]
    fn test_simulate_large() {
        let fishes = parse_data(&TEST_DATA);
        assert_eq!(fishes, vec![3, 4, 3, 1, 2]);

        let result_after_18 = simulate_growth_large(&fishes, 18);
        assert_eq!(result_after_18, 26);

        let result_after_80 = simulate_growth_large(&fishes, 80);
        assert_eq!(result_after_80, 5934);

        let result_after_256 = simulate_growth_large(&fishes, 256);
        assert_eq!(result_after_256, 26984457539);
    }
}
