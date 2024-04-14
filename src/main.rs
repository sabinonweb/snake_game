use crate::{
    background::Background,
    data::SCREEN_SIZE,
    food::Food,
    menu::Menu,
    grid::Direction,
    snake::{Ate, Snake},
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
mod menu;
mod random;
mod snake;

pub struct GameState {
    food: Food,
    snake: Snake,
    game_over: bool,
    score: u32,
    fps: u32,
    game_mode: GameMode,
    menu: Menu,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameMode {
    Menu,
    Screen,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            food: Food::new([0.98, 0.98, 0.85, 1.0].into()),
            snake: Snake::new((0, 0).into()),
            game_over: false,
            score: 0,
            fps: 8,
            game_mode: GameMode::Menu,
            menu: Menu::new(),
        }
    }
}

impl EventHandler for GameState {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);
        Background::draw(&mut canvas, ctx, &self.game_mode)?;

        match self.game_mode {
            GameMode::Menu => {
                self.menu.draw(&mut canvas, ctx)?; 
            }

            GameMode::Screen => { 
                self.food.draw(&mut canvas, ctx)?;
                self.snake.draw(ctx, &mut canvas)?; 
            } 
        }
        canvas.finish(ctx)?;
        
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        while ctx.time.check_update_time(self.fps) && !self.game_over && self.game_mode == GameMode::Screen {
            self.snake.update()?;

            if let Some(ate) = self.snake.snake_ate(&self.food) {
                match ate {
                    Ate::Food => {
                        let curr_pos = self.snake.body.last().unwrap();                   
                        self.score += 1;
                        let fps = Some(self.fps + (self.score * 1 / 5) as u32);
                        self.fps = match fps {
                            Some(fps) => {
                                if fps >= 15 {
                                    15
                                } else {
                                    fps
                                }
                            }
                            None => 8,
                        }; 
                        self.snake.body.push(curr_pos.clone());

                        self.food.position = Food::food_pos();
                    } 
                    Ate::Itself => self.game_over = true,
                }
            } 
        } 

        if self.game_over {
            self.game_mode = GameMode::Menu;
        }

        Ok(())
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            _button: event::MouseButton,
            x: f32,
            y: f32,
        ) -> Result<(), GameError> {
        if self.game_mode == GameMode::Menu {
            let x_coord = self.menu.x_coord();
            let y_coord = self.menu.y_coord();

            if x < x_coord.1 && x > x_coord.0 && y < y_coord.1 && y > y_coord.0 {
                self.game_mode = GameMode::Screen;
            }
        }
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
                    ctx.request_quit();
                } else if input.mods.contains(KeyMods::SHIFT) || input.mods.contains(KeyMods::CTRL) {
                    println!("Press both CTRL and SHIFT to kill the beast!");
                } else {
                    println!("Ayee Dinesh! Atleast try a little!");
                } 
            }

            KeyCode::Space => {
               self.game_mode = GameMode::Screen;
            }

            KeyCode::W => {  
                let direction = Direction::from_keyword(KeyCode::W);
                if self.snake.prev_dir != direction.inverse() && self.snake.curr_dir != direction {
                    self.snake.curr_dir = Direction::Up; 
                } 
                self.snake.prev_dir = self.snake.curr_dir;
            }

            KeyCode::A => {
                let direction = Direction::from_keyword(KeyCode::A);
                if self.snake.prev_dir != direction.inverse() && self.snake.curr_dir != direction {
                    self.snake.curr_dir = Direction::Left
                }
                self.snake.prev_dir = self.snake.curr_dir;
            }

            KeyCode::S => {
                let direction = Direction::from_keyword(KeyCode::S);
                if self.snake.prev_dir != direction.inverse() && self.snake.curr_dir != direction {
                    self.snake.curr_dir = Direction::Down;
                }
                self.snake.prev_dir = self.snake.curr_dir;
            }

            KeyCode::D => {
                let direction = Direction::from_keyword(KeyCode::D);
                if self.snake.prev_dir != direction.inverse() && self.snake.curr_dir != direction {
                    self.snake.curr_dir = Direction::Right;
                }
                self.snake.prev_dir = self.snake.curr_dir;
            }
            _ => ()
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new(
        "snake_game", "sabinonweb"
    )
    .window_setup(conf::WindowSetup::default().title("snake_game"))
    .window_mode(conf::WindowMode::default().transparent(true).maximized(false).dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .add_resource_path("../resources")
    .build()?;

    println!("x = {} y = {}", SCREEN_SIZE.0, SCREEN_SIZE.1);
    let state = GameState::new();
    event::run(ctx, event_loop, state)
}
