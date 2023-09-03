use crate::board_model::BoardModel;
use crate::piece_kind::PieceKind;

fn dxy(from: [u8; 2], to: [u8; 2]) -> [i8; 2] {
    [to[0] as i8 - from[0] as i8, to[1] as i8 - from[1] as i8]
}

// For when the move is straight
fn check_path_is_clear_straight(board: &BoardModel,
                                from: [u8; 2], to: [u8; 2],
                                dxy: [i8; 2]) -> bool {
    assert!(dxy[0] == 0 || dxy[1] == 0);

    // TODO: Might be need optimising.
    println!("dxy: {:?}", dxy);
    if dxy[0] == 0 {
        for yy in 1..dxy[1].abs() {
            let spot = [from[0], (from[1] as i8 + yy * dxy[1].signum()) as u8];
            println!("Checking: {:?}", spot);
            if board.is_occupied(spot) {
                return false;
            }
        }
    } else {
        for xx in 1..dxy[0].abs() {
            let spot = [(from[0] as i8 + xx * dxy[0].signum()) as u8, from[1]];
            println!("Checking: {:?}", spot);
            if board.is_occupied(spot) {
                return false;
            }
        }
    }

    true
}

// Requires that dxy is diagonal
fn check_path_is_clear_diagonal(board: &BoardModel,
                                from: [u8; 2], to: [u8; 2],
                                dxy: [i8; 2]) -> bool {
    assert!(dxy[0].abs() == dxy[1].abs());

    for step in 1..dxy[0].abs() {
        let spot = [ (from[0] as i8 + step * dxy[0].signum()) as u8, (from[1] as i8 + step * dxy[1].signum()) as u8 ];
        println!("diag> Checking: {:?}", spot);
        if board.is_occupied(spot) {
            return false;
        }
    }
    true
}

pub fn pawn(board: &BoardModel, from: [u8; 2], to: [u8; 2], occupied: bool, team: bool, is_first_move: bool) -> bool {
    let dxy = dxy(from, to);
    // if white -> upwards, else downwards
    let good_y_direction = if team { -1_i8 } else { 1_i8 };
    
    // TRY THIS: ( ( occupied && (dxy[0] == 1 || dxy[0] == -1) ) || (dxy[0] == 0) ) && (dxy[1] == good_y_direction)
    //
    (if occupied { dxy[0] == 1 || dxy[0] == -1 } else { dxy[0] == 0 }) &&
        ( (is_first_move && dxy[1] == good_y_direction * 2 && check_path_is_clear_straight(board, from, to, dxy)) ^ (dxy[1] == good_y_direction) )
}

pub fn rook(board: &BoardModel, from: [u8; 2], to: [u8; 2], occupied: bool, team: bool) -> bool {
    let dxy = dxy(from, to);
    // Exit early if it's not along a straight line
    let base_movement = (dxy[0] == 0) ^ (dxy[1] == 0);

    if base_movement {
        check_path_is_clear_straight(board, from, to, dxy)
    } else {
        false
    }
}

pub fn horse(from: [u8; 2], to: [u8; 2], occupied: bool, team: bool) -> bool {
    /*
     *  _
     * |
     * |
     *
     * _
     *  |
     *  |
     *
     *  __|
     *
     *  __
     *    |
     *
     *  |__
     *
     *   __
     *  |
     *
     *  |
     *  |_
     *
     *   |
     *  _|
     *
    */
    const LOOKUP: [[i8; 2]; 8] = [
        [1, 2],
        [-1, 2],
        [2, 1],
        [2, -1],
        [-2, 1],
        [-2, -1],
        [1, -2],
        [-1, -2]
    ];

    let dxy = dxy(from, to);

    for good_dxy in LOOKUP.iter() {
        if dxy == *good_dxy {
            return true;
        }
    }
    false
}

pub fn bishop(board: &BoardModel, from: [u8; 2], to: [u8; 2], occupied: bool, team: bool) -> bool {
    let dxy = dxy(from, to);
    dxy[0].abs() == dxy[1].abs() && check_path_is_clear_diagonal(board, from, to, dxy)
}

pub fn queen(board: &BoardModel, from: [u8; 2], to: [u8; 2], occupied: bool, team: bool) -> bool {
    rook(board, from, to, occupied, team) || bishop(board, from, to, occupied, team)
}

pub fn king(board: &BoardModel, from: [u8; 2], to: [u8; 2], occupied: bool, team: bool) -> bool {
    let dxy = dxy(from, to);
    // IF first move, and in the castle position with rook in the corner, then castle :)
    if dxy[0].abs() == 2 && (from[1] == 0 || from[1] == 7) {  // Castle?
        let rook_pos_needed = [to[0] + 1, to[1]];
        println!("Rook pos needed: {:?}", rook_pos_needed);
        board.kind_at(rook_pos_needed) == PieceKind::Rook
    } else {
        (dxy[0].abs() <= 1 && dxy[1].abs() <= 1) && queen(board, from, to, occupied, team)
    }
}

