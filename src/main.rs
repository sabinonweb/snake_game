use crate::{
    background::Background,
    data::{SCREEN_SIZE, SNAKE_POS},
    food::Food,
    grid::{Direction, Grid},
    random::random,
    snake::Snake,
};
use ggez::{
    event::EventHandler,  
    graphics::{Canvas, Color}, 
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
}

impl GameState {
    fn new() -> GameState {
        let food_pos = random().into();

        GameState {
            food: Food::new(Color::RED, food_pos),
            snake: Snake::new(SNAKE_POS.into()),
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

    fn key_down_event(
            &mut self,
            ctx: &mut Context,
            input: KeyInput,
            _repeated: bool,
        ) -> GameResult {
        match input.keycode.unwrap() {
            KeyCode::Q => {
                if input.mods.contains(KeyMods::SHIFT) && input.mods.contains(KeyMods::CTRL) {
                    println!("Ahhh! You killed the snake!");
                    ctx.request_quit();
                } else if input.mods.contains(KeyMods::SHIFT) || input.mods.contains(KeyMods::CTRL) {
                    println!("Press both CTRL and SHIFT to kill the beast!");
                } else {
                    println!("Atleast try a little!");
                } 
            }
            KeyCode::W => {
               let dir = Direction::from_keyword(KeyCode::W);
                println!("W pressed\n\n");
                println!("dir.inver() {:?}\n\n", &dir.inverse());                
                println!("prev_dir: {:?}\n\n", self.snake.prev_dir);
                println!("curr_dir: {:?}\n\n", self.snake.curr_dir);

                if self.snake.prev_dir != dir.inverse() && self.snake.prev_dir != self.snake.curr_dir {
                    self.snake.prev_dir = self.snake.curr_dir;
                    println!("self.current_dir: {:?}\n\n", self.snake.curr_dir);
                }
                self.snake.curr_dir = Direction::Up;
            }
            KeyCode::A => {
                let dir = Direction::from_keyword(KeyCode::A);
                println!("A pressed\n\n");
                println!("dir.inver() {:?}\n\n", &dir.inverse());
                println!("prev_dir: {:?}\n\n", self.snake.prev_dir);
                println!("curr_dir: {:?}\n\n", self.snake.curr_dir);

                if self.snake.prev_dir != dir.inverse() && self.snake.prev_dir != self.snake.curr_dir {
                    self.snake.prev_dir = self.snake.curr_dir;
                    println!("self.current_dir: {:?}\n\n", self.snake.curr_dir);
                }
                self.snake.curr_dir = Direction::Left;
            }
            KeyCode::S => {
                let dir = Direction::from_keyword(KeyCode::S);
                println!("S pressed\n\n");
                println!("dir.inver() {:?}\n\n",& dir.inverse());
                println!("prev_dir: {:?}\n\n", self.snake.prev_dir);
                println!("curr_dir: {:?}\n\n", self.snake.curr_dir);

                if self.snake.prev_dir != dir.inverse() && self.snake.prev_dir != self.snake.curr_dir {
                    self.snake.prev_dir = self.snake.curr_dir;
                    println!("self.current_dir: {:?}\n\n", self.snake.curr_dir);
                }
                self.snake.curr_dir = Direction::Down;
            }
            KeyCode::D => {
                let dir = Direction::from_keyword(KeyCode::D);
                println!("D pressed\n\n");
                println!("dir.inver() {:?}\n\n", &dir.inverse());
                println!("prev_dir: {:?}\n\n", self.snake.prev_dir);
                println!("curr_dir: {:?}\n\n", self.snake.curr_dir);

                if self.snake.prev_dir != dir.inverse() && self.snake.prev_dir != self.snake.curr_dir {
                    self.snake.prev_dir = self.snake.curr_dir;
                    println!("self.current_dir: {:?}\n\n", self.snake.curr_dir);
                }
                self.snake.curr_dir = Direction::Right;
            }
            _ => ()
        }
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

    let state = GameState::new();
    event::run(ctx, event_loop, state)
}
