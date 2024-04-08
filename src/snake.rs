use crate::{
    circular_queue::Queue, data::{GRID_DIMENSION, SCREEN_SIZE}, food::Food, grid::{Direction, Grid}
};
use ggez::{
    graphics::Canvas, * 
};
use std::collections::HashSet;

#[derive(Clone, Copy)]
pub enum Ate {
    Itself,
    Food,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segment {
    pub position: Grid,
}

impl Segment {
    fn new(x: i32, y: i32) -> Segment {
        Segment {
            position: Grid::new(x, y)
        }
    }

    pub fn curr_pos(&self) -> &Self {
        self
    }
}

impl From<(i32, i32)> for Segment {
    fn from(value: (i32, i32)) -> Self {
        Segment {
            position: Grid::new(value.0, value.1)
        }
    }
}

pub struct Snake {
    pub head: Segment,
    pub body: Queue,
    pub prev_dir: Direction,
    pub curr_dir: Direction,
    pub ate: Option<Ate>
}

impl Snake {
    pub fn new(pos: Grid) -> Snake {
        let mut body = Queue::new();
        body.enqueue(Segment::new(pos.x - (GRID_DIMENSION.0 + 32.0) as i32, pos.y));

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
        let color_body = [0.97, 1.0, 0.52, 1.0].into(); 

        self.head.position.draw_rect(ctx, canvas, color_head);

        // println!("draw called");
        for segment in self.body {
            segment.position.draw_rect(ctx, canvas, color_body);
        }   

        Ok(())
    }

    pub fn update(&mut self, food: &Food) -> GameResult {
        let mut curr_head_pos = self.head.position.current_position().clone();
        let mut curr_body_pos = self.head.position.current_position().clone();

        match self.curr_dir {
            Direction::Up => {
                curr_head_pos.0 += 8;
                self.head.position.y -= GRID_DIMENSION.1 as i32;
            },
                            
            Direction::Down => {
                curr_head_pos.0 -= 8;
                self.head.position.y += GRID_DIMENSION.1 as i32;
            },

            Direction::Left => {
                curr_head_pos.1 += 8;
                self.head.position.x -= GRID_DIMENSION.0 as i32;
                            },
            Direction::Right => {
                curr_head_pos.1 -= 8;
                self.head.position.x += GRID_DIMENSION.0 as i32;
            },
        _ => ()
        }

        // println!("body len: {:?}\n", self.body.len());
        for i in 1..self.body.queue.len() {
           self.body.queue[i].position = self.body.queue[i - 1].position; 
        }
        self.body.queue[0].position = curr_body_pos.into();  

    Ok(())
}

    pub fn is_snake_within_screen(&self) -> bool {
        let mut x = HashSet::new();
        let mut y = HashSet::new();
        
        for i in 0..SCREEN_SIZE.0 as i32 {
            for j in 0..SCREEN_SIZE.1 as i32 {
                x.insert(i);
                y.insert(j);
            }        
        }
        
        x.contains(&self.head.position.x) && y.contains(&self.head.position.y)
    }

    pub fn snake_ate(&self, food: &Food) -> Option<Ate> {
        if self.head.position == food.position {
            return Some(Ate::Food);
        } 
        
        for seg in &self.body.queue {
            if self.head.position == seg.position {
                return Some(Ate::Itself);
            }
        }
        
        // println!("Snake head: {:?}\n", self.head);
        // println!("Snake body: {:?}\n", self.body);

        None
    }

    pub fn body_pos(&self, curr_pos: &mut Segment) -> Segment {
        match self.curr_dir {
            Direction::Up => {
                curr_pos.position.y += 32;
                *curr_pos
            },
                            
            Direction::Down => {
                curr_pos.position.y -= 32;
                *curr_pos
            },

            Direction::Left => {
                curr_pos.position.x += 32;
                *curr_pos
            },
            Direction::Right => {
                curr_pos.position.x -= 32;
                *curr_pos
            },
        _ => *curr_pos
        }
    }
}

