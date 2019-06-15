use ggez::event::EventHandler;
use ggez::input::mouse::MouseButton;
use ggez::*;

mod puzzle;
use puzzle::Puzzle;

struct State {
    puzzle: Puzzle,
    solution: Vec<Vec<bool>>,
}

const SIZE: i32 = 100;
const BORDER: i32 = 20;

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use ggez::graphics::{DrawMode, DrawParam, Mesh, Rect, Scale};
        use nalgebra::Point2;

        graphics::clear(ctx, graphics::BLACK);

        // Draw grid
        for i in 0..=5 {
            let x_start = (500 - 10) as f32;
            let x_end = (500 - 10 + 5 * SIZE) as f32;
            let h_y = (300 - 10 + i * SIZE) as f32;
            let h_line = Mesh::new_line(
                ctx,
                &[Point2::new(x_start, h_y), Point2::new(x_end, h_y)],
                4.0,
                graphics::WHITE,
            )?;
            graphics::draw(ctx, &h_line, DrawParam::default())?;

            let y_start = (300 - 10) as f32;
            let y_end = (300 - 10 + 5 * SIZE) as f32;
            let v_x = (500 - 10 + i * SIZE) as f32;
            let v_line = Mesh::new_line(
                ctx,
                &[Point2::new(v_x, y_start), Point2::new(v_x, y_end)],
                4.0,
                graphics::WHITE,
            )?;
            graphics::draw(ctx, &v_line, DrawParam::default())?;
        }

        // Draw current solution
        for (r, row) in self.solution.iter().enumerate() {
            for (c, piece) in row.iter().enumerate() {
                let color = match piece {
                    true => graphics::WHITE,
                    false => graphics::BLACK,
                };

                let x = r as i32 * SIZE + 500;
                let y = c as i32 * SIZE + 300;

                let rect = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect::new(
                        x as f32,
                        y as f32,
                        (SIZE - BORDER) as f32,
                        (SIZE - BORDER) as f32,
                    ),
                    color,
                )?;
                graphics::draw(ctx, &rect, DrawParam::default())?;
            }
        }

        let s = if self.puzzle.check(self.solution.clone()) {
            "Correct"
        } else {
            "Not yet..."
        };
        let tf = graphics::TextFragment::new(s).scale(Scale { x: 50.0, y: 50.0 });
        let t = graphics::Text::new(tf);
        graphics::draw(ctx, &t, DrawParam::default())?;

        // row hints
        for (r, hint) in self.puzzle.row_hints().iter().enumerate() {
            let tf = graphics::TextFragment::new(hint.clone()).scale(Scale { x: 50.0, y: 50.0 });
            let t = graphics::Text::new(tf);
            let center_dest = Point2::new(200.0, 300.0 + r as f32 * SIZE as f32);
            graphics::draw(ctx, &t, DrawParam::default().dest(center_dest))?;
        }

        // col hints
        for (c, hint) in self.puzzle.col_hints().iter().enumerate() {
            let tf = graphics::TextFragment::new(hint.clone()).scale(Scale { x: 50.0, y: 50.0 });
            let t = graphics::Text::new(tf);
            let center_dest = Point2::new(550.0 + c as f32 * SIZE as f32, 75.0);
            graphics::draw(ctx, &t, DrawParam::default().dest(center_dest))?;
        }

        graphics::present(ctx)?;

        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let min_x = 500 - 5;
        let max_x = 500 + 5 * SIZE;

        let min_y = 300 - 5;
        let max_y = 300 + 5 * SIZE;

        let ix = x.floor() as i32;
        let iy = y.floor() as i32;

        if button != MouseButton::Left {
            return;
        }

        if (ix >= min_x) && (ix <= max_x) && (iy >= min_y) && (iy <= max_y) {
            let r = (ix - 500) / SIZE;
            let c = (iy - 300) / SIZE;

            let min_bx = 500 + r * SIZE - 5;
            let max_bx = min_bx + 140;

            let min_by = 300 + c * SIZE - 5;
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
        title: "Picross.LLXY".to_owned(),
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
