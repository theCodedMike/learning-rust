#[derive(Debug)]
pub enum Direction {
    NORTH = 0,
    SOUTH,
    EAST,
    WEST,
}

impl From<u32> for Direction {
    fn from(value: u32) -> Self {
        match value {
            0 => Direction::NORTH,
            1 => Direction::SOUTH,
            2 => Direction::EAST,
            _ => Direction::WEST,
        }
    }
}
