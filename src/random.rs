use crate::data::{GRID_SIZE, SCREEN_SIZE};
use rand::{seq::IteratorRandom, thread_rng};
use std::collections::HashSet;

pub(crate) fn random() -> (f32, f32) {
    let mut random_x: HashSet<i32> = HashSet::new();
    let mut random_y: HashSet<i32> = HashSet::new();

    let mut random = thread_rng();

    for i in (0..SCREEN_SIZE.0 as i32).step_by(GRID_SIZE.0 as usize) {
        for j in (0..SCREEN_SIZE.1 as i32).step_by(GRID_SIZE.1 as usize) {
            random_x.insert(i);
            random_y.insert(j);
        } 
    }

    println!("x = {:?}\n y= {:?}", random_x, random_y);

    (
        *random_x.iter().choose(&mut random).unwrap() as f32, 
        *random_y.iter().choose(&mut random).unwrap() as f32,
    )
}