use food::Food;
use ggez::{
    event::EventHandler, glam::Vec2, graphics::{Canvas, Color, DrawMode, Rect}, winit::event_loop, *
};
use crate::bit::Bit;
use crate::{data::SCREEN_SIZE, background::Grid, random::random};

mod data;
mod bit;
mod direction;
mod food;
mod background;
mod random;

pub struct GameState {
    food: Food,
    head: Bit,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameState {
        GameState {
            food: Food::new(ctx),
            head: Bit::new(ctx).unwrap(),
        }
    }
}

impl EventHandler for GameState {
    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = 
            Canvas::from_frame(ctx,  Color::BLACK);

        Grid::draw(&mut canvas, ctx)?;

        self.food.draw(ctx, &mut canvas)?;
        self.head.draw(ctx, &mut canvas)?;

        canvas.set_screen_coordinates(Rect {
            x: 50.0,
            y: 50.0,
            w: 100.0,
            h: 100.0,
        });

        canvas.finish(ctx)?;

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.food.update(ctx)?;
        self.head.update(ctx)?;

        Ok(())
    }
}


pub fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new(
        "snake_game", "sabinonweb"
    )
    .window_setup(conf::WindowSetup::default().title("snake_game"))
    .window_mode(conf::WindowMode::default().transparent(true).maximized(false).dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
    .build()?;
    let state = GameState::new(&mut ctx);
    event::run(ctx, event_loop, state)
} 
