use crate::{
    data::{GRID_DIMENSION, SCREEN_SIZE}, food::Food, grid::{Direction, Grid}
};
use ggez::{
    graphics::Canvas, * 
};
use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy)]
pub enum Ate {
    Itself,
    Food,
}

#[derive(Debug, PartialEq)]
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

impl From<(i32, i32)> for Segment {
    fn from(value: (i32, i32)) -> Self {
        Segment {
            position: Grid::new(value.0, value.1)
        }
    }
}

pub struct Snake {
    pub head: Segment,
    pub body: Vec<Segment>,
    pub prev_dir: Direction,
    pub curr_dir: Direction,
    pub ate: Option<Ate>
}

impl Snake {
    pub fn new(pos: Grid) -> Snake {
        let mut body: Vec<Segment> = Vec::new();
        body.push(Segment::new(pos.x - (GRID_DIMENSION.0 + 32.0) as i32, pos.y));
        //body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 + 64.0) as i32, pos.y));
        //body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 + 96.0) as i32, pos.y));
        //body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 + 128.0) as i32, pos.y));
        //body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 - 160.0) as i32, pos.y));
        //body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 - 192.0) as i32, pos.y));
        //body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 - 224.0) as i32, pos.y));
        //body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 - 256.0) as i32, pos.y));
        //body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 - 288.0) as i32, pos.y));
        //body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 - 320.0) as i32, pos.y));

        println!("Body: {:?}", body);
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

        for segment in &self.body {
            // println!("draw");
            segment.position.draw_rect(ctx, canvas, color_body);
        }   

        Ok(())
    }

    pub fn update(&mut self, food: &Food) -> GameResult {
        let mut curr_head_pos = self.head.position.current_position().clone();
        let mut curr_body_pos = self.head.position.current_position().clone();

        match self.curr_dir {
            Direction::Up => {
                curr_head_pos.0 += 32;
                self.head.position.y -= GRID_DIMENSION.1 as i32;
            },
                            
            Direction::Down => {
                curr_head_pos.0 -= 32;
                self.head.position.y += GRID_DIMENSION.1 as i32;
            },

            Direction::Left => {
                curr_head_pos.1 += 32;
                self.head.position.x -= GRID_DIMENSION.0 as i32;
                            },

            Direction::Right => {
                curr_head_pos.1 -= 32;
                self.head.position.x += GRID_DIMENSION.0 as i32;
            },
        _ => ()
        }

        for i in 1..self.body.len() {
            println!("len {:?}\n i = {}\n", self.body.len(), i);
            println!("body: {:?}\n", self.body[i].position);
            self.body[i].position = self.body[i - 1].position; 
            println!("[i] = {:?}\n[i-1] = {:?}", self.body[i].position, self.body[i - 1].position);
        }
        self.body[0].position = curr_body_pos.into(); 
 
       if let Some(item) = self.ate {
            match item {
                Ate::Food => self.body.push(curr_body_pos.into()),
                Ate::Itself => ()
            }
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

