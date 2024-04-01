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
        body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 - 64.0) as i32, pos.y));
        body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 - 96.0) as i32, pos.y));
        body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 - 128.0) as i32, pos.y));
        body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 - 160.0) as i32, pos.y));
        body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 - 192.0) as i32, pos.y));
        body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 - 224.0) as i32, pos.y));
        body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 - 256.0) as i32, pos.y));
        body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 - 288.0) as i32, pos.y));
        body.push_back(Segment::new(pos.x - (GRID_DIMENSION.0 - 320.0) as i32, pos.y));

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
        let color_body = [0.22, 0.29, 0.76, 1.0].into(); 

        self.head.position.draw_rect(ctx, canvas, color_body);

        for segment in &self.body {
            println!("draw");
            segment.position.draw_rect(ctx, canvas, color_head);
        }   

        Ok(())
    }

    pub fn update(&mut self, food: &Food) -> GameResult {
        let mut curr_head_pos = self.head.position.current_position().clone();
        let mut curr_body_pos = self.head.position.current_position().clone();

        match self.curr_dir {
            Direction::Up => {
                println!("curr_head_pos before: {:?}\n", curr_head_pos);
                curr_head_pos.0 += 32;
                // self.head.position = curr_head_pos.into();
                println!("curr_head_pos after: {:?}\n", curr_head_pos);
                self.head.position.y -= GRID_DIMENSION.1 as i32;
                println!("head_pos: {:?}\n", self.head.position);
                // println!("head: {:?}", self.head.position.current_position());
            },
                            
            Direction::Down => {
                println!("curr_head_pos before: {:?}\n", curr_head_pos);
                curr_head_pos.0 -= 32;
                // self.head.position = curr_head_pos.into();
                println!("curr_head_pos after: {:?}\n", curr_head_pos);
                self.head.position.y += GRID_DIMENSION.1 as i32;
                println!("head_pos: {:?}\n", self.head.position);
                // println!("head: {:?}", self.head.position.current_position());
            },

            Direction::Left => {
                println!("curr_head_pos before: {:?}\n", curr_head_pos);
                curr_head_pos.1 += 32;
                // self.head.position = curr_head_pos.into();
                println!("curr_head_pos after: {:?}\n", curr_head_pos);
                self.head.position.x -= GRID_DIMENSION.0 as i32;
                println!("head_pos: {:?}\n", self.head.position);
                // println!("head: {:?}", self.head.position.current_position());  
            },

            Direction::Right => {
                println!("curr_head_pos before: {:?}\n", curr_head_pos);
                curr_head_pos.1 -= 32;
                // self.head.position = curr_head_pos.into();
                println!("curr_head_pos after: {:?}\n", curr_head_pos);
                self.head.position.x += GRID_DIMENSION.0 as i32;
                println!("head_pos: {:?}\n", self.head.position);
                // println!("head: {:?}", self.head.position.current_position());
            },
        _ => ()
        }

        for segment in &mut self.body {
            // match self.curr_dir {
            //     Direction::Up => {
            //         segment.position.y -= 32;
            //         // println!("body: {:?}", segment.position.current_position());
            //     },
            //     Direction::Down => {
            //         segment.position.y += 32;
            //         // println!("body: {:?}", segment.position.current_position());
            //     },
            //     Direction::Left => {
            //         segment.position.x -= 32;
            //         // println!("body: {:?}", segment.position.current_position());
            //     },
            //     Direction::Right => {
            //         segment.position.x += 32;
            //         // println!("body: {:?}", segment.position.current_position());
            //     },
            //     _ => ()
            // }
            println!("\n\nseg:{:?}\n\n", segment);
            segment.position = curr_body_pos.into();
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

