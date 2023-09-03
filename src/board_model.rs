use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Index, IndexMut};

use crate::board_array::BoardArray;
use crate::piece::*;
use crate::piece_kind::{PieceKind, PieceDescriptor};
use crate::move_outcome::MoveOutcome;
use crate::consts::{BLACK, WHITE};
use crate::{start_board, new_piece};


pub struct BoardModel {
    board: BoardArray<Rc<RefCell<dyn Piece>>>,
}

impl BoardModel {
    pub fn new() -> Self {
        Self {
            // Default board setup
            board: BoardArray::new(start_board!()),
        }
    }

    pub fn descriptor_at(&self, xy: [u8; 2]) -> PieceDescriptor {
        self[xy].borrow().descriptor()
    }
    pub fn kind_at(&self, xy: [u8; 2]) -> PieceKind {
        self[xy].borrow().kind()
    }

    pub fn is_occupied(&self, xy: [u8; 2]) -> bool {
        self.kind_at(xy) != PieceKind::Empty
    }

    pub fn is_valid_move(&self, from: [u8; 2], to: [u8; 2]) -> bool {
        // Get diff
        let is_occupied = self.is_occupied(to);

         // First of all, if the `to` space is occupied, check it is of the other team.
        if is_occupied && self[to].borrow().team() == self[from].borrow().team() {
            return false;
        }
        // Then check the piece movement is correct
        if !self[from].borrow().is_valid_move(&self, from, to, is_occupied) {
            return false;
        }

        true
    }

    /// \brief Attempt to move a piece. Returns `None` if the move is invalid.
    pub fn move_piece(&mut self, from: [u8; 2], to: [u8; 2]) -> Option<MoveOutcome> {
        if !self.is_valid_move(from, to) {
            return None;
        }
               
        // Otherwise, valid move so continue :)
        let mut castling = false;

        println!("Moving: {:?} -> {:?}", from, to);
        let result = if self[to].borrow().kind() != PieceKind::Empty {
            Some(MoveOutcome::TookPiece(self[to].borrow().descriptor()))
        } else if self.kind_at(from) == PieceKind::King && (to[0] as i8 - from[0] as i8).abs() == 2 {  // Special case for castling
            castling = true;
            Some(MoveOutcome::Castle)
        } else {
            Some(MoveOutcome::JustMove)
        };

        self[from].borrow_mut().register_first_move();  // If a pawn, then it needs to know if at least one move
                                                        // has happened.

        // Swap and make old square empty.
        self[to] = Rc::clone(&self[from]);
        self[from] = new_piece!();  // Set old space empty.
        
        if castling {
            let rook_pos = [to[0] + 1, to[1]];
            let new_rook_pos = [to[0] - 1, to[1]];
            self[new_rook_pos] = Rc::clone(&self[rook_pos]);
            self[rook_pos] = new_piece!();  // Empty rook spot
        }
        
        result
    }
}

impl Index<[u8; 2]> for BoardModel {
    type Output = Rc<RefCell<dyn Piece>>;

    fn index(&self, idx: [u8; 2]) -> &Self::Output {
        &self.board[idx]
    }
}

impl IndexMut<[u8; 2]> for BoardModel {
    fn index_mut(&mut self, idx: [u8; 2]) -> &mut Self::Output {
        &mut self.board[idx]
    }
}

