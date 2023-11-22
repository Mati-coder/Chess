use std::cmp::max;

// Signed integers for easy distance computation, not meant to ever hold negative values
#[derive(Eq, PartialEq)]
struct Coord {x: i8, y: i8} 

enum Color {
    Black,
    White,
}

enum PieceType {
    King,
    Queen,
    Knight,
    Bishop,
    Pawn,
    Rook,
}

struct Piece {
    kind: PieceType,
    color: Color, 
    board: Board,
    pos: Coord,
}

impl Piece {
    //Done
    // Works for non-special moves only
    fn valid_move(&self, target) -> bool {
        if self.pos == target {return false}

        let deltax = target.x - self.pos.x;
        let deltay = target.y - self.pos.y;
        match self.kind {
            PieceType::King => deltax < 2 && deltay < 2,
            PieceType::Pawn => {
                if deltax != 0 || deltay.abs() > 1 {return false}
                match self.color {
                    Color::Black => deltax.signum() == -1,
                    Color::White => deltay.signum() == 1,
                }
            },
            PieceType::Queen => deltax.abs() == deltay.abs() || deltax == 0 || deltay ==0,
            PieceType::Rook => deltax == 0 || deltay == 0,
            PieceType::Bishop => deltax.abs() == deltay.abs(),
            PieceType::Knight => true,
        }
    }

    // Done
    // Doesn't check the actual target
    fn path_clear(&self, target: Coord) -> bool {
        // It's important to note that we assume the move is valid
        if let Knight = self.kind {
            true
        } else {
            // Under the previous assumption this method works for every piece
            let delta_x = target.x - self.pos.x;
            let delta_y = target.y - self.pos.y;
            let sign_dx = delta_x.signum();
            let sign_dy = delta_y.signum();
            for n in 1..=max(delta_x.abs(), delta_y.abs()) {
                if !self.board.is_empty(Coord{
                    x: self.pos.x + n * sign_dx, 
                    y: self.pos.y + n * sign_dy}) 
                {return false}
            }
            true
        }
    }
    
    //TODO
    // Check whether the piece can make a movement
    fn can_move(&self, target: Coord) -> Result<(), String> {
        // We don't need to check if pos is in the board because it is already checked before
        if self.special_cases(target) {return Ok()}
        if !self.valid_move(target) {return Err(String::from("Invalid move"))}
        if !self.path_clear(target) {return Err(String::from("There is a piece in the way"))}
        Ok(())
    }

    //TODO
    // Actually moves the piece (doesn't perform any checks)
    fn move_to(&self, target: Coord) -> Result<(), String> {        
        self.pos = target;
        
        Ok(())
    }
}