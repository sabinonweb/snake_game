use std::collections::HashSet;

use crate::{
    data::{GRID_DIMENSION, GRID_SIZE, SCREEN_SIZE},
    grid::Grid,
};
use ggez::{graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect}, *};

pub struct Food {
    pub color: Color,
    pub position: Grid,
}

impl Food {
    pub fn new(color: Color, food_pos: Grid) -> Food {
        Food { position: food_pos, color: color }
    }

    pub fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult { 
        self.position.draw_rect(ctx, canvas, self.color);

        Ok(())
    }
    
    pub fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        self.position.x  += 1;

        Ok(())
    }
}
