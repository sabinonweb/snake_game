use crate::{
    data::{GRID_DIMENSION, SCREEN_SIZE}, food::Food, grid::{Direction, Grid}
};
use ggez::{
    graphics::Canvas, * 
};

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
        let color_body = [0.95, 0.95, 0.25, 1.0].into(); 

        self.head.position.draw_rect(ctx, canvas, color_head)?; 

        for segment in &self.body {
            segment.position.draw_rect(ctx, canvas, color_body)?;
        }   

        Ok(())
    }

    pub fn update(&mut self) -> GameResult {
        let mut curr_head_pos = self.head.position.current_position().clone();

        match self.curr_dir {
            Direction::Up => {
                self.head.position.y -= GRID_DIMENSION.1 as i32;
            },
                            
            Direction::Down => {
                self.head.position.y += GRID_DIMENSION.1 as i32;
            },

            Direction::Left => {
                self.head.position.x -= GRID_DIMENSION.0 as i32;
                            },
            Direction::Right => {
                self.head.position.x += GRID_DIMENSION.0 as i32;
            },
        _ => ()
        } 
        
        // replaces the curr_segment's position with prev segment's position
        for segment in &mut self.body {
            let curr_pos = segment.position.current_position();
            segment.position = curr_head_pos.into();
            curr_head_pos = curr_pos.into();
        }

    Ok(())
}

    pub fn snake_ate(&self, food: &Food) -> Option<Ate> {
        if <(i32, i32) as Into<Grid>>::into((self.head.position.x % SCREEN_SIZE.0 as i32, self.head.position.y % SCREEN_SIZE.1 as i32)) == food.position {
            return Some(Ate::Food);
        } 
        
        for seg in &self.body {
            if self.head.position == seg.position {
                return Some(Ate::Itself);
            }
        }

        None
    }
}

