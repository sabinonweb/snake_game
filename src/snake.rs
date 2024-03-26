use ggez::*;
use crate::{background::Grid, bit::Bit, food::Food, random::random};
use std::collections::VecDeque;

pub struct Snake {
    food: Food,
    head: Bit,
    body: VecDeque<Bit>,
}

impl Snake {
    pub fn new(mut ctx: &mut Context) -> Snake {
        Snake {
            food: Food::new(&mut ctx),
            head: Bit::new(&mut ctx).unwrap(),
            body: VecDeque::new(),
        }    
    }

    pub fn eats_food(&self) -> bool {
        self.food.position == self.head.position
    }

    pub fn eats_itself(&self) -> bool {
        for bit in &self.body {
            for part in &self.body {
                bit == part | bit == self.head | part == self.head
            }
        }   
    }
}
