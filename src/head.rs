use ggez::{
    event::EventHandler, graphics::{Canvas, Color, DrawMode, Mesh, Rect}, input::keyboard, *
};
use crate::{
    data::{GRID_DIMENSION, SCREEN_SIZE},
    direction::Direction,
    Vec2,
};
use rand::{thread_rng, Rng};

pub struct GridPosition {
    pub x: f32,
    pub y: f32,
}

impl GridPosition {
    pub fn new(x: f32, y: f32) -> GameResult<GridPosition> {
        Ok(GridPosition {
            x,
            y,

        })
    }
}

pub struct Head {
    position: GridPosition,
    head: graphics::Mesh,
}

impl Head {
    pub fn new(ctx : &mut Context) -> GameResult<Head> {
        let (x, y) = Head::random_pos();

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

        Ok(Head {
            position: GridPosition::new(x, y)?,
            head: rectangle
        })
    }

    pub fn random_pos() -> (f32, f32) {
        let mut random = thread_rng();
        let x = random.gen_range(0..SCREEN_SIZE.0 as i32);
        let y = random.gen_range(0..SCREEN_SIZE.1 as i32);

        (x as f32, y as f32)
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
      
        canvas.draw(&self.head, Vec2::new(self.position.x, 300.0));
        
        
        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.position.x += 8.0;

        Ok(())
    }
}


 