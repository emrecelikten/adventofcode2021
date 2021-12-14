#![allow(dead_code)]
use std::cmp::*;
use std::str::FromStr;

fn parse_data<S: AsRef<str>>(s: S) -> Vec<u64> {
    s.as_ref()
        .trim()
        .split(',')
        .map(|x| u64::from_str(x).unwrap())
        .collect()
}

fn compute_median(numbers: &[u64]) -> u64 {
    let mut sorted = numbers.to_vec();
    sorted.sort_unstable();
    sorted[sorted.len() / 2]
}

fn compute_total_distance_to_median(positions: &[u64]) -> u64 {
    let median = compute_median(positions);
    positions
        .iter()
        .fold(0, |acc, &x| acc + max(median, x) - min(median, x))
}

fn compute_nonconstant_fuel_usage(x: u64, y: u64) -> u64 {
    let diff = max(x, y) - min(x, y);
    (diff * (diff + 1)) / 2 // Gauss addition
}

fn compute_total_nonconstant_fuel_spent(positions: &[u64], target: u64) -> u64 {
    positions
        .iter()
        .fold(0, |acc, &x| acc + compute_nonconstant_fuel_usage(target, x))
}

fn compute_optimum_total_distance_nonconstant(positions: &[u64]) -> u64 {
    // Not sure if there is a closed form solution for this
    // but the optimization surface should be convex since there is only a single minimum
    // We can do a numerical derivative and use a gradient descentish algorithm
    // Assume median is a good place to start
    let mut best_pos = compute_median(positions);
    let mut prev = compute_total_nonconstant_fuel_spent(positions, best_pos as u64) as f64;
    let mut prev_d: Option<f64> = None;
    let alpha = 2.0 * prev.sqrt() / prev; // Heuristic
                                          // let alpha = *positions.iter().max().unwrap() as f64 / prev.sqrt(); // Heuristic
    loop {
        // We would like to go until our gradient flips its sign and is abs() <= 1.5
        let up = compute_total_nonconstant_fuel_spent(positions, best_pos + 1) as f64;
        let mut grad = alpha * (up - prev);

        // Prevent getting stuck on the same pos
        // Rounds down if neg, up if pos
        if grad.abs() < 1.0 {
            grad = grad.signum()
        }
        let new_best_pos = (best_pos as f64 - grad) as u64;

        let cur = compute_total_nonconstant_fuel_spent(positions, new_best_pos) as f64;

        if prev_d.is_some() && prev_d.unwrap().signum() != grad.signum() && grad.abs() < 1.5 {
            break;
        }
        prev = cur;
        prev_d = Some(grad);
        best_pos = new_best_pos;
    }

    prev.round() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    const TEST_DATA: &str = r"16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_compute_median() {
        let data = parse_data(&TEST_DATA);
        let median = compute_median(&data);
        assert_eq!(median, 2)
    }

    #[test]
    fn test_compute_total_distance_to_median() {
        let data = parse_data(&TEST_DATA);
        let distance = compute_total_distance_to_median(&data);
        assert_eq!(distance, 37);
    }

    #[test]
    fn test_compute_optimum_distance() {
        let data = parse_data(&TEST_DATA);
        let distance = compute_optimum_total_distance_nonconstant(&data);
        assert_eq!(distance, 168);
    }

    #[test]
    fn test_d07() {
        let data = read_to_string("inputs/d07").unwrap();
        let positions = parse_data(data);

        let total_distance = compute_total_distance_to_median(&positions);
        println!("Day 07 result #1: {}", total_distance);

        let total_nonconstant_distance = compute_optimum_total_distance_nonconstant(&positions);
        println!("Day 07 result #2: {}", total_nonconstant_distance);
    }
}
