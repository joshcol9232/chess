use std::rc::Rc;
use std::ops::Index;

use crate::piece::*;
use crate::consts::{BLACK, WHITE};
use crate::{start_board, new_piece};

fn x_y_to_board_idx(x_y: [u8; 2]) -> u8 {
    //  0  1  2  3  4  5  6  7  8
    //  9 10 11 ...
    x_y[0] + x_y[1] * 8
}

// ----------------------------

pub struct BoardModel {
    board: [Rc<dyn Piece>; 8 * 8],
}

impl BoardModel {
    pub fn new() -> Self {
        Self {
            // Default board setup
            board: start_board!(),
        }
    }

}

impl Index<[u8; 2]> for BoardModel {
    type Output = Rc<dyn Piece>;

    fn index(&self, idx: [u8; 2]) -> &Self::Output {
        &self.board[x_y_to_board_idx(idx) as usize]
    }
}

