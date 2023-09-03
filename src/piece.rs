use crate::piece_kind::{PieceKind, PieceDescriptor};
use crate::move_checks::*;
use crate::board_model::BoardModel;

pub trait Piece {
    fn kind(&self) -> PieceKind;
    fn team(&self) -> bool;  // black -> 0, white -> 1
    fn descriptor(&self) -> PieceDescriptor { PieceDescriptor { kind: self.kind(), team: self.team() } }

    fn register_first_move(&mut self) {}
    fn is_valid_move(&self,
                     board: &BoardModel,
                     from: [u8; 2],
                     to: [u8; 2],
                     is_occupied: bool) -> bool;
}

#[macro_export]
macro_rules! new_piece {
    ($piece_name:ty, $team:expr) => {
        Rc::new(RefCell::new( <$piece_name>::new($team) ))
    };
    // Empty
    () => {
        Rc::new(RefCell::new( Empty ))
    }
}

/// PIECE DEFINITIONS
#[derive(new)]
pub struct Pawn {
    team: bool,
    #[new(value = "true")]
    first_move: bool,
}

impl Piece for Pawn {
    fn kind(&self) -> PieceKind { PieceKind::Pawn }
    fn team(&self) -> bool { self.team }
    fn is_valid_move(&self,
                     board: &BoardModel,
                     from: [u8; 2],
                     to: [u8; 2],
                     is_occupied: bool) -> bool {
        pawn(board, from, to, is_occupied, self.team(), self.first_move)
    }
    fn register_first_move(&mut self) { self.first_move = false; }
}

#[derive(new)]
pub struct Rook(pub bool);

impl Piece for Rook {
    fn kind(&self) -> PieceKind { PieceKind::Rook }
    fn team(&self) -> bool { self.0 }
    fn is_valid_move(&self,
                     board: &BoardModel,
                     from: [u8; 2],
                     to: [u8; 2],
                     is_occupied: bool) -> bool {
        rook(board, from, to, is_occupied, self.team())
    }
}

#[derive(new)]
pub struct Horse(pub bool);

impl Piece for Horse {
    fn kind(&self) -> PieceKind { PieceKind::Horse }
    fn team(&self) -> bool { self.0 }
    fn is_valid_move(&self,
                     board: &BoardModel,
                     from: [u8; 2],
                     to: [u8; 2],
                     is_occupied: bool) -> bool {
        horse(from, to, is_occupied, self.team())
    }
}

#[derive(new)]
pub struct Bishop(pub bool);

impl Piece for Bishop {
    fn kind(&self) -> PieceKind { PieceKind::Bishop }
    fn team(&self) -> bool { self.0 }
    fn is_valid_move(&self,
                     board: &BoardModel,
                     from: [u8; 2],
                     to: [u8; 2],
                     is_occupied: bool) -> bool {
        bishop(from, to, is_occupied, self.team())
    }
}

#[derive(new)]
pub struct Queen(pub bool);

impl Piece for Queen {
    fn kind(&self) -> PieceKind { PieceKind::Queen }
    fn team(&self) -> bool { self.0 }
    fn is_valid_move(&self,
                     board: &BoardModel,
                     from: [u8; 2],
                     to: [u8; 2],
                     is_occupied: bool) -> bool {
        queen(from, to, is_occupied, self.team())
    }
}

#[derive(new)]
pub struct King(pub bool);

impl Piece for King {
    fn kind(&self) -> PieceKind { PieceKind::King }
    fn team(&self) -> bool { self.0 }
    fn is_valid_move(&self,
                     board: &BoardModel,
                     from: [u8; 2],
                     to: [u8; 2],
                     is_occupied: bool) -> bool {
        king(from, to, is_occupied, self.team())
    }
}

#[derive(new)]
pub struct Empty;
impl Piece for Empty {
    fn kind(&self) -> PieceKind { PieceKind::Empty }
    fn team(&self) -> bool { false }
    fn is_valid_move(&self,
                     board: &BoardModel,
                     from: [u8; 2],
                     to: [u8; 2],
                     is_occupied: bool) -> bool { false }
}

