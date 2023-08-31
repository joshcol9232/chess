use piece_kind::PieceKind;

pub trait Piece {
    fn new() -> Self;
    /// \brief Return the enum identifier
    fn type() -> PieceKind;
    /// \brief Check that a move is valid for this piece
    fn valid_move(from: [u8; 2], to: [u8; 2]) -> bool;
}


