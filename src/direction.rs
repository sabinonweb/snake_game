use ggez::{
    input::keyboard::KeyCode,
    *,
};

pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn movement(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Right => Some(Direction::Right),
            KeyCode::Left => Some(Direction::Left),
            _ => None,   
        }
    }
}