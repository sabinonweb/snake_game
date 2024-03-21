use std::fmt::format;

use ggez::{
    conf::{Backend, FullscreenType, NumSamples, WindowMode, WindowSetup}, event::EventHandler, graphics::{Canvas, Color, DrawParam, Text}, *
};
struct State {
    // time each frame has taken
    dt: std::time::Duration,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.dt = ctx.time.delta();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(ctx, Some(Color::BLACK));
        let my_dest = glam::vec2(10.0, 10.0);
        let text = Text::new(format!("Hello Sabin! dt = {}ms", self.dt.as_millis()));
        canvas.draw(&text, DrawParam::default().dest(my_dest));
        println!("frame: {:?}\n\n", ctx.time.fps());

        canvas.finish(ctx)?;
        
        Ok(())
    }
}

pub fn main() {
    let mut state = State {
        dt: std::time::Duration::new(30, 0),
    };
    let c = conf::Conf {
        window_mode: WindowMode {
            width: 800.0,
            height: 600.0,
            maximized: false,
            fullscreen_type: FullscreenType::Windowed,
            borderless: false,
            min_width: 1.0,
            max_width: 0.0,
            min_height: 1.0,
            max_height: 0.0,
            resizable: false,
            visible: true,
            transparent: false,
            resize_on_scale_factor_change: false,
            logical_size: None,
        },
        window_setup: WindowSetup {
            title: "sabinonweb".to_owned(),
            samples: NumSamples::Four,
            vsync: true,
            icon: "".to_owned(),
            srgb: true
        },
        backend: Backend::Vulkan,
    };
    // println!("c = {:?}\n", c);
    let (mut ctx, event_loop) = ContextBuilder::new(
        "snake_game", "awesome_person"
        )
        .default_conf(c)
        .build()
        .unwrap();
    state.draw(&mut ctx);

    event::run(ctx, event_loop, state);
}