pub mod dense_grid;
pub mod sparse_grid;

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
