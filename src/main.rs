use ggez::event::EventHandler;
use ggez::input::mouse::MouseButton;
use ggez::*;
use std::time::Duration;

struct State {
    dt: Duration,
}

// frame time for 30fps
const FT30: Duration = Duration::from_nanos(33_333_333);

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        if self.dt < FT30 {
            timer::sleep(FT30 - self.dt)
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let s = format!("Hello ggez! dt = {}ms", self.dt.subsec_millis());

        let tf = graphics::TextFragment::new(s);

        let t = graphics::Text::new(tf);

        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx, &t, graphics::DrawParam::default())?;
        graphics::present(ctx)?;

        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        println!("{:?} {} {}", button, x, y)
    }
}

fn main() {
    let state = &mut State {
        dt: Duration::new(0, 0),
    };

    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("picross", "llxy")
        .conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state).unwrap();
}
