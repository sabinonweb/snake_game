use std::collections::HashSet;

use crate::{
    data::{GRID_DIMENSION, GRID_SIZE, SCREEN_SIZE},
    head::GridPosition,
    random::random,
};
use ggez::{graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect}, *};

pub struct Food {
    position: GridPosition,
    food: graphics::Mesh,
}

impl Food {
    pub fn new(ctx: &mut Context) -> Food {
        let (x, y) = random();

        let food = Mesh::new_rectangle(
            ctx, 
            DrawMode::fill(),
            Rect {
                x,
                y,
                w: GRID_DIMENSION.0,
                h: GRID_DIMENSION.1,
            },
            Color::RED,
        ).unwrap();

        Food { position: GridPosition::new(x, y).unwrap(), food }
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let mut rng = rand::thread_rng();

        canvas.draw(
            &self.food, 
            DrawParam::new(),
        );

        Ok(())
    }
    
    pub fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        self.position.x  += 0.7;

        Ok(())
    }
}
