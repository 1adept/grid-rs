#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GridPos {
    pub(super) pos: usize,
}

impl GridPos {
    pub fn new(pos: usize) -> Self {
        Self { pos }
    }
}
