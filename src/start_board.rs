#[macro_export]
macro_rules! start_board {
    () => {
[
    new_piece!(Rook, BLACK), new_piece!(Horse, BLACK), new_piece!(Bishop, BLACK), new_piece!(Queen, BLACK), new_piece!(King, BLACK), new_piece!(Bishop, BLACK), new_piece!(Horse, BLACK), new_piece!(Rook, BLACK),
    new_piece!(Pawn, BLACK), new_piece!(Pawn, BLACK), new_piece!(Pawn, BLACK), new_piece!(Pawn, BLACK), new_piece!(Pawn, BLACK), new_piece!(Pawn, BLACK), new_piece!(Pawn, BLACK), new_piece!(Pawn, BLACK),
    new_piece!(), new_piece!(), new_piece!(), new_piece!(), new_piece!(), new_piece!(), new_piece!(), new_piece!(),
    new_piece!(), new_piece!(), new_piece!(), new_piece!(), new_piece!(), new_piece!(), new_piece!(), new_piece!(),
    new_piece!(), new_piece!(), new_piece!(), new_piece!(), new_piece!(), new_piece!(), new_piece!(), new_piece!(),
    new_piece!(), new_piece!(), new_piece!(), new_piece!(), new_piece!(), new_piece!(), new_piece!(), new_piece!(),
    new_piece!(Pawn, WHITE), new_piece!(Pawn, WHITE), new_piece!(Pawn, WHITE), new_piece!(Pawn, WHITE), new_piece!(Pawn, WHITE), new_piece!(Pawn, WHITE), new_piece!(Pawn, WHITE), new_piece!(Pawn, WHITE),
    new_piece!(Rook, WHITE), new_piece!(Horse, WHITE), new_piece!(Bishop, WHITE), new_piece!(Queen, WHITE), new_piece!(King, WHITE), new_piece!(Bishop, WHITE), new_piece!(Horse, WHITE), new_piece!(Rook, WHITE)
]
    }
}

