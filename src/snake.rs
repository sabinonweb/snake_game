use ggez::*;
use crate::{background::Grid, head::Head};

pub struct Snake {
    head: Head,
    body: Vec<Grid>,
}

impl Snake {
    fn new(ctx: Context) -> Snake {
        Snake {
            head: Head::new(ctx),
            body: Vec::new(),
        }    
    }
}