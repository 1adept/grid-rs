/// Create a grid
///
/// # Example
///
/// ```
/// # use grid::*;
/// let grid = grid!(
///     [0, 0, 1, 0, 0,],
///     [0, 0, 1, 0, 0,],
///     [3, 2, 1, 0, 0,],
///     [0, 1, 0, 0, 4,],
///     [0, 1, 1, 1, 4,]
/// );
/// assert_eq!(grid.size(), (5 * 5));
/// assert_eq!(grid.width(), 5);
/// assert_eq!(grid.get(&grid.pos_at(2, 2).unwrap()), Some(&1));
/// assert_eq!(grid.get(&grid.pos_at(3, 4).unwrap()), Some(&4));
/// ```
#[macro_export]
macro_rules! grid {
    ( $( $x:expr ),* ) => {
        {
            let mut rows = Vec::new();
            $(
                rows.push( Vec::from( $x ) );
            )*
            let width = rows.get(0).map(Vec::len).or(Some(0)).unwrap();
            let data = rows.into_iter().flatten().collect();
            Grid::new(width, data)
        }
    };
}
