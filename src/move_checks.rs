use crate::board_model::BoardModel;

fn dxy(from: [u8; 2], to: [u8; 2]) -> [i8; 2] {
    [to[0] as i8 - from[0] as i8, to[1] as i8 - from[1] as i8]
}

// For when the move is straight
fn check_path_is_clear_straight(board: &BoardModel,
                                from: [u8; 2], to: [u8; 2],
                                dxy: [i8; 2]) -> bool {
    assert!(dxy[0] == 0 || dxy[1] == 0);

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

pub fn pawn(board: &BoardModel, from: [u8; 2], to: [u8; 2], occupied: bool, team: bool, is_first_move: bool) -> bool {
    let dxy = dxy(from, to);
    // if white -> upwards, else downwards
    let good_y_direction = if team { -1_i8 } else { 1_i8 };
    
    // TRY THIS: ( ( occupied && (dxy[0] == 1 || dxy[0] == -1) ) || (dxy[0] == 0) ) && (dxy[1] == good_y_direction)
    //
    let base_movement = (if occupied { dxy[0] == 1 || dxy[0] == -1 } else { dxy[0] == 0 }) &&
        ( dxy[1] == good_y_direction || (is_first_move && dxy[1] == good_y_direction * 2) );

    // Check there's nothing in the way
    if base_movement {
        check_path_is_clear_straight(board, from, to, dxy)
    } else {
        false
    }
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

pub fn bishop(from: [u8; 2], to: [u8; 2], occupied: bool, team: bool) -> bool {
    let dxy = dxy(from, to);
    dxy[0] == dxy[1]
}

pub fn queen(from: [u8; 2], to: [u8; 2], occupied: bool, team: bool) -> bool {
    let dxy = dxy(from, to);
    false
}

pub fn king(from: [u8; 2], to: [u8; 2], occupied: bool, team: bool) -> bool {
    let dxy = dxy(from, to);
    false
}

