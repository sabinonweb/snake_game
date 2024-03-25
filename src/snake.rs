use ggez::*;
use crate::{background::Grid, head::Head, random::random};
use std::collections::VecDeque;

pub struct Snake {
    head: Head,
    body: VecDeque<Grid>,
}

impl Snake {
    pub fn new(mut ctx: &mut Context) -> Snake {
        Snake {
            head: Head::new(&mut ctx).unwrap(),
            body: VecDeque::new(),
        }    
    }


}