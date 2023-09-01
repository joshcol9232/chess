use crate::piece_kind::{PieceKind, PieceDescriptor};
use crate::move_checks::*;

pub trait Piece {
    fn kind(&self) -> PieceKind;
    fn team(&self) -> bool;  // black -> 0, white -> 1
    fn descriptor(&self) -> PieceDescriptor { PieceDescriptor { kind: self.kind(), team: self.team() } }
                                 //
    // let dxy: [i8; 2] = [to[0] as i8 - from[0] as i8, to[1] as i8 - from[1] as i8];
    fn is_valid_move(&self,
                     dxy: [i8; 2],
                     is_occupied: bool) -> bool;
}


// Option<Rc<dyn Piece>>
#[macro_export]
macro_rules! new_piece {
    ($piece_name:expr, $team:expr) => {
        Rc::new( $piece_name($team) )
    };
    // Empty
    () => {
        Rc::new( Empty )
    }
}

/// PIECE DEFINITIONS
pub struct Pawn(pub bool);

impl Piece for Pawn {
    fn kind(&self) -> PieceKind { PieceKind::Pawn }
    fn team(&self) -> bool { self.0 }
    fn is_valid_move(&self,
                     dxy: [i8; 2],
                     is_occupied: bool) -> bool {
        pawn(dxy, is_occupied, self.team())
    }
}

pub struct Rook(pub bool);

impl Piece for Rook {
    fn kind(&self) -> PieceKind { PieceKind::Rook }
    fn team(&self) -> bool { self.0 }
    fn is_valid_move(&self,
                     dxy: [i8; 2],
                     is_occupied: bool) -> bool {
        rook(dxy, is_occupied, self.team())
    }
}

pub struct Horse(pub bool);

impl Piece for Horse {
    fn kind(&self) -> PieceKind { PieceKind::Horse }
    fn team(&self) -> bool { self.0 }
    fn is_valid_move(&self,
                     dxy: [i8; 2],
                     is_occupied: bool) -> bool {
        horse(dxy, is_occupied, self.team())
    }
}

pub struct Bishop(pub bool);

impl Piece for Bishop {
    fn kind(&self) -> PieceKind { PieceKind::Bishop }
    fn team(&self) -> bool { self.0 }
    fn is_valid_move(&self,
                     dxy: [i8; 2],
                     is_occupied: bool) -> bool {
        bishop(dxy, is_occupied, self.team())
    }
}

pub struct Queen(pub bool);

impl Piece for Queen {
    fn kind(&self) -> PieceKind { PieceKind::Queen }
    fn team(&self) -> bool { self.0 }
    fn is_valid_move(&self,
                     dxy: [i8; 2],
                     is_occupied: bool) -> bool {
        queen(dxy, is_occupied, self.team())
    }
}

pub struct King(pub bool);

impl Piece for King {
    fn kind(&self) -> PieceKind { PieceKind::King }
    fn team(&self) -> bool { self.0 }
    fn is_valid_move(&self,
                     dxy: [i8; 2],
                     is_occupied: bool) -> bool {
        king(dxy, is_occupied, self.team())
    }
}

pub struct Empty;
impl Piece for Empty {
    fn kind(&self) -> PieceKind { PieceKind::Empty }
    fn team(&self) -> bool { false }
    fn is_valid_move(&self, dxy: [i8; 2], is_occupied: bool) -> bool { false }
}

