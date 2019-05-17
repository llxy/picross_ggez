use ggez::event::EventHandler;
use ggez::input::mouse::MouseButton;
use ggez::*;
use std::time::Duration;

struct State {
    dt: Duration,
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if timer::check_update_time(ctx, 30) {
            self.dt = timer::delta(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use ggez::graphics::{DrawParam, Scale};
        use nalgebra::Point2;

        let s = format!("Picross version -1\n\ndt = {}ms", self.dt.subsec_millis());
        let tf = graphics::TextFragment::new(s).scale(Scale { x: 50.0, y: 50.0 });
        let t = graphics::Text::new(tf);

        graphics::clear(ctx, graphics::BLACK);
        let (tw, th) = t.dimensions(ctx);
        let center_dest = Point2::new(1280.0 - (tw as f32 / 2.0), 800.0 - (th as f32 / 2.0));
        graphics::draw(ctx, &t, DrawParam::default().dest(center_dest))?;
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

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("picross", "llxy")
        .conf(get_conf())
        .build()
        .unwrap();

    event::run(ctx, event_loop, state).unwrap();
}

fn get_conf() -> conf::Conf {
    use ggez::conf::*;

    let ws = WindowSetup {
        title: "Picross".to_owned(),
        samples: NumSamples::Zero,
        vsync: true,
        transparent: false,
        icon: "".to_owned(),
        srgb: true,
    };

    let wm = WindowMode {
        width: 2560.0,
        height: 1600.0,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        hidpi: true,
        resizable: false,
    };

    Conf {
        window_mode: wm,
        window_setup: ws,
        backend: Backend::default(),
        modules: ModuleConf::default(),
    }
}
