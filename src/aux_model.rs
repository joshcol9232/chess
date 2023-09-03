use crate::board_model;
use crate::board_array::BoardArray;

use std::collections::HashMap;

pub struct AuxModel {
    // Display states
    states: BoardArray<Option<SquareState>>,
    last_selected: Option<[u8; 2]>,
}

impl AuxModel {
    pub fn new() -> Self {
        Self {
            states: [None; 8 * 8].into(),
            last_selected: None
        }
    }

    pub fn set_state(&mut self, xy: [u8; 2], state: SquareState) {
        self.states[xy] = Some(state);

        if let Some(last_pos) = self.last_selected {
            if last_pos != xy {
                self.states[last_pos] = None;
            }
        }
        self.last_selected = Some(xy);
    }

    pub fn get_state(&self, xy: [u8; 2]) -> Option<SquareState> {
        self.states[xy]
    }

    pub fn clear_states(&mut self) {
        for s in self.states.iter_mut() { *s = None; }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SquareState {
    ValidMove,    // Green :)
    InvalidMove,  // Red :(
    IsSelected,   // Blue
}

