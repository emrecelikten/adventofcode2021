use crate::common::error::CommonError;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub trait DenseGrid<V: Default> {
    fn get_pos(&self, x: usize, y: usize) -> &V;
    fn get_pos_mut(&mut self, x: usize, y: usize) -> &mut V;
    fn set_pos(&mut self, x: usize, y: usize, value: V);
    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)>;
    fn get_neighbours_cross(&self, x: usize, y: usize) -> Vec<(usize, usize)>;
    fn vstack_mut(&mut self, other: Self) -> Result<(), CommonError>;
    fn hstack_mut(&mut self, other: Self) -> Result<(), CommonError>;
}

#[derive(Clone, PartialEq, Debug)]
pub struct ArrayGrid<V> {
    pub x_size: usize,
    pub y_size: usize,
    pub underlying: Vec<V>,
}

impl<V: Default + Clone> ArrayGrid<V> {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        ArrayGrid {
            x_size,
            y_size,
            underlying: vec![V::default(); x_size * y_size],
        }
    }

    pub fn new_with_default(x_size: usize, y_size: usize, v: &V) -> Self {
        ArrayGrid {
            x_size,
            y_size,
            underlying: vec![v.clone(); x_size * y_size],
        }
    }
}

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

    // These should normally return a impl Iterator, but doing it in trait methods seems to be
    // unstable unfortunately.
    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        _get_neighbours_array(self, x, y, &crate::common::collections::NEIGHBOURS_2D)
    }

    fn get_neighbours_cross(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        _get_neighbours_array(self, x, y, &crate::common::collections::NEIGHBOURS_CROSS_2D)
    }

    fn vstack_mut(&mut self, other: Self) -> Result<(), CommonError> {
        if other.x_size != self.x_size {
            Err(CommonError::Dimensions(format!(
                "Grid dimensions do not match! Self: {}, other: {}",
                self.x_size, other.x_size
            )))
        } else {
            self.underlying.extend(other.underlying);
            self.y_size += other.y_size;
            Ok(())
        }
    }

    fn hstack_mut(&mut self, other: Self) -> Result<(), CommonError> {
        if other.y_size != self.y_size {
            Err(CommonError::Dimensions(format!(
                "Grid dimensions do not match! Self: {}, other: {}",
                self.y_size, other.y_size
            )))
        } else {
            let mut new_vec = vec![V::default(); self.underlying.len() + other.underlying.len()];
            let new_x_size = self.x_size + other.x_size;
            for y in 0..self.y_size {
                let offset_from_self = y * new_x_size;
                let range_from_self = offset_from_self..offset_from_self + self.x_size;
                let self_range = (y * self.x_size)..((y + 1) * self.x_size);
                new_vec[range_from_self].clone_from_slice(&self.underlying[self_range]);

                let offset_from_other = y * new_x_size + self.x_size;
                let range_from_other = offset_from_other..offset_from_other + other.x_size;
                let other_range = (y * other.x_size)..((y + 1) * other.x_size);
                new_vec[range_from_other].clone_from_slice(&other.underlying[other_range]);
            }
            self.x_size += other.x_size;
            self.underlying = new_vec;
            Ok(())
        }
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
                underlying: result,
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
                .collect::<Vec<i32>>(),
        })
    }
}

/// Best effort
impl Display for ArrayGrid<i32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity(self.underlying.len() + self.y_size);
        for y in 0..self.y_size {
            let cur_line = &self.underlying[self.x_size * y..self.x_size * (y + 1)];
            for &int in cur_line {
                s.push_str(&int.to_string());
                s.push('\t');
            }

            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_arraygrid_vstack() {
        let data1 = r"1234567
2345678";
        let err = r"123456
123456";

        let mut array_grid: ArrayGrid<char> = data1.parse().unwrap();
        let err_grid: ArrayGrid<char> = err.parse().unwrap();

        assert!(array_grid.vstack_mut(array_grid.clone()).is_ok());
        for y in 0..array_grid.y_size / 2 {
            for x in 0..array_grid.x_size {
                assert_eq!(
                    array_grid.get_pos(x, y + array_grid.y_size / 2),
                    array_grid.get_pos(x, y)
                );
            }
        }

        assert!(array_grid.vstack_mut(err_grid).is_err());
    }
    #[test]
    fn test_arraygrid_hstack() {
        let data1 = r"123
456
789";
        let data2 = r"12
34
56";
        let err = r"1234
1234";

        let data1_grid: ArrayGrid<char> = data1.parse().unwrap();
        let data2_grid: ArrayGrid<char> = data2.parse().unwrap();
        let err_grid: ArrayGrid<char> = err.parse().unwrap();

        let mut result = data1_grid.clone();
        assert!(result.hstack_mut(data2_grid.clone()).is_ok());

        for y in 0..data1_grid.y_size {
            for x in 0..data1_grid.x_size {
                assert_eq!(result.get_pos(x, y), data1_grid.get_pos(x, y));
            }
        }

        for y in 0..data2_grid.y_size {
            for x in 0..data2_grid.x_size {
                assert_eq!(
                    result.get_pos(x + data1_grid.x_size, y),
                    data2_grid.get_pos(x, y)
                );
            }
        }

        assert!(result.hstack_mut(err_grid).is_err());
    }
}
