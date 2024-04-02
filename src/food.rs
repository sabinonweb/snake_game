use std::collections::HashSet;

use crate::{
    data::{GRID_DIMENSION, GRID_SIZE, SCREEN_SIZE},
    grid::Grid, random::random,
};
use ggez::{graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect}, *};

pub struct Food {
    pub color: Color,
    pub position: Grid,
}

impl Food {
    pub fn new(color: Color) -> Food {
        let position = Food::food_pos();
        Food { position, color: color }
    }

    pub fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        self.position.draw_rect(ctx, canvas, self.color);

        Ok(())
    }
    
    pub fn update(&mut self) -> Result<(), GameError> {
        self.position.x  += 1;

        Ok(())
    }

    pub fn food_pos() -> Grid {
        let (x, y) = random();
        Grid::new(x, y)
    }
}
