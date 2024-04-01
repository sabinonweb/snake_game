use crate::{
    data::GRID_DIMENSION,
};
use ggez::{graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect}, input::keyboard::KeyCode, *};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Grid {
    pub x: i32,
    pub y: i32,
}

impl Grid {
    pub fn new(x: i32, y: i32) -> Grid {
        Grid {
            x, y
        }
    }

    pub fn draw_rect(&self, ctx: &mut Context, canvas: &mut Canvas, color: Color) -> GameResult {
        let rect = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect {
                x: self.x as f32,
                y: self.y as f32,
                w: GRID_DIMENSION.0,
                h: GRID_DIMENSION.1,
            },
            color,
        )?;

        canvas.draw(&rect, DrawParam::default());

        Ok(())
    }
}

impl From<(i32, i32)> for Grid {
    fn from(pos: (i32, i32)) -> Self {
        Grid {x: pos.0, y: pos.1}
    }
}

impl From<Grid> for (i32, i32) {
    fn from(grid: Grid) -> Self {
        (grid.x, grid.y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down, 
    Left, 
    Right,
    None
}

impl Direction {
    pub fn inverse(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            _ => Direction::None,
        }
    }

    pub fn from_keyword(key: KeyCode) -> Direction {
        match key {
            KeyCode::W => Direction::Up,
            KeyCode::A => Direction::Left,
            KeyCode::S => Direction::Down,
            KeyCode::D => Direction::Right,
            _ => Direction::None,
        }
    }
}


