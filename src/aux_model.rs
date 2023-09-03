use crate::board_model;
use crate::board_array::BoardArray;

use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Highlight {
    pos: [u8; 2],
    state: SquareState,
}

pub struct AuxModel {
    // Display states
    hovering: Option<Highlight>,
    currently_selected: Option<[u8; 2]>,
}

impl AuxModel {
    pub fn new() -> Self {
        Self {
            hovering: None,
            currently_selected: None,
        }
    }

    pub fn set_state(&mut self, xy: [u8; 2], state: SquareState) {
        match state {
            SquareState::IsSelected => {
                self.currently_selected = Some(xy);
            }
            s => {
                self.hovering = Some(Highlight { pos: xy, state: s });
            }
        }
    }

    pub fn get_state(&self, xy: [u8; 2]) -> Option<SquareState> {
        if let Some(selected) = self.currently_selected {
            if selected == xy {
                return Some(SquareState::IsSelected);
            }
        }

        if let Some(highlight) = self.hovering {
            if highlight.pos == xy {
                return Some(highlight.state);
            }
        }
        None
    }

    pub fn clear_hovering(&mut self) {
        self.hovering = None;
    }
    pub fn clear_selected(&mut self) {
        self.currently_selected = None;
    }
    pub fn clear(&mut self) {
        self.clear_hovering();
        self.clear_selected();
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SquareState {
    ValidMove,    // Green :)
    InvalidMove,  // Red :(
    IsSelected,   // Blue
}

