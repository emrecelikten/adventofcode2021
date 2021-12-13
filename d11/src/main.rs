use common::collections::{ArrayGrid, DenseGrid};
use common::error::CommonError;
use std::fs::read_to_string;

fn flash(grid: &mut ArrayGrid<i32>, x: usize, y: usize) -> usize {
    let fill_fn = |grid: &mut ArrayGrid<i32>, x: usize, y: usize| {
        let cur = grid.get_pos(x, y);
        if *cur <= 9 {
            return None;
        }
        // Hax
        grid.set_pos(x, y, i32::MIN);
        let neighbours = grid.get_neighbours(x, y);
        for (x_n, y_n) in neighbours {
            let cur_neigh = grid.get_pos(x_n, y_n);
            if *cur_neigh >= 0 {
                grid.set_pos(x_n, y_n, cur_neigh + 1);
            }
        }

        Some(1)
    };

    let expand_fn = |grid: &ArrayGrid<i32>, x: usize, y: usize| {
        let neighbours = grid.get_neighbours(x, y);

        neighbours
            .iter()
            .filter(|&&(x_n, y_n)| {
                let cur_neigh = grid.get_pos(x_n, y_n);
                *cur_neigh > 9
            })
            .cloned()
            .collect()
    };

    grid.generic_flood_fill_mut(x, y, fill_fn, expand_fn)
}

fn step(grid: &mut ArrayGrid<i32>) -> usize {
    for e in grid.underlying.iter_mut() {
        *e += 1;
    }

    let mut num_flashes = 0;
    for y in 0..grid.y_size {
        for x in 0..grid.x_size {
            if *grid.get_pos(x, y) >= 9 {
                num_flashes += flash(grid, x, y);
            }
        }
    }

    for e in grid.underlying.iter_mut() {
        if *e < 0 {
            *e = 0;
        }
    }

    num_flashes
}

fn iterate(grid: &mut ArrayGrid<i32>, num_steps: usize) -> usize {
    let mut num_flashes = 0;
    for _ in 0..num_steps {
        num_flashes += step(grid);
    }
    num_flashes
}

fn find_sync(grid: &mut ArrayGrid<i32>) -> usize {
    let mut step_num = 0;
    loop {
        step(grid);
        step_num += 1;
        if grid.underlying.iter().all(|e| *e == 0) {
            return step_num;
        }
    }
}

fn main() -> Result<(), CommonError> {
    let data = read_to_string("input")?;
    let mut grid: ArrayGrid<i32> = data.parse()?;

    let num_flashes = iterate(&mut grid.clone(), 100);
    println!("Result #1: {}", num_flashes);

    let sync_step = find_sync(&mut grid);
    println!("Result #2: {}", sync_step);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_SMALL: &str = r"11111
19991
19191
19991
11111";

    const TEST_DATA: &str = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_flash() {
        let mut grid: ArrayGrid<i32> = ArrayGrid::from_str(&TEST_DATA_SMALL).unwrap();
        let num_flashes = flash(&mut grid, 1, 1);
        assert_eq!(num_flashes, 9);
    }

    #[test]
    fn test_step() {
        let large_data = r"6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637";

        let mut grid: ArrayGrid<i32> = ArrayGrid::from_str(&large_data).unwrap();
        let expected = r"8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848";

        let expected_grid: ArrayGrid<i32> = ArrayGrid::from_str(&expected).unwrap();
        let num_flashes = step(&mut grid);
        assert_eq!(num_flashes, 35);
        assert_eq!(grid, expected_grid);
    }

    #[test]
    fn test_iterate() {
        let mut grid: ArrayGrid<i32> = ArrayGrid::from_str(&TEST_DATA).unwrap();
        let num_flashes = iterate(&mut grid, 10);
        assert_eq!(num_flashes, 204);
    }

    #[test]
    fn test_find_sync() {
        let mut grid: ArrayGrid<i32> = ArrayGrid::from_str(&TEST_DATA).unwrap();
        let sync_step = find_sync(&mut grid);
        assert_eq!(sync_step, 195);
    }
}
