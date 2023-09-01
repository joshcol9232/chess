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

/// PIECE DEFINITIONS
pub struct Pawn { team: bool }

impl Piece for Pawn {
    fn kind(&self) -> PieceKind {
        return PieceKind::Pawn;
    }
    fn team(&self) -> bool { return self.team }
    fn is_valid_move(&self,
                     dxy: [i8; 2],
                     is_occupied: bool) -> bool {
        pawn(dxy, is_occupied, self.team)
    }
}

pub struct Rook { team: bool }

impl Piece for Rook {
    fn kind(&self) -> PieceKind {
        return PieceKind::Rook;
    }
    fn team(&self) -> bool { return self.team }
    fn is_valid_move(&self,
                     dxy: [i8; 2],
                     is_occupied: bool) -> bool {
        rook(dxy, is_occupied, self.team)
    }
}

pub struct Horse { team: bool }

impl Piece for Horse {
    fn kind(&self) -> PieceKind {
        return PieceKind::Horse;
    }
    fn team(&self) -> bool { return self.team }
    fn is_valid_move(&self,
                     dxy: [i8; 2],
                     is_occupied: bool) -> bool {
        horse(dxy, is_occupied, self.team)
    }
}

pub struct Bishop { team: bool }

impl Piece for Bishop {
    fn kind(&self) -> PieceKind {
        return PieceKind::Bishop;
    }
    fn team(&self) -> bool { return self.team }
    fn is_valid_move(&self,
                     dxy: [i8; 2],
                     is_occupied: bool) -> bool {
        bishop(dxy, is_occupied, self.team)
    }
}

pub struct Queen { team: bool }

impl Piece for Queen {
    fn kind(&self) -> PieceKind {
        return PieceKind::Queen;
    }
    fn team(&self) -> bool { return self.team }
    fn is_valid_move(&self,
                     dxy: [i8; 2],
                     is_occupied: bool) -> bool {
        queen(dxy, is_occupied, self.team)
    }
}

pub struct King { team: bool }

impl Piece for King {
    fn kind(&self) -> PieceKind {
        return PieceKind::King;
    }
    fn team(&self) -> bool { return self.team }
    fn is_valid_move(&self,
                     dxy: [i8; 2],
                     is_occupied: bool) -> bool {
        king(dxy, is_occupied, self.team)
    }
}

