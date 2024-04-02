use ggez::{
    graphics::{Canvas, Color},
    *,
};
use crate::{
    data::{GRID_DIMENSION, GRID_SIZE, SCREEN_SIZE},
    grid::Grid,
};
use std::collections::HashSet;

pub struct Background;

impl Background {
    pub fn draw(canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
         for i in (0..SCREEN_SIZE.0 as i32 + 32).step_by(32) {
            for j in (0..SCREEN_SIZE.1 as i32 + 32).step_by(32) {
                let background = Grid::new(i, j);
                let color_even: Color = [0.52, 0.88, 0.65, 1.0].into();
                let color_odd: Color = [0.0, 1.0, 0.47, 1.0].into();
                if ((i + j) / GRID_DIMENSION.0 as i32) % 2 == 0 {
                    background.draw_rect(ctx, canvas, color_even)?; 
                }
                else {
                    background.draw_rect(ctx, canvas, color_odd)?;
                }
            }
        }

        Ok(())
    }
}


