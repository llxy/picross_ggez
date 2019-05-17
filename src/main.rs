use ggez::event::EventHandler;
use ggez::input::mouse::MouseButton;
use ggez::*;
use nalgebra::DMatrix;
use std::time::Duration;

struct State {
    dt: Duration,
    puzzle: DMatrix<bool>,
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

        let pit = self.puzzle.iter();
        let ncols = self.puzzle.ncols();
        for (n, piece) in pit.enumerate() {
            let color = match piece {
                true => graphics::WHITE,
                false => graphics::BLACK,
            };

            let x = n / ncols * 220 + 300;
            let y = n % ncols * 220 + 300;

            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(x as f32, y as f32, 200.0, 200.0),
                color,
            )?;
            graphics::draw(ctx, &rect, graphics::DrawParam::default())?;
        }

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
        puzzle: rand_puzzle(),
    };

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("picross", "llxy")
        .conf(get_conf())
        .build()
        .unwrap();

    event::run(ctx, event_loop, state).unwrap();
}

fn rand_puzzle() -> DMatrix<bool> {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    DMatrix::from_fn(5, 5, |_r, _c| rng.gen_bool(0.5))
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
