use crate::data::SCREEN_SIZE;
use rand::{seq::IteratorRandom, thread_rng};
use std::collections::HashSet;

pub(crate) fn random() -> (i32, i32) {
    let mut random_x: Vec<i32> = Vec::new();
    let mut random_y: Vec<i32> = Vec::new();

    let mut random = thread_rng();

    for i in (0..SCREEN_SIZE.0 as i32 - 32).step_by(32) {
        for j in (0..SCREEN_SIZE.1 as i32 - 32).step_by(32) {
            random_x.push(i);
            random_y.push(j);
        } 
    }

    random_x.sort();
    random_y.sort();

    (
        *random_x.iter().choose(&mut random).unwrap(), 
        *random_y.iter().choose(&mut random).unwrap(),
    )
}
