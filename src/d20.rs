use crate::common::collections::sparse_grid::{HashGrid, SparseGrid};
use std::fmt::Debug;

const OFFSET: usize = usize::MAX >> 1;

#[derive(Debug)]
struct Bounds {
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
    outside: char,
}
impl From<&HashGrid<char>> for Bounds {
    fn from(grid: &HashGrid<char>) -> Self {
        let mut bounds = Bounds {
            x_min: usize::MAX,
            x_max: usize::MIN,
            y_min: usize::MAX,
            y_max: usize::MIN,
            outside: '.',
        };

        for &(x, y) in grid.keys() {
            if x < bounds.x_min {
                bounds.x_min = x;
            }
            if y < bounds.y_min {
                bounds.y_min = y;
            }
            if x > bounds.x_max {
                bounds.x_max = x;
            }
            if y > bounds.y_max {
                bounds.y_max = y;
            }
        }
        bounds
    }
}

fn parse_data<S: AsRef<str> + Debug>(data: &[S]) -> (Vec<char>, HashGrid<char>) {
    let enhancement_algorithm = data[0].as_ref().chars().collect();
    let parsed: HashGrid<char> = data[1].as_ref().parse().unwrap();
    let mut grid: HashGrid<char> = HashGrid::new();
    // Hax
    for ((x, y), v) in &*parsed {
        grid.set_pos(x + OFFSET, y + OFFSET, *v);
    }
    (enhancement_algorithm, grid)
}

fn get_binary(x: usize, y: usize, grid: &HashGrid<char>, bounds: &Bounds) -> usize {
    let mut binary_str = String::with_capacity(9);
    let outside_digit = if bounds.outside == '#' { '1' } else { '0' };

    for y_neigh in -1..=1i8 {
        for x_neigh in -1..=1i8 {
            let x_new = if x_neigh.is_negative() {
                x - x_neigh.abs() as usize
            } else {
                x + x_neigh as usize
            };
            let y_new = if y_neigh.is_negative() {
                y - y_neigh.abs() as usize
            } else {
                y + y_neigh as usize
            };
            if x_new < bounds.x_min
                || x_new > bounds.x_max
                || y_new < bounds.y_min
                || y_new > bounds.y_max
            {
                binary_str.push(outside_digit)
            } else {
                match grid.get_pos(x_new, y_new) {
                    None => {
                        binary_str.push('0');
                    }
                    Some(_) => {
                        binary_str.push('1');
                    }
                }
            }
        }
    }

    usize::from_str_radix(&binary_str, 2).unwrap()
}

fn enhance(
    enhancement_algorithm: &[char],
    grid: &HashGrid<char>,
    bounds: &Bounds,
) -> (HashGrid<char>, Bounds) {
    let mut new = HashGrid::new();

    for y in bounds.y_min - 1..=bounds.y_max + 1 {
        for x in bounds.x_min - 1..=bounds.x_max + 1 {
            let pos = get_binary(x, y, grid, bounds);
            let ch = enhancement_algorithm[pos];
            if ch == '#' {
                new.insert((x, y), ch);
            }
        }
    }

    let new_outside = if bounds.outside == '#' {
        *enhancement_algorithm.last().unwrap()
    } else {
        enhancement_algorithm[0]
    };

    let new_bounds = Bounds {
        x_min: bounds.x_min - 1,
        x_max: bounds.x_max + 1,
        y_min: bounds.y_min - 1,
        y_max: bounds.y_max + 1,
        outside: new_outside,
    };

    (new, new_bounds)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_get_binary() {
        let str = read_to_string("inputs/d20_test").unwrap();
        let data: Vec<&str> = str.split("\n\n").collect();
        let (enhancement_algorithm, grid) = parse_data(&data);
        let bounds = Bounds::from(&grid);
        let binary_value = get_binary(2 + OFFSET, 2 + OFFSET, &grid, &bounds);
        assert_eq!(binary_value, 34);
        assert_eq!(enhancement_algorithm[binary_value], '#');
    }

    #[test]
    fn test_enhance() {
        let str = read_to_string("inputs/d20_test").unwrap();
        let data: Vec<&str> = str.split("\n\n").collect();
        let (enhancement_algorithm, mut grid) = parse_data(&data);
        let mut bounds = Bounds::from(&grid);

        for _ in 0..2 {
            let result = enhance(&enhancement_algorithm, &grid, &bounds);
            grid = result.0;
            bounds = result.1;
        }
        assert_eq!(grid.len(), 35);

        for _ in 0..48 {
            let result = enhance(&enhancement_algorithm, &grid, &bounds);
            grid = result.0;
            bounds = result.1;
        }

        assert_eq!(grid.len(), 3351);
    }

    #[test]
    fn test_d20() {
        let str = read_to_string("inputs/d20").unwrap();
        let data: Vec<&str> = str.split("\n\n").collect();
        let (enhancement_algorithm, mut grid) = parse_data(&data);
        let mut bounds = Bounds::from(&grid);
        for _ in 0..2 {
            let result = enhance(&enhancement_algorithm, &grid, &bounds);
            grid = result.0;
            bounds = result.1;
        }
        println!("Day 20 result #1: {}", grid.len());
        for _ in 0..48 {
            let result = enhance(&enhancement_algorithm, &grid, &bounds);
            grid = result.0;
            bounds = result.1;
        }
        println!("Day 20 result #2: {}", grid.len());
    }
}
