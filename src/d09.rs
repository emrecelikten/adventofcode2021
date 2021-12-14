#![allow(dead_code)]
use crate::common::algorithms;
use crate::common::collections::dense_grid::{ArrayGrid, DenseGrid};

type CoordPair = (usize, usize);

fn find_minima_positions(grid: &ArrayGrid<char>) -> Vec<CoordPair> {
    let mut minima = Vec::new();
    for y in 0..grid.y_size {
        for x in 0..grid.x_size {
            let cur = grid.get_pos(x, y);
            let is_minimum = grid
                .get_neighbours_cross(x, y)
                .iter()
                .all(|&(x_n, y_n)| grid.get_pos(x_n, y_n) > cur);

            if is_minimum {
                minima.push((x, y))
            }
        }
    }
    minima
}

fn calculate_risk_level_sum(grid: &ArrayGrid<char>, minima_positions: &[CoordPair]) -> usize {
    minima_positions
        .iter()
        .map(|&(x, y)| grid.get_pos(x, y).to_digit(10).unwrap() as usize + 1)
        .sum()
}

fn calculate_basin_size_mul(grid: &ArrayGrid<char>, minima_positions: &[CoordPair]) -> usize {
    let mut cur_grid = grid.clone();
    let mut basin_size: Vec<usize> = Vec::new();
    for &(x, y) in minima_positions {
        let num_filled = algorithms::grid::boundary_fill_cross_mut(&mut cur_grid, x, y, '*', '9');
        if num_filled > 0 {
            basin_size.push(num_filled);
        }
    }

    basin_size.sort_by(|a, b| b.partial_cmp(a).unwrap());
    basin_size.iter().take(3).product()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;

    const TEST_DATA: &str = r"2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_calculate_minima_sum() {
        let grid = TEST_DATA.parse().unwrap();
        let minima_positions = find_minima_positions(&grid);

        assert_eq!(minima_positions.len(), 4);
        assert_eq!(minima_positions[0], (1, 0));
        assert_eq!(minima_positions[1], (9, 0));
        assert_eq!(minima_positions[2], (2, 2));
        assert_eq!(minima_positions[3], (6, 4));

        let sum = calculate_risk_level_sum(&grid, &minima_positions);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_calculate_basin_size_mul() {
        let grid = TEST_DATA.parse().unwrap();
        let minima_positions = find_minima_positions(&grid);
        let sum = calculate_basin_size_mul(&grid, &minima_positions);
        assert_eq!(sum, 1134);
    }

    #[test]
    fn test_d09() {
        let grid = read_to_string("inputs/d09").unwrap().parse().unwrap();
        let minima_positions = find_minima_positions(&grid);

        let risk_level_sum = calculate_risk_level_sum(&grid, &minima_positions);
        println!("Day 09 result #1: {}", risk_level_sum);

        let basin_size_mul = calculate_basin_size_mul(&grid, &minima_positions);
        println!("Day 09 result #2: {}", basin_size_mul);
    }
}
