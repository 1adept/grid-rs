use std::fmt::Display;

use super::grid_pos::GridPos;

#[must_use]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, data: Vec<T>) -> Self {
        Grid { data, width }
    }

    /// Creates a new grid with width and height
    pub fn new_empty(width: usize, height: usize) -> Self
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
    /// # use grid::{Grid, GridPos};
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
    #[must_use]
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
    /// Calls `get_neighbors(position).into_iter().flatten().collect::<Vec<GridPos>>();`
    ///
    /// # Example
    /// ```
    /// # use grid::{Grid, GridPos};
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
    #[must_use]
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

    /// Gets `GridPos` at 0-indexed grid
    ///
    /// # Example
    ///
    /// ```
    /// # use grid::Grid;
    /// # use grid::GridPos;
    /// /*
    ///     1,2,3,
    ///     4,5,6,
    ///  */
    /// let grid = Grid::new(3, vec![1,2,3,4,5,6]);
    /// let pos_1 = GridPos::new(0);
    /// let pos_4 = GridPos::new(3);
    /// let pos_3 = GridPos::new(2);
    /// let pos_6 = GridPos::new(5);
    /// assert_eq!(grid.pos_at(5, 5), None);
    /// assert_eq!(grid.pos_at(0, 0), Some(pos_1));
    /// assert_eq!(grid.pos_at(1, 0), Some(pos_4));
    /// assert_eq!(grid.pos_at(0, 2), Some(pos_3));
    /// assert_eq!(grid.pos_at(1, 2), Some(pos_6));
    /// ```
    #[must_use]
    pub fn pos_at(&self, row: usize, col: usize) -> Option<GridPos> {
        if col >= self.width {
            return None;
        }
        let height = self.size() / self.width;
        if row > height {
            return None;
        }

        let pos = (self.width * row) + col;
        if pos < self.size() {
            Some(GridPos::new(pos))
        } else {
            None
        }
    }

    /// Get a reference offset by row, col
    ///
    /// # Example
    ///
    /// ```
    /// # use grid::*;
    /// /**
    ///     1,2,3,
    ///     4,5,6,
    ///     7,8,9,
    ///  */
    /// let grid = Grid::new(3, vec![1,2,3,4,5,6,7,8,9]);
    /// let pos_0_0 = grid.pos_at(0, 0).unwrap();
    /// let pos_1_1 = grid.pos_at(1, 1).unwrap();
    /// assert_eq!(grid.get(&pos_1_1), grid.get_at_offset(&pos_0_0, 1, 1));
    /// assert_eq!(grid.get_at_offset(&grid.pos_at(1, 2).unwrap(), 0, 1), None);
    /// assert_eq!(grid.get_at_offset(&grid.pos_at(1, 2).unwrap(), 1, 0), Some(&9));
    /// ```
    #[must_use]
    pub fn get_at_offset(
        &self,
        at_position: &GridPos,
        row_offset: i8,
        col_offset: i8,
    ) -> Option<&T> {
        let col = col_offset + (at_position.pos % self.width) as i8;
        let row = row_offset + (at_position.pos / self.width) as i8;
        if row < 0 || col < 0 {
            return None
        }
        let col = col as usize;
        let row = row as usize;
        let pos = self.pos_at(row, col);
        if let Some(pos) = pos {
            self.get(&pos)
        } else {
            None
        }
    }

    /// Get a reference to the value at the specified position
    #[must_use]
    pub fn get(&self, pos: &GridPos) -> Option<&T> {
        if pos.pos < self.size() {
            Some(&self.data[pos.pos])
        } else {
            None
        }
    }

    /// Gets a mutable reference to the value at the specified position
    #[must_use]
    pub fn get_mut(&mut self, pos: &GridPos) -> Option<&mut T> {
        if pos.pos < self.size() {
            Some(&mut self.data[pos.pos])
        } else {
            None
        }
    }

    #[must_use]
    pub fn iter(&self) -> GridIterator<T> {
        GridIterator {
            grid: self,
            index: 0,
        }
    }

    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub fn size(&self) -> usize {
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
        let next = self.grid.get(&GridPos::new(self.index));
        self.index += 1;
        next
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
                    let seperator = if index % self.width == self.width - 1 {
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
        let all_widths: Vec<usize> = data.iter().map(std::vec::Vec::len).collect();
        let first_width = &all_widths[0];
        let all_widths_same = all_widths.iter().all(|width| *width == *first_width);
        
        assert!(all_widths_same, "Grid malformed! Not all rows have the same width");

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
        
        assert!(all_widths_same, "Malformed grid! Not all rows have to same width!");

        let data: Vec<T> = data.iter().flat_map(|slice| slice.to_vec()).collect();
        Grid {
            data,
            width: *first_width,
        }
    }
}
