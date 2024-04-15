use ggez::{
    context::Context, graphics::{self, Canvas, Color, DrawMode, DrawParam, Mesh, PxScale, Rect, Text, TextFragment}, GameResult, glam,
};

use crate::data::{GRID_SIZE, SCREEN_SIZE};

pub struct Menu {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            x: SCREEN_SIZE.0 / 2.8,
            y: SCREEN_SIZE.1 / 3.0,
            w: GRID_SIZE.0 * 10.0,
            h: GRID_SIZE.1 * 3.0,
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

        let text = Text::new(TextFragment {
            text: "New Game".to_string(),
            color: Some(Color::new(0.67, 0.75, 0.82, 1.0)),
            font: Some("LiberationMono-Regular".into()),
            scale: Some(PxScale::from(50.0)),
            ..Default::default()
        });

        let game_title = Text::new(TextFragment {
            text: "ggez_snake".to_string(),
            color: Some(Color::new(0.67, 0.75, 0.82, 1.0)),
            font: Some("LiberationMono-Regular".into()),
            scale: Some(PxScale::from(50.0)),
            ..Default::default()
        });

        let game_text = Text::new(TextFragment {
            text: "\nOnly place where your\n snake gets bigger".to_string(),
            color: Some(Color::new(0.67, 0.75, 0.82, 1.0)),
            font: Some("LiberationMono-Regular".into()),
            scale: Some(PxScale::from(35.0)),
            ..Default::default()
        });
        
        let play_text = Text::new(TextFragment {
            text: "(SPACE): play\n(SHIFT/CTRL + Q): quit".to_string(),
            color: Some(Color::new(0.67, 0.75, 0.82, 1.0)),
            font: Some("LiberationMono-Regular".into()),
            scale: Some(PxScale::from(35.0)),
            ..Default::default()
        });

        canvas.draw(&rect, DrawParam::default());
        canvas.draw(&text, DrawParam::default().dest(glam::vec2(SCREEN_SIZE.0 / 2.5, SCREEN_SIZE.1 / 2.9)));
        canvas.draw(&game_title, DrawParam::default().dest(glam::vec2(SCREEN_SIZE.0 / 2.8, SCREEN_SIZE.1 / 23.0)));
        canvas.draw(&game_text, DrawParam::default().dest(glam::vec2(SCREEN_SIZE.0 / 3.0, SCREEN_SIZE.1 / 8.5)));
        canvas.draw(&play_text, DrawParam::default().dest(glam::vec2(SCREEN_SIZE.0 / 2.8, SCREEN_SIZE.1 / 2.0)));

        Ok(())
    }  

    pub fn x_coord(&self) -> (f32, f32) {
        (self.x, self.x + self.w)
    }

    pub fn y_coord(&self) -> (f32, f32) {
        (self.y, self.y + self.h)
    }
}
