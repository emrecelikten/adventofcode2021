use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

pub trait SparseGrid<V: Clone> {
    fn get_pos(&self, x: usize, y: usize) -> Option<&V>;
    fn get_pos_mut(&mut self, x: usize, y: usize) -> Option<&mut V>;
    fn get_or_insert_pos_mut(&mut self, x: usize, y: usize, value: &V) -> &mut V;
    fn set_pos(&mut self, x: usize, y: usize, value: V);
}

type HMap<V> = std::collections::HashMap<(usize, usize), V>;
pub struct HashGrid<V>(HMap<V>);

impl<V: Clone> HashGrid<V> {
    pub fn new() -> Self {
        HashGrid {
            0: Default::default(),
        }
    }
}

impl<V: Clone> Default for HashGrid<V> {
    fn default() -> Self {
        Self::new()
    }
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
}

impl<V> Deref for HashGrid<V> {
    type Target = HMap<V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V> DerefMut for HashGrid<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
}
