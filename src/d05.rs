#![allow(dead_code)]
use crate::common::collections::sparse_grid::{HashGrid, SparseGrid};
use crate::common::error::CommonError;
use std::str::FromStr;

#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl FromStr for Line {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num_iter = s
            .split(" -> ")
            .flat_map(|chunk| chunk.split(','))
            .map(|elem| usize::from_str(elem));

        let result = Line {
            x1: num_iter
                .next()
                .ok_or(CommonError::Parse("Malformed input"))??,
            y1: num_iter
                .next()
                .ok_or(CommonError::Parse("Malformed input"))??,
            x2: num_iter
                .next()
                .ok_or(CommonError::Parse("Malformed input"))??,
            y2: num_iter
                .next()
                .ok_or(CommonError::Parse("Malformed input"))??,
        };

        Ok(result)
    }
}

fn trace_line_on_grid(line: &Line, grid: &mut HashGrid<usize>) {
    let x_diff = line.x1 as f64 - line.x2 as f64;
    let y_diff = line.y1 as f64 - line.y2 as f64;

    if x_diff.abs() >= y_diff.abs() {
        for x in 0..=x_diff.abs() as usize {
            // Linear interpolation
            let y = ((y_diff * x as f64) / x_diff.abs()).round();

            let x_pos = (line.x1 as f64 - x as f64 * x_diff.signum()) as usize;
            let y_pos = (line.y1 as f64 - y) as usize;

            *grid.get_or_insert_pos_mut(x_pos, y_pos, &0) += 1;
        }
    } else {
        for y in 0..=y_diff.abs() as usize {
            // Linear interpolation
            let x = ((x_diff * y as f64) / y_diff.abs()).round();

            let x_pos = (line.x1 as f64 - x) as usize;
            let y_pos = (line.y1 as f64 - y as f64 * y_diff.signum()) as usize;

            *grid.get_or_insert_pos_mut(x_pos, y_pos, &0) += 1;
        }
    }
}

fn count_overlaps(grid: &HashGrid<usize>) -> usize {
    grid.iter().filter(|&(_, v)| *v >= 2).count()
}

fn find_non_diag_overlaps(lines: &[Line]) -> usize {
    let mut grid = HashGrid::new();
    lines
        .iter()
        .filter(|line| line.x1 == line.x2 || line.y1 == line.y2)
        .for_each(|line| trace_line_on_grid(line, &mut grid));

    count_overlaps(&grid)
}

fn find_all_overlaps(lines: &[Line]) -> usize {
    let mut grid = HashGrid::new();
    lines
        .iter()
        .for_each(|line| trace_line_on_grid(line, &mut grid));

    count_overlaps(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{file_io, parse};
    const TEST_DATA: &str = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_trace_line_on_grid() {
        let mut grid = HashGrid::new();

        // Horizontal
        let line_horizontal = Line {
            x1: 5,
            y1: 0,
            x2: 0,
            y2: 0,
        };

        trace_line_on_grid(&line_horizontal, &mut grid);
        assert_eq!(grid.get_pos(0, 0), Some(&1));
        assert_eq!(grid.get_pos(3, 0), Some(&1));
        assert_eq!(grid.get_pos(5, 0), Some(&1));

        // Vertical
        let line_vertical = Line {
            x1: 0,
            y1: 5,
            x2: 0,
            y2: 0,
        };
        trace_line_on_grid(&line_vertical, &mut grid);
        trace_line_on_grid(&line_vertical, &mut grid);
        assert_eq!(grid.get_pos(0, 0), Some(&3));
        assert_eq!(grid.get_pos(0, 3), Some(&2));
        assert_eq!(grid.get_pos(0, 5), Some(&2));

        // Diagonal
        let line_diag = Line {
            x1: 3,
            y1: 1,
            x2: 1,
            y2: 3,
        };

        trace_line_on_grid(&line_diag, &mut grid);
        assert_eq!(grid.get_pos(3, 1), Some(&1));
        assert_eq!(grid.get_pos(2, 2), Some(&1));
        assert_eq!(grid.get_pos(1, 3), Some(&1));

        let line_diag_rev = Line {
            x1: 1,
            y1: 3,
            x2: 3,
            y2: 1,
        };

        trace_line_on_grid(&line_diag_rev, &mut grid);
        assert_eq!(grid.get_pos(1, 3), Some(&2));
        assert_eq!(grid.get_pos(2, 2), Some(&2));
        assert_eq!(grid.get_pos(3, 1), Some(&2));
    }

    #[test]
    fn test_count_overlaps() {
        let mut grid = HashGrid::new();

        // Vertical
        let line1 = Line {
            x1: 0,
            y1: 5,
            x2: 0,
            y2: 0,
        };
        trace_line_on_grid(&line1, &mut grid);
        trace_line_on_grid(&line1, &mut grid);

        // Diagonal
        let line3 = Line {
            x1: 1,
            y1: 2,
            x2: 5,
            y2: 3,
        };

        trace_line_on_grid(&line3, &mut grid);
        let result = count_overlaps(&grid);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_find_non_diag_overlaps() {
        let lines = parse::transform_iter(TEST_DATA.lines(), |l| Line::from_str(l)).unwrap();
        let overlaps = find_non_diag_overlaps(&lines);
        assert_eq!(overlaps, 5);
    }

    #[test]
    fn test_find_all_overlaps() {
        let lines = parse::transform_iter(TEST_DATA.lines(), |l| Line::from_str(l)).unwrap();
        let overlaps = find_all_overlaps(&lines);
        assert_eq!(overlaps, 12);
    }

    #[test]
    fn test_d05() {
        let lines = file_io::read_lines_as_structs("inputs/d05").unwrap();
        let non_diag_overlaps = find_non_diag_overlaps(&lines);
        println!("Day 05 result #1: {}", non_diag_overlaps);

        let all_overlaps = find_all_overlaps(&lines);
        println!("Day 05 result #2: {}", all_overlaps);
    }
}
