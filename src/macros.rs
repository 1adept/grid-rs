#[macro_export]
macro_rules! grid {
    ( $( $x:expr ),* ) => {
        {
            let mut row = Vec::new();
            $(
                row.push( Vec::from( $x ) );
            )*
            Grid::from(row)
        }
    };
}