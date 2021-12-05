use std::collections::HashMap;

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
        self.entry((x, y)).or_insert(V::default())
    }

    fn set_pos(&mut self, x: usize, y: usize, value: V) {
        self.insert((x, y), value);
    }
}

pub struct ArrayGrid<V> {
    x_size: usize,
    y_size: usize,
    underlying: Box<[V]>,
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

impl<V: Default> DenseGrid<V> for ArrayGrid<V> {
    fn get_pos(&self, x: usize, y: usize) -> &V {
        &self.underlying[y * self.x_size + x]
    }

    fn get_pos_mut(&mut self, x: usize, y: usize) -> &mut V {
        &mut self.underlying[y * self.x_size + x]
    }

    fn set_pos(&mut self, x: usize, y: usize, value: V) {
        self.underlying[y * self.x_size + x] = value;
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
}
