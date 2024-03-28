use crate::{
    data::GRID_DIMENSION,
    grid::{Direction, Grid},
};
use ggez::{
    *,
    graphics::Canvas, 
};
use std::collections::VecDeque;

pub struct Snake {
    pub head: Grid,
    pub body: VecDeque<Grid>,
    pub prev_dir: Direction,
    pub curr_dir: Direction,
}

impl Snake {
    pub fn new(pos: Grid) -> Snake {
        let mut body: VecDeque<Grid> = VecDeque::new();
        body.push_back(Grid::new(pos.x as i32, pos.y));
        // body.push_back(Grid::new(pos.x, pos.y));
       // body.push_back(Grid::new(pos.x - GRID_DIMENSION.0 as i32, pos.y));
       // body.push_back(Grid::new(pos.x - (GRID_DIMENSION.0 - 32.0) as i32, pos.y));
       // body.push_back(Grid::new(pos.x - (GRID_DIMENSION.0 - 64.0) as i32, pos.y));
       // body.push_back(Grid::new(pos.x - (GRID_DIMENSION.0 - 96.0) as i32, pos.y));

        Snake {
           head: Grid::new(pos.x + GRID_DIMENSION.0 as i32, pos.y),
            body,
            prev_dir: Direction::Right,
            curr_dir: Direction::Right,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        //let color_head = [0.0, 0.0, 0.0, 1.0].into();
        let color_body = [0.73, 0.77, 0.4, 1.0].into();

        self.head.draw_rect(ctx, canvas, color_body);

        //for segment in &self.body {
        //    segment.draw_rect(ctx, canvas, color_body);
        //}   

        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        println!("Current Direction: {:?}", self.curr_dir);
        match self.curr_dir {
            Direction::Up => self.head.y += 32,
            Direction::Down => self.head.y -= 32,
            Direction::Right => self.head.x += 32,
            Direction::Left => self.head.x -= 32,
            _ => ()
        }

        Ok(())
    }
}

