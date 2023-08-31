use crate::move_checks::*;

pub enum PieceKind {
    Pawn,
    Rook,
    Horse,
    Bishop,
    Queen,
    King
}

/// \brief Checks a move is valid for a given kind.
///        `is_occupied` -> Flag to say if the `to` spot is currently occupied. This sometimes
///        `team` -> 0 = black, 1 = white
///        matters e.g for pawns.
pub fn is_valid_move(kind: &PieceKind,
                     from: &[u8; 2],
                     to: &[u8; 2],
                     team: bool,
                     is_occupied: bool) -> bool
{
    let dxy: [i8; 2] = [to[0] as i8 - from[0] as i8, to[1] as i8 - from[1] as i8];

    match kind {
        PieceKind::Pawn => pawn(dxy, is_occupied, team),
        PieceKind::Rook => rook(dxy, is_occupied, team),
        PieceKind::Horse => horse(dxy, is_occupied, team),
        PieceKind::Bishop => bishop(dxy, is_occupied, team),
        PieceKind::Queen => queen(dxy, is_occupied, team),
        PieceKind::King => king(dxy, is_occupied, team),
    }
}

