use ggez::{graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect}, *};
use crate::data::{GRID_DIMENSION, GRID_SIZE, SCREEN_SIZE};

pub struct Background {
    x: f32,
    y: f32,
}

impl Background {
    fn new(x: f32, y: f32) -> Background {
        Background {
            x,
            y,
        }
    }

    pub fn draw_rect(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let rect = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect {
                x: self.x,
                y: self.y,
                w: GRID_SIZE.0,
                h: GRID_SIZE.1,
            },
            Color::GREEN,
        )?;

        canvas.draw(&rect, DrawParam::default());

        Ok(())
    }

    pub fn draw(canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        for i in (0..SCREEN_SIZE.0 as i32).step_by(GRID_SIZE.0 as usize) {
            for j in (0..SCREEN_SIZE.1 as i32).step_by(GRID_SIZE.1 as usize) {
                let background = Background::new(i as f32, j as f32);
                background.draw_rect(ctx, canvas)?;
                println!("i = {} j = {}", i, j)
            }
        }

        Ok(())
    }
}