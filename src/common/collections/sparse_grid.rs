use crate::common::error::CommonError;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

pub trait SparseGrid<V: Clone> {
    fn get_pos(&self, x: usize, y: usize) -> Option<&V>;
    fn get_pos_mut(&mut self, x: usize, y: usize) -> Option<&mut V>;
    fn get_or_insert_pos_mut(&mut self, x: usize, y: usize, value: &V) -> &mut V;
    fn set_pos(&mut self, x: usize, y: usize, value: V);
    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)>;
    fn get_neighbours_cross(&self, x: usize, y: usize) -> Vec<(usize, usize)>;
}

type HMap<V> = std::collections::HashMap<(usize, usize), V>;

#[derive(Clone)]
pub struct HashGrid<V> {
    underlying: HMap<V>,
}

impl<V: Clone> HashGrid<V> {
    pub fn new() -> Self {
        HashGrid {
            underlying: Default::default(),
        }
    }
}

impl<V: Clone> Default for HashGrid<V> {
    fn default() -> Self {
        Self::new()
    }
}

fn _get_neighbours_hash<V: Clone>(
    grid: &HashGrid<V>,
    x: usize,
    y: usize,
    neighbours: &[(i32, i32)],
) -> Vec<(usize, usize)> {
    let neighbours: Vec<(usize, usize)> = neighbours
        .iter()
        .filter_map(|&(x_neigh, y_neigh)| {
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
            if grid.contains_key(&(x_new, y_new)) {
                Some((x_new, y_new))
            } else {
                None
            }
        })
        .collect();

    neighbours
}

impl<V: Clone> SparseGrid<V> for HashGrid<V> {
    fn get_pos(&self, x: usize, y: usize) -> Option<&V> {
        self.get(&(x, y))
    }

    fn get_pos_mut(&mut self, x: usize, y: usize) -> Option<&mut V> {
        self.get_mut(&(x, y))
    }

    fn get_or_insert_pos_mut(&mut self, x: usize, y: usize, value: &V) -> &mut V {
        self.entry((x, y)).or_insert_with(|| value.clone())
    }

    fn set_pos(&mut self, x: usize, y: usize, value: V) {
        self.insert((x, y), value);
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        _get_neighbours_hash(self, x, y, &crate::common::collections::NEIGHBOURS_2D)
    }

    fn get_neighbours_cross(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        _get_neighbours_hash(self, x, y, &crate::common::collections::NEIGHBOURS_CROSS_2D)
    }
}

impl FromStr for HashGrid<char> {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().filter(|ch| *ch != '\n').collect();

        let x_size = s
            .chars()
            .position(|ch| ch == '\n')
            .ok_or(CommonError::Parse(
                "No linebreaks found while converting to grid.",
            ))?;
        let y_size = chars.len() / x_size;

        if y_size * x_size != chars.len() {
            Err(CommonError::Parse("Grid data is malformed."))
        } else {
            let mut result = HMap::new();

            for x in 0..x_size {
                for y in 0..y_size {
                    let ch = chars[y * x_size + x];
                    if ch == '#' {
                        result.insert((x, y), ch);
                    }
                }
            }
            Ok(Self { underlying: result })
        }
    }
}

impl<V> Deref for HashGrid<V> {
    type Target = HMap<V>;

    fn deref(&self) -> &Self::Target {
        &self.underlying
    }
}

impl<V> DerefMut for HashGrid<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.underlying
    }
}

impl Display for HashGrid<char> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut min_x = usize::MAX;
        let mut max_x = usize::MIN;
        let mut min_y = usize::MAX;
        let mut max_y = usize::MIN;

        for &(x, y) in self.keys() {
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }

        let x_size = max_x - min_x + 2; // +1 for newlines
        let y_size = max_y - min_y + 1;
        let mut vec = vec!['.'; y_size * (x_size) - 1];

        for y in 1..y_size {
            vec[y * x_size - 1] = '\n';
        }

        for (&(x, y), &ch) in self.iter() {
            vec[x - min_x + (y - min_y) * x_size] = ch;
        }

        write!(f, "{}", vec.iter().collect::<String>())
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
    fn test_display() {
        let mut hash_grid = HashGrid::new();
        hash_grid.set_pos(5, 5, '#');
        hash_grid.set_pos(6, 6, '#');
        hash_grid.set_pos(5, 7, '#');
        hash_grid.set_pos(7, 7, '#');

        let expected = r"#..
.#.
#.#";
        assert_eq!(format!("{}", hash_grid), expected);
    }

    #[test]
    fn test_fromstr() {
        let grid = r"#..
.#.
#.#";
        let grid: HashGrid<char> = grid.parse().unwrap();
        assert_eq!(grid.get_pos(0, 0), Some(&'#'));
        assert_eq!(grid.get_pos(1, 0), None);
        assert_eq!(grid.get_pos(2, 2), Some(&'#'));
        assert_eq!(grid.deref().len(), 4);
    }
}
