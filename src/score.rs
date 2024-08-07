use ggez::{
    context::Context, graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, PxScale, Rect, Text, TextFragment}, GameResult, glam,
};

use crate::data::{GRID_SIZE, SCREEN_SIZE};

pub struct Score {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub score: u32,
}

impl Score {
    pub fn new() -> Score {
        Score {
            x: SCREEN_SIZE.0 / 2.8,
            y: SCREEN_SIZE.1 / 3.0,
            w: GRID_SIZE.0 * 10.0,
            h: GRID_SIZE.1 * 3.0,
            score: 0,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        let color = [0.24, 0.45, 0.25, 1.0].into();
        let rect = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect{
                x: self.x,
                y: self.y,
                w: self.w,
                h: self.h,
            },
           color, 
        )?;

        let score = format!("Score: {}", self.score);

        let text = Text::new(TextFragment {
            text: score,
            color: Some(Color::new(0.67, 0.75, 0.82, 1.0)),
            font: Some("LiberationMono-Regular".into()),
            scale: Some(PxScale::from(50.0)),
            ..Default::default()
        });
        
        canvas.draw(&rect, DrawParam::default());
        canvas.draw(&text, DrawParam::default().dest(glam::vec2(SCREEN_SIZE.0 / 2.5, SCREEN_SIZE.1 / 2.9)));

        Ok(())
    }  

    pub fn x_coord(&self) -> (f32, f32) {
        (self.x, self.x + self.w)
    }

    pub fn y_coord(&self) -> (f32, f32) {
        (self.y, self.y + self.h)
    }
}
