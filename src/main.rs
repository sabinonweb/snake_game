use crate::{
    background::Background,
    data::{FPS, GRID_SIZE, SCREEN_SIZE, SNAKE_POS},
    food::Food,
    grid::{Direction, Grid},
    random::random,
    snake::Snake,
};
use ggez::{
    event::EventHandler, 
    glam::Vec2, 
    graphics::{Canvas, Color, DrawMode, Rect}, 
    input::keyboard::{KeyCode, KeyMods, KeyInput},
    *
};

mod background;
mod data;
mod food;
mod grid;
mod random;
mod snake;

pub struct GameState {
    food: Food,
    snake: Snake,
    direction: Direction,

}

impl GameState {
    fn new(ctx: &mut Context) -> GameState {
        let food_pos = random().into();

        GameState {
            food: Food::new(Color::RED, food_pos),
            snake: Snake::new(SNAKE_POS.into()),
            direction: Direction::new(),
        }
    }
}

impl EventHandler for GameState {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        Background::draw(&mut canvas, ctx);
        self.food.draw(&mut canvas, ctx);
        self.snake.draw(ctx, &mut canvas);

        canvas.finish(ctx)?;

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.snake.update(ctx);

        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new(
        "snake_game", "sabinonweb"
    )
    .window_setup(conf::WindowSetup::default().title("snake_game"))
    .window_mode(conf::WindowMode::default().transparent(true).maximized(false).dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
    .build()?;

    let state = GameState::new(&mut ctx);
    event::run(ctx, event_loop, state)
}
