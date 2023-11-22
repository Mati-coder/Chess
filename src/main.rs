use std::cmp::max;

// Signed integers for easy difference computation, not meant to be negative
struct Coord {x: i8, y: i8} 

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
    board: Board,
    pos: Coord,
}

impl Piece {
    // Done
    fn path_clear(&self, target: Coord) -> bool {
        // It's important to note that we assume the move is valid
        if let Knight = self.kind {
            self.board.isEmpty(target)
        } else {
            // Under the previous assumption this method works for every piece
            let deltax = self.pos.x - target.x;
            let deltay = self.pos.y - target.y;
            let signx = deltax.signum();
            let signy = deltay.signum();
            for n in 1..=max(deltax.abs(), deltay.abs()) {
                if !self.board.is_empty(Coord{
                    x: self.pos.x + n * signx, 
                    y: self.pos.y + n * signy}) 
                {return false}
            }
            true
        }
    }
    
    //TODO
    // Check whether the piece can make a movement
    fn can_move(&self, target: Coord) -> Result<(), String> {
        // We don't need to check if pos is in the board because it is already checked before
        if !self.path_clear(target) {return Err(String::from("There is a piece in the way"))}
    }

    //TODO3

    // Actually moves the piece
    fn move_to(&self, target: Coord) -> Result<(), String> {        
        if let Err(err) = self.can_move(target) { return Err(err) }
        Ok(())
    }
}