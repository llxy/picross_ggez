use ggez::event::EventHandler;
use ggez::input::mouse::MouseButton;
use ggez::*;
use std::time::Duration;

mod puzzle;
use puzzle::Puzzle;

struct State {
    dt: Duration,
    puzzle: Puzzle,
    solution: Vec<Vec<bool>>,
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

        graphics::clear(ctx, graphics::BLACK);

        // Draw current solution
        for (r, row) in self.solution.iter().enumerate() {
            for (c, piece) in row.iter().enumerate() {
                let color = match piece {
                    true => graphics::WHITE,
                    false => graphics::BLACK,
                };

                let x = r * 150 + 500;
                let y = c * 150 + 300;

                let rect = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(x as f32, y as f32, 130.0, 130.0),
                    color,
                )?;
                graphics::draw(ctx, &rect, graphics::DrawParam::default())?;
            }
        }

        let s = if self.puzzle.check(self.solution.clone()) {
            "Correct".to_string()
        } else {
            format!("Picross version -1\n\ndt = {}ms", self.dt.subsec_millis())
        };
        let tf = graphics::TextFragment::new(s).scale(Scale { x: 50.0, y: 50.0 });
        let t = graphics::Text::new(tf);
        // let (tw, th) = t.dimensions(ctx);
        // let center_dest = Point2::new(1280.0 - (tw as f32 / 2.0), 800.0 - (th as f32 / 2.0));
        graphics::draw(ctx, &t, DrawParam::default())?;

        // let ncols = self.solution.ncols();

        // for (r, c, piece) in self.solution.iter().enumerate().map(|(n, x)| (n / ncols, n % ncols, x)) {
        // }

        // Draw hints

        // row hints

        // col hints

        graphics::present(ctx)?;

        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let min_x = 500 - 5;
        let max_x = 500 + 5 * 150;

        let min_y = 300 - 5;
        let max_y = 300 + 5 * 150;

        let ix = x.floor() as i32;
        let iy = y.floor() as i32;

        if button != MouseButton::Left {
            return;
        }

        if (ix >= min_x) && (ix <= max_x) && (iy >= min_y) && (iy <= max_y) {
            let r = (ix - 500) / 150;
            let c = (iy - 300) / 150;

            let min_bx = 500 + r * 150 - 5;
            let max_bx = min_bx + 140;

            let min_by = 300 + c * 150 - 5;
            let max_by = min_by + 140;

            if (ix >= min_bx) && (ix <= max_bx) && (iy >= min_by) && (iy <= max_by) {
                let rs = r as usize;
                let cs = c as usize;
                self.solution[rs][cs] = !self.solution[rs][cs];
            }
        }

        // println!("{:?} {} {}", button, ix, iy)
    }
}

fn main() {
    let state = &mut State {
        dt: Duration::new(0, 0),
        puzzle: Puzzle::rand_new(),
        solution: init_solution(),
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

fn init_solution() -> Vec<Vec<bool>> {
    vec![vec![false; 5]; 5]
}
