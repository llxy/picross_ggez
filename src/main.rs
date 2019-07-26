use ggez::event::{quit, EventHandler, KeyCode, KeyMods};
use ggez::input::mouse::MouseButton;
use ggez::*;
use ron::ser::{to_string_pretty, PrettyConfig};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod puzzle;
use puzzle::Puzzle;

struct Picross {
    puzzle: Puzzle,
    solution: Vec<Vec<bool>>,
}

const PUZZLE_SIZE: usize = 8;

const PIECE_SIZE: i32 = 80;
const BORDER: i32 = 20;

impl EventHandler for Picross {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    #[allow(clippy::unreadable_literal)]
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, Rect, Scale};
        use nalgebra::Point2;

        graphics::clear(ctx, Color::from_rgb_u32(0x2E3440));

        // Draw grid
        for i in 0..=PUZZLE_SIZE {
            let x_start = (500 - 10) as f32;
            let x_end = (500 - 10 + PUZZLE_SIZE as i32 * PIECE_SIZE) as f32;
            let h_y = (300 - 10 + i as i32 * PIECE_SIZE) as f32;
            let h_line = Mesh::new_line(
                ctx,
                &[Point2::new(x_start, h_y), Point2::new(x_end, h_y)],
                4.0,
                Color::from_rgb_u32(0x4C566A),
            )?;
            graphics::draw(ctx, &h_line, DrawParam::default())?;

            let y_start = (300 - 10) as f32;
            let y_end = (300 - 10 + PUZZLE_SIZE as i32 * PIECE_SIZE) as f32;
            let v_x = (500 - 10 + i as i32 * PIECE_SIZE) as f32;
            let v_line = Mesh::new_line(
                ctx,
                &[Point2::new(v_x, y_start), Point2::new(v_x, y_end)],
                4.0,
                Color::from_rgb_u32(0x4C566A),
            )?;
            graphics::draw(ctx, &v_line, DrawParam::default())?;
        }

        // Draw current solution
        for (r, row) in self.solution.iter().enumerate() {
            for (c, piece) in row.iter().enumerate() {
                if *piece {
                    let cl = Color::from_rgb_u32(0xD8DEE9);

                    let x = r as i32 * PIECE_SIZE + 500;
                    let y = c as i32 * PIECE_SIZE + 300;

                    let rect = Mesh::new_rectangle(
                        ctx,
                        DrawMode::fill(),
                        Rect::new(
                            x as f32,
                            y as f32,
                            (PIECE_SIZE - BORDER) as f32,
                            (PIECE_SIZE - BORDER) as f32,
                        ),
                        cl,
                    )?;
                    graphics::draw(ctx, &rect, DrawParam::default())?;
                }
            }
        }

        {
            let s = if self.puzzle.check(self.solution.clone()) {
                "Correct"
            } else {
                "Not yet..."
            };
            let tf = graphics::TextFragment::new(s).scale(Scale { x: 50.0, y: 50.0 });
            let t = graphics::Text::new(tf);
            graphics::draw(ctx, &t, DrawParam::default().dest(Point2::new(10.0, 10.0)))?;
        }

        {
            let s = format!("Size: {}", PUZZLE_SIZE);
            let tf = graphics::TextFragment::new(s).scale(Scale { x: 50.0, y: 50.0 });
            let t = graphics::Text::new(tf);
            graphics::draw(ctx, &t, DrawParam::default().dest(Point2::new(10.0, 70.0)))?;
        }

        // row hints
        for (r, hint) in self.puzzle.row_hints().iter().enumerate() {
            let tf = graphics::TextFragment::new(hint.clone()).scale(Scale { x: 50.0, y: 50.0 });
            let t = graphics::Text::new(tf);
            let center_dest = Point2::new(200.0, 300.0 + r as f32 * PIECE_SIZE as f32);
            graphics::draw(ctx, &t, DrawParam::default().dest(center_dest))?;
        }

        // col hints
        for (c, hint) in self.puzzle.col_hints().iter().enumerate() {
            let tf = graphics::TextFragment::new(hint.clone()).scale(Scale { x: 50.0, y: 50.0 });
            let t = graphics::Text::new(tf);
            let center_dest = Point2::new(550.0 + c as f32 * PIECE_SIZE as f32, 75.0);
            graphics::draw(ctx, &t, DrawParam::default().dest(center_dest))?;
        }

        graphics::present(ctx)?;

        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let min_x = 500 - 5;
        let max_x = 500 + PUZZLE_SIZE as i32 * PIECE_SIZE;

        let min_y = 300 - 5;
        let max_y = 300 + PUZZLE_SIZE as i32 * PIECE_SIZE;

        let ix = x.floor() as i32;
        let iy = y.floor() as i32;

        if button != MouseButton::Left {
            return;
        }

        if (ix >= min_x) && (ix <= max_x) && (iy >= min_y) && (iy <= max_y) {
            let r = (ix - 500) / PIECE_SIZE;
            let c = (iy - 300) / PIECE_SIZE;

            let min_bx = 500 + r * PIECE_SIZE - 5;
            let max_bx = min_bx + 140;

            let min_by = 300 + c * PIECE_SIZE - 5;
            let max_by = min_by + 140;

            if (ix >= min_bx) && (ix <= max_bx) && (iy >= min_by) && (iy <= max_by) {
                let rs = r as usize;
                let cs = c as usize;
                self.solution[rs][cs] = !self.solution[rs][cs];
            }
        }
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _: KeyMods, _repeat: bool) {
        if keycode == KeyCode::R {}

        match keycode {
            KeyCode::R => {
                self.puzzle = Puzzle::rand_new(PUZZLE_SIZE);
                self.solution = init_solution(PUZZLE_SIZE);
            }
            KeyCode::Escape => quit(ctx),
            _ => {}
        }
    }
}

fn main() {
    let (ctx, eloop) = &mut ContextBuilder::new("picross", "llxy")
        .conf(get_conf())
        .build()
        .unwrap();

    let picross = &mut Picross {
        puzzle: Puzzle::rand_new(PUZZLE_SIZE),
        solution: init_solution(PUZZLE_SIZE),
    };

    event::run(ctx, eloop, picross).unwrap();
}

fn get_conf() -> conf::Conf {
    use ggez::conf::*;

    let ws = WindowSetup {
        title: "Picross.LLXY".to_owned(),
        samples: NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };

    let wm = WindowMode {
        width: 1600.0,
        height: 1000.0,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    };

    let conf = Conf {
        window_mode: wm,
        window_setup: ws,
        backend: Backend::default(),
        modules: ModuleConf::default(),
    };

    let pretty = PrettyConfig::new()
        .with_depth_limit(2)
        .with_separate_tuple_members(true)
        .with_enumerate_arrays(true);
    let s = to_string_pretty(&conf, pretty).expect("Serialization failed");

    let path = Path::new("config.ron");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(s.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    };

    conf
}

fn init_solution(size: usize) -> Vec<Vec<bool>> {
    vec![vec![false; size]; size]
}
