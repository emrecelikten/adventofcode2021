use crate::error::CommonError;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

pub const NEIGHBOURS_CROSS_2D: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
pub const NEIGHBOURS_2D: [(i32, i32); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn _get_neighbours_array<V: Default>(
    grid: &ArrayGrid<V>,
    x: usize,
    y: usize,
    neighbours: &[(i32, i32)],
) -> Vec<(usize, usize)> {
    let neighbours: Vec<(usize, usize)> = neighbours
        .iter()
        .filter_map(|(x_neigh, y_neigh)| {
            let x_new = x as i32 + x_neigh;
            let y_new = y as i32 + y_neigh;
            if x_new >= 0 && x_new < grid.x_size as i32 && y_new >= 0 && y_new < grid.y_size as i32
            {
                Some((x_new as usize, y_new as usize))
            } else {
                None
            }
        })
        .collect();

    neighbours
}

pub trait SparseGrid<V: Default> {
    fn get_pos(&self, x: usize, y: usize) -> Option<&V>;
    fn get_pos_mut(&mut self, x: usize, y: usize) -> Option<&mut V>;
    fn get_or_insert_pos_mut(&mut self, x: usize, y: usize) -> &mut V;
    fn set_pos(&mut self, x: usize, y: usize, value: V);
}

pub trait DenseGrid<V: Default> {
    fn get_pos(&self, x: usize, y: usize) -> &V;
    fn get_pos_mut(&mut self, x: usize, y: usize) -> &mut V;
    fn set_pos(&mut self, x: usize, y: usize, value: V);
    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)>;
    fn get_neighbours_cross(&self, x: usize, y: usize) -> Vec<(usize, usize)>;
    fn boundary_fill_cross(&self, x: usize, y: usize, fill: V, boundary: V) -> (usize, Self);
    fn generic_flood_fill<FillFn, ExpandFn>(
        &self,
        x: usize,
        y: usize,
        fill_fn: FillFn,
        expand_fn: ExpandFn,
    ) -> (usize, Self)
    where
        FillFn: Fn(&mut Self, usize, usize) -> Option<usize>,
        ExpandFn: Fn(&Self, usize, usize) -> Vec<(usize, usize)>;
    fn generic_flood_fill_mut<FillFn, ExpandFn>(
        &mut self,
        x: usize,
        y: usize,
        fill_fn: FillFn,
        expand_fn: ExpandFn,
    ) -> usize
    where
        FillFn: Fn(&mut Self, usize, usize) -> Option<usize>,
        ExpandFn: Fn(&Self, usize, usize) -> Vec<(usize, usize)>;
}

pub type HashGrid<V> = HashMap<(usize, usize), V>;

impl<V: Default> SparseGrid<V> for HashGrid<V> {
    fn get_pos(&self, x: usize, y: usize) -> Option<&V> {
        self.get(&(x, y))
    }

    fn get_pos_mut(&mut self, x: usize, y: usize) -> Option<&mut V> {
        self.get_mut(&(x, y))
    }

    fn get_or_insert_pos_mut(&mut self, x: usize, y: usize) -> &mut V {
        self.entry((x, y)).or_insert_with(V::default)
    }

    fn set_pos(&mut self, x: usize, y: usize, value: V) {
        self.insert((x, y), value);
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ArrayGrid<V> {
    pub x_size: usize,
    pub y_size: usize,
    pub underlying: Box<[V]>,
}

impl<V: Default + Clone> ArrayGrid<V> {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        ArrayGrid {
            x_size,
            y_size,
            underlying: vec![V::default(); x_size * y_size].into_boxed_slice(),
        }
    }
}

impl<V: Default + Clone + PartialEq> DenseGrid<V> for ArrayGrid<V> {
    fn get_pos(&self, x: usize, y: usize) -> &V {
        &self.underlying[y * self.x_size + x]
    }

    fn get_pos_mut(&mut self, x: usize, y: usize) -> &mut V {
        &mut self.underlying[y * self.x_size + x]
    }

    fn set_pos(&mut self, x: usize, y: usize, value: V) {
        self.underlying[y * self.x_size + x] = value;
    }

    // This should normally return a impl Iterator, but doing it in trait methods seems to be
    // unstable unfortunately.
    fn get_neighbours_cross(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        _get_neighbours_array(self, x, y, &NEIGHBOURS_CROSS_2D)
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        _get_neighbours_array(self, x, y, &NEIGHBOURS_2D)
    }

    fn boundary_fill_cross(&self, x: usize, y: usize, fill: V, boundary: V) -> (usize, Self) {
        let fill_fn = |grid: &mut Self, x_cur: usize, y_cur: usize| {
            let cur = grid.get_pos(x_cur, y_cur);
            if cur != &boundary && cur != &fill {
                grid.set_pos(x_cur, y_cur, fill.clone());
                Some(1)
            } else {
                None
            }
        };

        let expand_fn = |grid: &Self, x_cur: usize, y_cur: usize| {
            let neighbours = self.get_neighbours_cross(x_cur, y_cur);
            neighbours
                .iter()
                .filter(|&&(x_neigh, y_neigh)| {
                    let neigh = grid.get_pos(x_neigh, y_neigh);
                    neigh != &boundary && neigh != &fill
                })
                .cloned()
                .collect()
        };

        self.generic_flood_fill(x, y, fill_fn, expand_fn)
    }

    fn generic_flood_fill<FillFn, ExpandFn>(
        &self,
        x: usize,
        y: usize,
        fill_fn: FillFn,
        expand_fn: ExpandFn,
    ) -> (usize, Self)
    where
        FillFn: Fn(&mut Self, usize, usize) -> Option<usize>,
        ExpandFn: Fn(&Self, usize, usize) -> Vec<(usize, usize)>,
    {
        let mut new_grid: ArrayGrid<V> = self.clone();
        (
            new_grid.generic_flood_fill_mut(x, y, fill_fn, expand_fn),
            new_grid,
        )
    }

