use ggez::{
    glam::Vec2, graphics::{Canvas, Color, DrawMode, Rect}, *
};

pub struct Grid {
    length: f32,
    rectangle: graphics::Mesh,
}

impl Grid {
    fn new(ctx: &mut Context) -> GameResult<Grid> {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(), 
            Rect {
                x: 10.0,
                y: 10.0,
                w: 32.0,
                h: 32.0,
            }, 
            Color::WHITE
        )?;

        Ok(Grid {length: 10.0, rectangle})
    }
}

impl event::EventHandler<ggez::GameError> for Grid {
    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = 
            Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(&self.rectangle, Vec2::new(self.length, 300.0));

        canvas.finish(ctx)?;

        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context) -> Result<(), ggez::GameError> {
        self.length = self.length + 1.0;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("snake_game", "sabinonweb");
    let (mut ctx, event_loop) = cb.build()?;
    let state = Grid::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}