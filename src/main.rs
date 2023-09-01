mod consts;
mod move_checks;
mod piece_kind;
mod piece;

mod start_board;
mod board_view;
mod board_model;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use piece_kind::PieceKind;
use board_view::BoardView;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend
    board_view: BoardView,
}

impl App {
    fn new(gl: GlGraphics) -> Result<Self, String> {
        let board_view = BoardView::new()?;
        Ok(Self {
            gl,
            board_view,
        })
    }

    fn render(&mut self, args: &RenderArgs) {
        self.board_view.render(&mut self.gl, args);
    }

    fn update(&mut self, args: &UpdateArgs) {
        // TODO: Game logic
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Chess", consts::WINDOW_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(GlGraphics::new(opengl))
                      .expect("Couldn't initialise app ->");

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
