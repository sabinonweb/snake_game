use ggez::{
    graphics::{Canvas, Color},
    *,
};
use crate::{
    data::{GRID_DIMENSION, SCREEN_SIZE},
    grid::Grid,
};
use super::GameMode;

pub struct Background;

impl Background {
    pub fn draw(canvas: &mut Canvas, ctx: &mut Context, game_mode: &GameMode) -> GameResult {
        for i in (0..SCREEN_SIZE.0 as i32 + 32).step_by(32) {
            for j in (0..SCREEN_SIZE.1 as i32 + 32).step_by(32) {
                let background = Grid::new(i, j);
                let color_even: Color = [0.25, 0.95, 0.35, 1.0].into();
                let color_odd: Color = [0.68, 0.74, 0.62, 1.0].into();
                let color: Color = [0.14, 0.3, 0.15, 1.0].into();
                if *game_mode == GameMode::Screen {
                    if ((i + j) / GRID_DIMENSION.0 as i32) % 2 == 0 {
                        background.draw_rect(ctx, canvas, color_even)?; 
                    }
                    else {
                        background.draw_rect(ctx, canvas, color_odd)?;
                    }
                } else {
                    background.draw_rect(ctx, canvas, color)?;
                }
            }
        }
         
        Ok(())
    }
}


