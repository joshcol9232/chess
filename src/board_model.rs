use std::rc::Rc;
use std::ops::Index;

use crate::piece::*;
use crate::piece_kind::{PieceKind, PieceDescriptor};
use crate::consts::{BLACK, WHITE};
use crate::{start_board, new_piece};

fn x_y_to_board_idx(x_y: [u8; 2]) -> usize {
    //  0  1  2  3  4  5  6  7  8
    //  9 10 11 ...
    (x_y[0] + x_y[1] * 8) as usize
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

    pub fn descriptor_at(&self, x_y: [u8; 2]) -> PieceDescriptor {
        self[x_y].descriptor()
    }
    pub fn kind_at(&self, x_y: [u8; 2]) -> PieceKind {
        self[x_y].kind()
    }
}

impl Index<[u8; 2]> for BoardModel {
    type Output = Rc<dyn Piece>;

    fn index(&self, idx: [u8; 2]) -> &Self::Output {
        &self.board[x_y_to_board_idx(idx)]
    }
}
