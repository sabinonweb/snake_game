use crate::data::{GRID_SIZE, SCREEN_SIZE};
use rand::{seq::IteratorRandom, thread_rng};
use std::collections::HashSet;

pub(crate) fn random() -> (i32, i32) {
    let mut random_x: HashSet<i32> = HashSet::new();
    let mut random_y: HashSet<i32> = HashSet::new();

    let mut random = thread_rng();

    for i in (0..SCREEN_SIZE.0 as i32).step_by(GRID_SIZE.0 as usize) {
        for j in (0..SCREEN_SIZE.1 as i32).step_by(GRID_SIZE.1 as usize) {
            random_x.insert(i);
            random_y.insert(j);
        } 
    }

    let mut x: Vec<i32> = random_x.clone().into_iter().collect();
    let mut y: Vec<i32> = random_y.clone().into_iter().collect();
    x.sort();
    y.sort();

    // println!("x = {:?}\n y= {:?}\n", x, y);

    (
        *random_x.iter().choose(&mut random).unwrap(), 
        *random_y.iter().choose(&mut random).unwrap(),
    )
}
