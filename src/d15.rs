use crate::common::collections::dense_grid::{ArrayGrid, DenseGrid};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

type Coord = (usize, usize);

#[derive(PartialEq, Eq)]
struct Cost {
    coords: Coord,
    cost: i32, // TODO: Add u32 support for ArrayGrid
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn find_best_path_cost(grid: &ArrayGrid<i32>) -> i32 {
    let mut vertexes: BinaryHeap<Cost> = BinaryHeap::new();
    let mut costs: ArrayGrid<i32> =
        ArrayGrid::new_with_default(grid.x_size, grid.y_size, &i32::MAX);

    vertexes.push(Cost {
        coords: (0, 0),
        cost: 0,
    });

    while let Some(Cost { coords, cost }) = vertexes.pop() {
        let (x, y) = coords;
        if x == grid.x_size - 1 && y == grid.y_size - 1 {
            return cost;
        }

        for (x_neigh, y_neigh) in grid.get_neighbours_cross(x, y) {
            let new_cost = cost + grid.get_pos(x_neigh, y_neigh);
            if new_cost < *costs.get_pos(x_neigh, y_neigh) {
                costs.set_pos(x_neigh, y_neigh, new_cost);
                vertexes.push(Cost {
                    coords: (x_neigh, y_neigh),
                    cost: new_cost,
                });
            }
        }
    }
    panic!()
}

fn stack_grids(grid: &mut ArrayGrid<i32>) {
    let mut orig_grid = grid.clone();
    // Horizontal stacking - harder, so done first
    for i in 1..5 {
        let mut cur_grid = orig_grid.clone();
        cur_grid.underlying.iter_mut().for_each(|e| {
            *e += i;
            if *e > 9 {
                *e %= 9;
            }
        });

        assert!(grid.hstack_mut(cur_grid).is_ok());
    }

    orig_grid = grid.clone();

    // Vertical stacking
    for i in 1..5 {
        let mut cur_grid = orig_grid.clone();
        cur_grid.underlying.iter_mut().for_each(|e| {
            *e += i;
            if *e > 9 {
                *e %= 9;
            }
        });

        assert!(grid.vstack_mut(cur_grid).is_ok());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    const TEST_DATA: &str = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_find_best_path() {
        let grid: ArrayGrid<i32> = TEST_DATA.parse().unwrap();
        let cost = find_best_path_cost(&grid);
        assert_eq!(cost, 40);
    }

    #[test]
    fn test_stacked_cost() {
        let mut grid: ArrayGrid<i32> = TEST_DATA.parse().unwrap();
        stack_grids(&mut grid);
        assert_eq!(grid.x_size, 50);
        assert_eq!(grid.y_size, 50);

        let stacked_expected: ArrayGrid<i32> =
            read_to_string("inputs/d15_test").unwrap().parse().unwrap();
        assert_eq!(grid, stacked_expected);

        let cost = find_best_path_cost(&grid);
        assert_eq!(cost, 315);
    }

    #[test]
    fn test_d15() {
        let mut grid: ArrayGrid<i32> = read_to_string("inputs/d15").unwrap().parse().unwrap();
        let cost = find_best_path_cost(&grid);
        println!("Day 15 result #1: {}", cost);

        stack_grids(&mut grid);
        let stacked_cost = find_best_path_cost(&grid);
        println!("Day 15 result #2: {}", stacked_cost);
    }
}
