pub const GRID_SIZE: (f32, f32) = (30.0, 20.0);
pub const GRID_DIMENSION: (f32, f32) = (32.0 , 32.0);

pub const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 * GRID_DIMENSION.0,
    GRID_SIZE.1 * GRID_DIMENSION.1
);

pub const FPS: i32 = 8;
pub const SNAKE_POS: (i32, i32) = (GRID_SIZE.0 as i32 / 4, GRID_SIZE.1 as i32 / 2);

