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