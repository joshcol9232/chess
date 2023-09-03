mod consts;
mod move_checks;
mod piece_kind;
mod piece;
mod move_outcome;

mod start_board;
mod board_view;
mod board_array;
mod board_model;
mod board_controller;
mod aux_model;

#[macro_use]
extern crate derive_new;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent,
                    UpdateArgs, UpdateEvent,
                    ButtonArgs, ButtonEvent,
                    MouseCursorEvent,
                    Button, ButtonState};
use piston::window::WindowSettings;

use std::rc::Rc;
use std::cell::RefCell;

use piece_kind::PieceKind;
use board_view::BoardView;
use board_model::BoardModel;
use aux_model::AuxModel;
use board_controller::BoardController;


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend
    board_view: BoardView,
    board_model: Rc<RefCell<BoardModel>>,
    auxiliary_model: Rc<RefCell<AuxModel>>,     // Model containing information for rendering
    board_controller: BoardController,
}

impl App {
    fn new(gl: GlGraphics) -> Result<Self, String> {
        let board_view = BoardView::new()?;
        Ok(Self {
            gl,
            board_view,
            board_model: Rc::new(RefCell::new(BoardModel::new())),
            auxiliary_model: Rc::new(RefCell::new(AuxModel::new())),
            board_controller: BoardController::new(),
        })
    }

    fn render(&mut self, args: &RenderArgs) {
        self.board_view.render(&mut self.gl, args, &self.board_model.borrow(), &self.auxiliary_model.borrow());
    }

    fn update(&mut self, args: &UpdateArgs) {
        // TODO: Game logic
    }

    fn handle_button(&mut self, args: &ButtonArgs) {
        match args {
            ButtonArgs { button: Button::Mouse(mouse_button), state: ButtonState::Press, .. } => {
                self.board_controller.process_mouse_button(mouse_button,
                                                           &mut self.board_model.borrow_mut());
            }
            _ => ()
        }
    }

    fn handle_mouse_cursor(&mut self, new_xy: [f64; 2]) {
        self.board_controller.set_mouse_pos(new_xy,
                                            &self.board_model.borrow(),
                                            &mut self.auxiliary_model.borrow_mut());
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Chess", consts::WINDOW_SIZE)
        .graphics_api(opengl)
        .resizable(false)
        .samples(0)
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

        if let Some(args) = e.button_args() {
            app.handle_button(&args);
        }

        if let Some(pos) = e.mouse_cursor_args() {
            app.handle_mouse_cursor(pos);
        }
    }
}
