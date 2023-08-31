
pub fn pawn(dxy: [i8; 2], occupied: bool, team: bool) -> bool {
    // if white -> upwards, else downwards
    let good_y_direction = if team { -1_i8 } else { 1_i8 };
    
    // TRY THIS: ( ( occupied && (dxy[0] == 1 || dxy[0] == -1) ) || (dxy[0] == 0) ) && (dxy[1] == good_y_direction)
    (if occupied { dxy[0] == 1 || dxy[0] == -1 } else { dxy[0] == 0 }) && (dxy[1] == good_y_direction)
}

pub fn rook(dxy: [i8; 2], occupied: bool, team: bool) -> bool {
    false
}

pub fn horse(dxy: [i8; 2], occupied: bool, team: bool) -> bool {
    false
}

pub fn bishop(dxy: [i8; 2], occupied: bool, team: bool) -> bool {
    false
}

pub fn queen(dxy: [i8; 2], occupied: bool, team: bool) -> bool {
    false
}

pub fn king(dxy: [i8; 2], occupied: bool, team: bool) -> bool {
    false
}

