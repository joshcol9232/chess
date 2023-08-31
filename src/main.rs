use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod consts;
mod move_checks;
mod piece_kind;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
        });

        let square_size = args.window_size[0] as f64 / 8.0;
        let square = rectangle::square(0.0, 0.0, square_size);

        // Render board background
        for x_idx in 0..8 {
            for y_idx in 0..8 {
                let (x, y) = (square_size * x_idx as f64, square_size * y_idx as f64);
                let mul: bool = ((x_idx + y_idx) % 2) != 1;
                let color = [0.2 + mul as u8 as f32 * 0.8, 0.2 + mul as u8 as f32 * 0.6, 0.2, 1.0];

                self.gl.draw(args.viewport(), |c, gl| {
                    let transform = c
                        .transform
                        .trans(x, y);
                    rectangle(color, square, transform, gl);
                });
            }
        }

        // Render pieces

    }

    fn update(&mut self, args: &UpdateArgs) {
        // TODO: Game logic
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("spinning-square", consts::WINDOW_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
