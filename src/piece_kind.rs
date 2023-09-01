#[derive(PartialEq, Eq, Hash)]
pub enum PieceKind {
    Pawn,
    Rook,
    Horse,
    Bishop,
    Queen,
    King
}

#[derive(PartialEq, Eq, Hash)]
pub struct PieceDescriptor {
    pub kind: PieceKind,
    pub team: bool
}

