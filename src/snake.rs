use crate::{
    data::{GRID_DIMENSION, SCREEN_SIZE}, food::Food, grid::{Direction, Grid}
};
use ggez::{
    *,
    graphics::Canvas, 
};
use std::collections::{HashSet, VecDeque};

pub enum Ate {
    Itself,
    Food,
}

#[derive(Debug)]
pub struct Segment {
    position: Grid,
}

impl Segment {
    fn new(x: i32, y: i32) -> Segment {
        Segment {
            position: Grid::new(x, y)
        }
    }
}

pub struct Snake {
    pub head: Segment,
    pub body: VecDeque<Segment>,
    pub prev_dir: Direction,
    pub curr_dir: Direction,
    pub ate: Option<Ate>
}

impl Snake {
    pub fn new(pos: Grid) -> Snake {
        let mut body: VecDeque<Segment> = VecDeque::new();
        body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 - 32.0) as i32, pos.y));
              
        Snake {
            head: Segment::new(pos.x - GRID_DIMENSION.0 as i32, pos.y),
            body,
            prev_dir: Direction::None,
            curr_dir: Direction::Right,   
            ate: None,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let color_head = [0.0, 0.0, 0.0, 1.0].into();
        let color_body = [0.22, 0.29, 0.76, 1.0].into();
        
        // println!("Head position: {:?}", self.head.position);
        // println!("Body position: {:?}", self.body);

        self.head.position.draw_rect(ctx, canvas, color_body);

        for segment in &self.body {
            segment.position.draw_rect(ctx, canvas, color_head);
        }   

        Ok(())
    }

    pub fn update(&mut self, food: &Food) -> GameResult {
            match self.curr_dir {
                Direction::Up => {
                    self.head.position.y -= 32;
                },
                
                Direction::Down => {
                    self.head.position.y += 32;
                },

                Direction::Left => {
                    self.head.position.x -= 32;
                },

                Direction::Right => {
                    self.head.position.x += 32;
                },
            _ => ()
            }

        for segment in &mut self.body {
            match self.curr_dir {
                Direction::Up => segment.position.y -= 32,
                Direction::Down => segment.position.y += 32,
                Direction::Left => segment.position.x -= 32,
                Direction::Right => segment.position.x += 32,
                _ => ()
            }
        }

        if self.ate_food(food) {
            self.ate = Some(Ate::Food);
        } else if self.ate_itself() {
            self.ate = Some(Ate::Itself);
        } else {
            self.ate = None;
        }

    Ok(())
}

    pub fn is_snake_within_screen(&self) -> bool {
        let mut x = HashSet::new();
        let mut y = HashSet::new();
        
        println!("Screen called!");

        for i in 0..SCREEN_SIZE.0 as i32 {
            for j in 0..SCREEN_SIZE.1 as i32 {
                x.insert(i);
                y.insert(j);
            }        
        }
        
        x.contains(&self.head.position.x) && y.contains(&self.head.position.y)
    }

    pub fn ate_food(&self, food: &Food) -> bool {
        self.head.position == food.position
    }

    pub fn ate_itself(&self) -> bool {
        for seg in &self.body {
            return self.head.position == seg.position;

        }
        false
    }
}

