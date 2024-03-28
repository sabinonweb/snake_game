use ggez::{
    graphics::{Canvas, Color},
    *,
};
use crate::{
    data::{GRID_SIZE, SCREEN_SIZE},
    grid::Grid,
};
use std::collections::HashSet;

pub struct Background;

impl Background {
    pub fn draw(canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        let mut random_x: HashSet<i32> = HashSet::new();
        let mut random_y: HashSet<i32> = HashSet::new();

        for i in (0..SCREEN_SIZE.0 as i32).step_by(GRID_SIZE.0 as usize) {
            for j in (0..SCREEN_SIZE.1 as i32).step_by(GRID_SIZE.1 as usize) {
                // println!("I = {} j = {}", i, j);
                random_x.insert(i);
                random_y.insert(j);
                let background = Grid::new(i, j);
                let color_even: Color = [0.01, 0.18, 0.06, 1.0].into();
                let color_odd: Color = [0.0, 1.0, 0.47, 1.0].into();
                if is_even(i / 30) && is_even(j / 20) {
                    background.draw_rect(ctx, canvas, color_even)?; 
                }
                else {
                    background.draw_rect(ctx, canvas, color_even)?;
                }
            }
        }

        let mut x: Vec<i32> = random_x.clone().into_iter().collect();
        let mut y: Vec<i32> = random_y.clone().into_iter().collect();
        x.sort();
        y.sort();

    // println!("x_bg = {:?}\n y_bg = {:?}\n", x, y);


        Ok(())
    }
}

pub(crate) fn is_even(n : i32) -> bool {
        n % 2 == 0
    }

