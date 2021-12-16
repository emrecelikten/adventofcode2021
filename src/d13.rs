use crate::common::collections::sparse_grid::HashGrid;
use crate::common::error::CommonError;
use crate::common::parse;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

impl FromStr for Fold {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('=');
        let axis = iter
            .next()
            .ok_or(CommonError::Parse(
                "Malformed instruction string regarding axis.",
            ))?
            .chars()
            .last()
            .unwrap();
        let coord = iter
            .next()
            .ok_or(CommonError::Parse(
                "Malformed instruction string regarding coord.",
            ))?
            .parse()?;

        match axis {
            'x' => Ok(Fold::X(coord)),
            'y' => Ok(Fold::Y(coord)),
            _ => Err(CommonError::Parse("Unknown axis detected.")),
        }
    }
}

fn parse_lines<S: AsRef<str>>(lines: &[S]) -> (HashGrid<char>, Vec<Fold>) {
    let mut grid = HashGrid::new();
    let mut iter = lines.iter();
    for line in &mut iter {
        if line.as_ref().is_empty() {
            break;
        }
        let splitted: Vec<usize> = line
            .as_ref()
            .split(',')
            .map(|e| e.trim().parse().unwrap())
            .collect();

        assert_eq!(splitted.len(), 2);

        grid.insert((splitted[0], splitted[1]), '#');
    }

    let instructions = parse::transform_iter(iter, |e| e.as_ref().parse()).unwrap();

    (grid, instructions)
}

fn fold(grid: &mut HashGrid<char>, fold: &Fold) {
    match fold {
        Fold::X(fold_x) => {
            let to_be_folded: Vec<(usize, usize)> =
                grid.keys().filter(|(x, _)| x >= fold_x).cloned().collect();

            for (x, y) in to_be_folded {
                grid.insert((fold_x - (x - fold_x), y), '#');
                grid.remove(&(x, y));
            }
        }
        Fold::Y(fold_y) => {
            let to_be_folded: Vec<(usize, usize)> =
                grid.keys().filter(|(_, y)| y >= fold_y).cloned().collect();

            for (x, y) in to_be_folded {
                grid.insert((x, fold_y - (y - fold_y)), '#');
                grid.remove(&(x, y));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::collections::sparse_grid::SparseGrid;
    use crate::common::file_io;

    const TEST_DATA: &str = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_parse_lines() {
        let lines: Vec<&str> = TEST_DATA.lines().collect();
        let (grid, instructions) = parse_lines(&lines);
        assert_eq!(grid.get_pos(0, 0), None);
        assert_eq!(grid.get_pos(6, 10), Some(&'#'));
        assert_eq!(grid.get_pos(9, 0), Some(&'#'));
        assert_eq!(grid.len(), 18);

        assert_eq!(instructions[0], Fold::Y(7));
        assert_eq!(instructions[1], Fold::X(5));
        assert_eq!(instructions.len(), 2);
    }

    #[test]
    fn test_fold() {
        let lines: Vec<&str> = TEST_DATA.lines().collect();
        let (mut grid, instructions) = parse_lines(&lines);

        fold(&mut grid, &instructions[0]);
        assert_eq!(grid.len(), 17);
        assert_eq!(grid.get_pos(0, 0), Some(&'#'));

        fold(&mut grid, &instructions[1]);
        assert_eq!(grid.len(), 16);
        assert_eq!(grid.get_pos(0, 4), Some(&'#'));
    }

    #[test]
    fn test_d13() {
        let lines = file_io::read_lines_as_strings("inputs/d13").unwrap();
        let (mut grid, instructions) = parse_lines(&lines);

        fold(&mut grid, &instructions[0]);
        let num_dots_after_first_fold = grid.len();
        println!("Day 13 result #1: {}", num_dots_after_first_fold);

        for instruction in instructions.iter().skip(1) {
            fold(&mut grid, instruction);
        }
        println!("Day 13 result #2: \n{}", grid);
    }
}
