use ggez::{
    event::EventHandler, graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect}, input::keyboard, *
};
use crate::{
    data::{FPS, GRID_DIMENSION, SCREEN_SIZE}, direction::Direction, random::random, Vec2
};
use rand::{thread_rng, Rng};

#[derive(PartialEq, Eq)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> GameResult<GridPosition> {
        Ok(GridPosition {
            x,
            y,

        })
    }
}

pub struct Bit {
    position: GridPosition,
    bit: graphics::Mesh,
}

impl Bit {
    pub fn new(ctx : &mut Context) -> GameResult<Bit> {
        let (x, y) = random();

        let rectangle = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect {
                x,
                y,
                w: GRID_DIMENSION.0,
                h: GRID_DIMENSION.1,
            },
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            }
        )?;

        Ok(Bit {
            position: GridPosition::new(x as i32, y as i32)?,
            bit: rectangle
        })
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
      
        canvas.draw(&self.bit, DrawParam::new());
        
        
        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.position.x += FPS;

        Ok(())
    }
}


 
