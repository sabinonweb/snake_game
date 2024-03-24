use crate::{
    data::{GRID_DIMENSION, GRID_SIZE, SCREEN_SIZE},
    head::GridPosition
};
use ggez::{event::EventHandler, graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect}, *};
use rand::Rng;

pub struct Food {
    position: GridPosition,
    food: graphics::Mesh,
}

impl Food {
    pub fn new(ctx: &mut Context) -> Food {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0.0..GRID_SIZE.0);
        let y = rng.gen_range(0.0..GRID_SIZE.1);

        println!("Screen Size: {:?}", SCREEN_SIZE);

        println!("x = {}, y = {}", x, y);

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
