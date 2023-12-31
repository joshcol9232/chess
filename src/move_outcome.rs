use crate::piece_kind::PieceDescriptor;

pub enum MoveOutcome {
    TookPiece(PieceDescriptor),
    Castle,
    JustMove,
}