    fn generic_flood_fill_mut<FillFn, ExpandFn>(
        &mut self,
        x: usize,
        y: usize,
        fill_fn: FillFn,
        expand_fn: ExpandFn,
    ) -> usize
    where
        FillFn: Fn(&mut Self, usize, usize) -> Option<usize>,
        ExpandFn: Fn(&Self, usize, usize) -> Vec<(usize, usize)>,
    {
        let mut num_filled = 0;
        let mut queue = VecDeque::new();
        queue.push_back((x, y));
        while !queue.is_empty() {
            let (x_cur, y_cur) = queue.pop_front().unwrap();

            let cur_filled = fill_fn(self, x_cur, y_cur);
            if cur_filled.is_none() {
                continue;
            }
            num_filled += cur_filled.unwrap();

            let neighbours = expand_fn(self, x_cur, y_cur);
            for (x_neigh, y_neigh) in neighbours {
                queue.push_back((x_neigh, y_neigh));
            }
        }

        num_filled
    }
}

impl FromStr for ArrayGrid<char> {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Vec<char> = s.chars().filter(|ch| *ch != '\n').collect();

        let x_size = s
            .chars()
            .position(|ch| ch == '\n')
            .ok_or(CommonError::Parse(
                "No linebreaks found while converting to grid.",
            ))?;
        let y_size = result.len() / x_size;

        if y_size * x_size != result.len() {
            Err(CommonError::Parse("Grid data is malformed."))
        } else {
            Ok(Self {
                x_size,
                y_size,
                underlying: result.into_boxed_slice(),
            })
        }
    }
}

impl FromStr for ArrayGrid<i32> {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Better
        let char_grid: ArrayGrid<char> = ArrayGrid::from_str(s)?;
        Ok(Self {
            x_size: char_grid.x_size,
            y_size: char_grid.y_size,
            underlying: char_grid
                .underlying
                .iter()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
                .into_boxed_slice(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sparse_get_set<G: SparseGrid<String>>(sparse_grid: &mut G) {
        let val1 = "LoremIpsum".to_string();
        let val2 = "DolorSit".to_string();
        sparse_grid.set_pos(4, 4, val1.clone());
        sparse_grid.set_pos(3, 3, val2.clone());

        assert_eq!(sparse_grid.get_pos(4, 4), Some(&val1));
        assert_eq!(sparse_grid.get_pos(3, 3), Some(&val2));

        assert_eq!(sparse_grid.get_pos(0, 0), None);
    }

    fn test_sparse_get_pos_mut<G: SparseGrid<String>>(sparse_grid: &mut G) {
        let val1 = "LoremIpsum".to_string();
        sparse_grid.set_pos(4, 4, val1);

        let val = sparse_grid.get_pos_mut(4, 4).unwrap();
        *val = "Test".to_string();

        assert_eq!(sparse_grid.get_pos(4, 4), Some(&"Test".to_string()));
    }

    fn test_dense_get_set<G: DenseGrid<String>>(dense_grid: &mut G) {
        let val1 = "LoremIpsum".to_string();
        let val2 = "DolorSit".to_string();
        dense_grid.set_pos(4, 4, val1.clone());
        dense_grid.set_pos(3, 3, val2.clone());

        assert_eq!(dense_grid.get_pos(4, 4), &val1);
        assert_eq!(dense_grid.get_pos(3, 3), &val2);

        assert_eq!(dense_grid.get_pos(0, 0), &String::default());
    }

    #[test]
    fn test_hashgrid_get_set() {
        let mut hash_grid = HashGrid::new();
        test_sparse_get_set(&mut hash_grid);
    }

    #[test]
    fn test_hashgrid_get_pos_mut() {
        let mut hash_grid = HashGrid::new();
        test_sparse_get_pos_mut(&mut hash_grid)
    }

    #[test]
    fn test_arraygrid_get_set() {
        let mut array_grid = ArrayGrid::new(5, 5);
        test_dense_get_set(&mut array_grid);
    }

    #[test]
    fn test_arraygrid_parse() {
        let data = r"1234567
2345678";

        let array_grid: ArrayGrid<char> = ArrayGrid::from_str(data).unwrap();
        assert_eq!(array_grid.x_size, 7);
        assert_eq!(array_grid.y_size, 2);
        assert_eq!(*array_grid.get_pos(2, 1), '4');
        assert_eq!(*array_grid.get_pos(6, 0), '7');
        assert_eq!(*array_grid.get_pos(6, 1), '8');
    }

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
        let array_grid = ArrayGrid::from_str(data).unwrap();
        let expected_grid = ArrayGrid::from_str(expected).unwrap();

        for x in 1..=5 {
            let (num_filled, filled_grid) = array_grid.boundary_fill_cross(x, 1, 'X', '#');
            assert_eq!(num_filled, 10);
            assert_eq!(filled_grid, expected_grid);
        }

        for x in 1..=3 {
            let (num_filled, filled_grid) = array_grid.boundary_fill_cross(x, 2, 'X', '#');
            assert_eq!(num_filled, 10);
            assert_eq!(filled_grid, expected_grid);
        }

        let (num_filled, filled_grid) = array_grid.boundary_fill_cross(5, 2, 'X', '#');
        assert_eq!(num_filled, 10);
        assert_eq!(filled_grid, expected_grid);

        let (num_filled, filled_grid) = array_grid.boundary_fill_cross(2, 3, 'X', '#');
        assert_eq!(num_filled, 10);
        assert_eq!(filled_grid, expected_grid);
    }
}
