use ggez::{
    context::Context, graphics::{Canvas, DrawMode, DrawParam, Mesh, Rect}, GameResult
};

use crate::data::{GRID_SIZE, SCREEN_SIZE};

pub struct Menu {
    pub x: u32,
    pub y: u32,
}

impl Menu {
    pub fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        let color = [0.95, 0.56, 0.53, 1.0].into();
        let rect = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect{
                x: SCREEN_SIZE.0 / 2.0,
                y: SCREEN_SIZE.1 / 2.0,
                w: GRID_SIZE.0 * 10.0,
                h: GRID_SIZE.1 * 3.0,
            },
           color, 
        );

        canvas.draw(rect, DrawParam::default());
        Ok(())
    }   
}

