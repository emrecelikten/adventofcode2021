use crate::common::collections::dense_grid::DenseGrid;
use std::collections::VecDeque;

pub fn generic_flood_fill_mut<G, V, FillFn, ExpandFn>(
    grid: &mut G,
    x: usize,
    y: usize,
    fill_fn: FillFn,
    expand_fn: ExpandFn,
) -> usize
where
    V: Default,
    G: DenseGrid<V>,
    FillFn: Fn(&mut G, usize, usize) -> Option<usize>,
    ExpandFn: Fn(&G, usize, usize) -> Vec<(usize, usize)>,
{
    let mut num_filled = 0;
    let mut queue = VecDeque::new();
    queue.push_back((x, y));
    while !queue.is_empty() {
        let (x_cur, y_cur) = queue.pop_front().unwrap();

        let cur_filled = fill_fn(grid, x_cur, y_cur);
        if cur_filled.is_none() {
            continue;
        }
        num_filled += cur_filled.unwrap();

        let neighbours = expand_fn(grid, x_cur, y_cur);
        for (x_neigh, y_neigh) in neighbours {
            queue.push_back((x_neigh, y_neigh));
        }
    }

    num_filled
}

pub fn boundary_fill_cross_mut<G, V>(
    grid: &mut G,
    x: usize,
    y: usize,
    fill: V,
    boundary: V,
) -> usize
where
    V: Default + PartialEq + Clone,
    G: DenseGrid<V>,
{
    let fill_fn = |grid: &mut G, x_cur: usize, y_cur: usize| {
        let cur = grid.get_pos(x_cur, y_cur);
        if cur != &boundary && cur != &fill {
            grid.set_pos(x_cur, y_cur, fill.clone());
            Some(1)
        } else {
            None
        }
    };

    let expand_fn = |grid: &G, x_cur: usize, y_cur: usize| {
        let neighbours = grid.get_neighbours_cross(x_cur, y_cur);
        neighbours
            .iter()
            .filter(|&&(x_neigh, y_neigh)| {
                let neigh = grid.get_pos(x_neigh, y_neigh);
                neigh != &boundary && neigh != &fill
            })
            .cloned()
            .collect()
    };

    generic_flood_fill_mut(grid, x, y, fill_fn, expand_fn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::collections::dense_grid::ArrayGrid;

    #[test]
    fn test_boundary_fill_cross() {
        let data = r"#######
#.....#
#...#.#
##.#.##";

        let expected = r"#######
#XXXXX#
#XXX#X#
##X#.##";
        let orig_grid: ArrayGrid<char> = data.parse().unwrap();
        let expected_grid: ArrayGrid<char> = expected.parse().unwrap();

        for x in 1..=5 {
            let mut grid = orig_grid.clone();
            let num_filled = boundary_fill_cross_mut(&mut grid, x, 1, 'X', '#');
            assert_eq!(num_filled, 10);
            assert_eq!(grid, expected_grid);
        }

        for x in 1..=3 {
            let mut grid = orig_grid.clone();
            let num_filled = boundary_fill_cross_mut(&mut grid, x, 2, 'X', '#');
            assert_eq!(num_filled, 10);
            assert_eq!(grid, expected_grid);
        }

        {
            let mut grid = orig_grid.clone();
            let num_filled = boundary_fill_cross_mut(&mut grid, 5, 2, 'X', '#');
            assert_eq!(num_filled, 10);
            assert_eq!(grid, expected_grid);
        }

        {
            let mut grid = orig_grid;
            let num_filled = boundary_fill_cross_mut(&mut grid, 2, 3, 'X', '#');
            assert_eq!(num_filled, 10);
            assert_eq!(grid, expected_grid);
        }
    }
}
