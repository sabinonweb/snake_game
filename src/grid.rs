use crate::{
    data::GRID_DIMENSION,
    GameState,
};
use ggez::{graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect}, *};

#[derive(Clone, Copy)]
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

pub enum Direction {
    Up,
    Down, 
    Left, 
    Right,
    None,
}

impl Direction {
    pub fn new() -> Direction {
        Direction::Right
    } 
}


