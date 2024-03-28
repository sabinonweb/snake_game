use ggez::{
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect},
    *,
};
use crate::{
    data::{GRID_SIZE, SCREEN_SIZE},
    grid::Grid,
};

pub struct Background;

impl Background {
    pub fn draw(canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        for i in (0..SCREEN_SIZE.0 as i32).step_by(GRID_SIZE.0 as usize) {
            for j in (0..SCREEN_SIZE.1 as i32).step_by(GRID_SIZE.1 as usize) {
                let background = Grid::new(i, j);
                let color: Color = [0.01, 0.18, 0.06, 1.0].into();
                background.draw_rect(ctx, canvas, color)?;            }
        }

        Ok(())
    }
}
