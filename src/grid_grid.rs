use std::fmt::Display;

use super::grid_pos::GridPos;

pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    /// Creates a new grid with width and height
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        let size = width * height;
        let mut data = Vec::with_capacity(size);
        data.resize_with(size, || Default::default());

        Grid { data, width }
    }

    /// Gets Neighbors (or None) of the specified position.
    /// First Neighbor is UP followed by the other 3 in a clockwise order
    /// # Example
    /// ```
    /// # use grid::grid::{Grid, GridPos};
    /// let slices: &[&[i32]] = &[
    ///     &[0, 1, 2],
    ///     &[3, 4, 5],
    ///     &[6, 7, 8]];
    /// let grid = Grid::from(slices);
    /// # let pos_of_0 = GridPos::new(0);
    /// # let pos_of_1 = GridPos::new(1);
    /// # let pos_of_3 = GridPos::new(3);
    /// # let pos_of_4 = GridPos::new(4);
    /// # let pos_of_5 = GridPos::new(5);
    /// # let pos_of_6 = GridPos::new(6);
    /// # let pos_of_7 = GridPos::new(7);
    /// # let pos_of_8 = GridPos::new(8);
    /// assert_eq!(
    ///     grid.get_neighbors(&pos_of_0),
    ///     [None, Some(pos_of_1), Some(pos_of_3), None]);
    /// assert_eq!(
    ///     grid.get_neighbors(&pos_of_4),
    ///     [Some(pos_of_1), Some(pos_of_5), Some(pos_of_7), Some(pos_of_3)]);
    /// assert_eq!(
    ///     grid.get_neighbors(&pos_of_7),
    ///     [Some(pos_of_4), Some(pos_of_8), None, Some(pos_of_6)]);
    /// ```
    pub fn get_neighbors(&self, position: &GridPos) -> [Option<GridPos>; 4] {
        let index = position.pos;

        let pos_in_row = index % self.width;

        let mut neighbors: [Option<GridPos>; 4] = Default::default();
        // Up
        if index >= self.width {
            neighbors[0] = Some(GridPos::new(index - self.width));
        }
        // Right
        if pos_in_row + 1 < self.width {
            neighbors[1] = Some(GridPos::new(index + 1));
        }
        // Down
        if index < self.size() - self.width {
            neighbors[2] = Some(GridPos::new(index + self.width));
        }
        // Left
        if pos_in_row > 0 {
            neighbors[3] = Some(GridPos::new(index - 1));
        }
        neighbors
    }

    /// Gets Neighbors (all Some(...)) of the specified position
    ///
    /// Calls get_neighbors(position).into_iter().flatten().collect::<Vec<GridPos>>();
    ///
    /// # Example
    /// ```
    /// # use grid::grid::{Grid, GridPos};
    /// let slices: &[&[i32]] = &[
    ///     &[0, 1, 2],
    ///     &[3, 4, 5],
    ///     &[6, 7, 8]];
    /// let grid = Grid::from(slices);
    /// # let pos_of_0 = GridPos::new(0);
    /// # let pos_of_1 = GridPos::new(1);
    /// # let pos_of_3 = GridPos::new(3);
    /// # let pos_of_4 = GridPos::new(4);
    /// # let pos_of_5 = GridPos::new(5);
    /// # let pos_of_6 = GridPos::new(6);
    /// # let pos_of_7 = GridPos::new(7);
    /// # let pos_of_8 = GridPos::new(8);
    ///
    /// assert_eq!(
    ///     grid.get_neighbors_flat(&pos_of_8),
    ///     vec![pos_of_5, pos_of_7]);
    /// ```
    pub fn get_neighbors_flat(&self, position: &GridPos) -> Vec<GridPos> {
        self.get_neighbors(position)
            .into_iter()
            .flatten()
            .collect::<Vec<GridPos>>()
    }

    /// Places a new value at the specified grid position
    pub fn put(&mut self, pos: &GridPos, new_value: T) {
        if let Some(old_value) = self.get_mut(pos) {
            let _new_value = std::mem::replace(old_value, new_value);
        }
    }

    /// Get a reference to the value at the specified position
    pub fn get(&self, pos: &GridPos) -> Option<&T> {
        if pos.pos < self.size() {
            Some(&self.data[pos.pos])
        } else {
            None
        }
    }

    /// Gets a mutable reference to the value at the specified position
    pub fn get_mut(&mut self, pos: &GridPos) -> Option<&mut T> {
        if pos.pos < self.size() {
            Some(&mut self.data[pos.pos])
        } else {
            None
        }
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator {
            grid: self,
            index: 0,
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    index: usize,
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.grid.get(&GridPos::new(self.index))
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    let seperator = if index > 0 && index % self.width == 0 {
                        ",\n"
                    } else {
                        ", "
                    };
                    format!("{item}{seperator}")
                })
                .fold(String::from(""), |acc, next| format!("{acc}{next}"))
        )
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    /// Creates a grid from the provided data.
    /// # Panics
    /// Panics, when not all data-rows have to same width
    fn from(data: Vec<Vec<T>>) -> Self {
        let all_widths: Vec<usize> = data.iter().map(|slice| slice.len()).collect();
        let first_width = &all_widths[0];
        let all_widths_same = all_widths.iter().all(|width| *width == *first_width);
        if !all_widths_same {
            panic!("Grid malformed! Not all rows have the same width");
        }

        let grid = data.into_iter().flatten().collect();

        Grid {
            data: grid,
            width: *first_width,
        }
    }
}

impl<T> From<&[&[T]]> for Grid<T>
where
    T: Clone,
{
    /// Creates a grid from the provided data.
    /// # Panics
    /// Panics, when not all data-rows have to same width
    fn from(data: &[&[T]]) -> Self {
        let all_widths: Vec<usize> = data.iter().map(|slice| slice.len()).collect();
        let first_width = &all_widths[0];
        let all_widths_same = all_widths.iter().all(|width| *width == *first_width);
        if !all_widths_same {
            panic!("Malformed grid! Not all rows have to same width!");
        }

        let data: Vec<T> = data.iter().flat_map(|slice| slice.to_vec()).collect();
        Grid {
            data,
            width: *first_width,
        }
    }
}
