use piston::input::{MouseButton};

use crate::consts::WINDOW_SIZE_F64;
use crate::board_model::{self, BoardModel};
use crate::aux_model::{self, AuxModel};
use crate::move_outcome::MoveOutcome;

pub struct BoardController {
    mouse_pos: [f64; 2],
    selected_to_move: Option<[u8; 2]>,
}

impl BoardController {
    pub fn new() -> Self {
        Self {
            mouse_pos: [0.0, 0.0],
            selected_to_move: None,
        }
    }

    fn set_selected(&mut self, pos: [u8; 2], aux_model: &mut AuxModel) {
        self.selected_to_move = Some(pos);
        aux_model.set_state(pos, aux_model::SquareState::IsSelected);
    }

    pub fn set_mouse_pos(&mut self, new: [f64; 2], model: &BoardModel, aux_model: &mut AuxModel) {
        self.mouse_pos = new;
        // TODO: Could show if the mouse is hovering over a good or bad square.

        if let Some(selected_piece) = self.selected_to_move {
            let board_pos = mouse_pos_to_board_pos(&new);
            aux_model.set_state(board_pos,
                                if model.is_valid_move(selected_piece, board_pos) {
                                    aux_model::SquareState::ValidMove
                                } else {
                                    aux_model::SquareState::InvalidMove
                                });
        } else {
            aux_model.clear_states();
        }
    }

    fn process_move_outcome(&mut self, outcome: MoveOutcome) {

    }

    pub fn process_mouse_button(&mut self, button: &MouseButton, model: &mut BoardModel) {
        match button {
            MouseButton::Left => {
                if let Some(to_move) = self.selected_to_move {
                    let new_pos = mouse_pos_to_board_pos(&self.mouse_pos);
                    
                    // If player selects same piece, cancel move
                    if new_pos == to_move {
                        self.selected_to_move = None;
                    } else if let Some(outcome) = model.move_piece(to_move, new_pos){
                        // Otherwise, actually move the piece
                        self.selected_to_move = None;
                        self.process_move_outcome(outcome);
                    }

                    // Else, move is not valid, keep the selected to move.
                } else {
                    let select_pos = mouse_pos_to_board_pos(&self.mouse_pos);
                    if model.is_occupied(select_pos) {
                        println!("Selected: {:?}", select_pos);
                        self.selected_to_move = Some(select_pos);
                    }
                }
            }
            MouseButton::Right => {
                self.selected_to_move = None;
            }
            _ => ()
        }
    }


}

/// \brief Converts [x, y] screen coordinates to board positions
/// \detail Converts to space [0.0, 8.0), then floor rounds to get grid index.
fn mouse_pos_to_board_pos(mouse_pos: &[f64; 2]) -> [u8; 2] {
    [( (mouse_pos[0] / WINDOW_SIZE_F64[0]) * 8.0 ).floor() as u8,
     ( (mouse_pos[1] / WINDOW_SIZE_F64[1]) * 8.0 ).floor() as u8]
}

