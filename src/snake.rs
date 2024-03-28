use crate::{
    data::{FPS, GRID_DIMENSION},
    grid::Grid
};
use ggez::{
    *,
    graphics::Canvas,
    input::keyboard::KeyCode,
};
use std::collections::VecDeque;

pub struct Snake {
    pub head: Grid,
    pub body: VecDeque<Grid>,
}

impl Snake {
    pub fn new(pos: Grid) -> Snake {
        let mut body: VecDeque<Grid> = VecDeque::new();
        body.push_back(Grid::new(pos.x - GRID_DIMENSION.0 as i32, pos.y));
        body.push_back(Grid::new(pos.x, pos.y));
        body.push_back(Grid::new(pos.x - GRID_DIMENSION.0 as i32, pos.y));
        body.push_back(Grid::new(pos.x - (GRID_DIMENSION.0 - 32.0) as i32, pos.y));
        body.push_back(Grid::new(pos.x - (GRID_DIMENSION.0 - 64.0) as i32, pos.y));
        body.push_back(Grid::new(pos.x - (GRID_DIMENSION.0 - 96.0) as i32, pos.y));

        Snake {
            head: Grid::new(pos.x, pos.y),
            body,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let color_head = [0.0, 0.0, 0.0, 1.0].into();
        let color_body = [0.73, 0.77, 0.4, 1.0].into();

        self.head.draw_rect(ctx, canvas, color_head);

        for segment in &self.body {
            segment.draw_rect(ctx, canvas, color_body);
        }   

        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        let k_ctx = &ctx.keyboard;

        if k_ctx.is_key_pressed(KeyCode::Right) {
            self.head.x += FPS;
        } else if k_ctx.is_key_pressed(KeyCode::Left) {
            self.head.x -= FPS;
        } else if k_ctx.is_key_pressed(KeyCode::Up) {
            self.head.y += FPS;
        } else if k_ctx.is_key_pressed(KeyCode::Down) {
            self.head.y -= FPS;
        }

        Ok(())
    }
}

