use std::cmp::max;


use crate::PieceType::Rook;
use crate::PieceType::Bishop;
use crate::PieceType::Queen;
use crate::PieceType::King;
use crate::PieceType::Knight;
use crate::PieceType::Empty;
use crate::PieceType::Pawn;

use crate::Color::White;
use crate::Color::Black;
// Signed integers for easy distance computation, not meant to ever hold negative values

#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
struct Coord {x: i8, y: i8} 

struct Board {
    board: [[(PieceType, Color); 8]; 8],
    pieces: Vec<Piece>,
}

#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
enum Color {
    Black,
    White,
}

#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
enum PieceType {
    King,
    Queen,
    Knight,
    Bishop,
    Pawn,
    Rook,
    Empty, //Used for empty spaces on the board
}

struct Piece {
    kind: PieceType,
    color: Color, 
    pos: Coord,
}

impl Piece{
    //Done
    // Works for non-special moves only
    fn valid_move(&self, target: &Coord) -> bool {
        if self.pos == *target {return false}

        let delta_x = target.x - self.pos.x;
        let delta_y = target.y - self.pos.y;
        match self.kind {
            King => (delta_x*delta_y).abs() == 1,
            Pawn => {
                if delta_x != 0 || delta_y.abs() > 1 {return false}
                match self.color {
                    Color::Black => delta_x.signum() == -1,
                    Color::White => delta_y.signum() == 1,
                }
            },
            Queen => delta_x.abs() == delta_y.abs() || delta_x*delta_y == 0,
            Rook => delta_x * delta_y == 0,
            Bishop => delta_x.abs() == delta_y.abs(),
            Knight => delta_x < 2 && delta_y < 2,
            Empty => false
        }
    }

    // Done
    // Doesn't check the actual target
    fn path_clear(&self, target: &Coord, board: &mut Board) -> bool {
        // It's important to note that we assume the move is valid
        
        match self.kind {
            Knight => true,
            Empty => false,
            // Under the previous assumption this method works for every piece
            _ => {
                let delta_x = target.x - self.pos.x;
                let delta_y = target.y - self.pos.y;
                let sign_dx = delta_x.signum();
                let sign_dy = delta_y.signum();
                for n in 1..=max(delta_x.abs(), delta_y.abs()) {
                    if !board.is_empty(&Coord{
                        x: self.pos.x + n * sign_dx, 
                        y: self.pos.y + n * sign_dy}) 
                    {return false}
                }
                true
            }
        }
    }
    
    //TODO
    // Check whether the piece can make a movement
    fn can_move(&self, target: &Coord, board: &mut Board) -> Result<(), String> {
        // We don't need to check if pos is in the board because it is already checked before
        if self.special_cases(&target) {return Ok(())}
        if !self.valid_move(&target) {return Err(String::from("Invalid move"))}
        if !self.path_clear(&target, board) {return Err(String::from("There is a piece in the way"))}
        Ok(())
    }

    //TODO
    // Return true if the move is a special case
    fn special_cases(&self, _target: &Coord) -> bool {
        false
    }

    //Done
    // Actually moves the piece (doesn't perform any checks)
    fn move_to(&mut self, target: &Coord, board: &mut Board) -> Result<(), String> {
        board.change(&self.pos, &PieceType::Empty, &self.color);
        board.change(target, &self.kind, &self.color);
        self.pos = *target;
        Ok(())
    }
}

impl Board {
    //Done
    fn is_empty(&self, target: &Coord) -> bool {
        self.board[target.x as usize][target.y as usize].0 == PieceType::Empty
    }

    // Done
    fn change(&mut self, pos: &Coord, new: &PieceType, color: &Color) {
        self.board[pos.x as usize][pos.y as usize] = (*new, *color);
    }

    fn new() -> Self {
        // Inverted over the Y axis
        let board: [[(PieceType, Color); 8]; 8] = [
            [(Rook, White), (Knight, White), (Bishop, White), (Queen, White), (King, White), (Bishop, White), (Knight, White), (Rook, White)],
            [(Pawn, White), (Pawn, White), (Pawn, White), (Pawn, White), (Pawn, White), (Pawn, White), (Pawn, White), (Pawn, White)],
            [(Empty, White), (Empty, White), (Empty, White), (Empty, White), (Empty, White), (Empty, White), (Empty, White), (Empty, White)],
            [(Empty, White), (Empty, White), (Empty, White), (Empty, White), (Empty, White), (Empty, White), (Empty, White), (Empty, White)],
            [(Empty, Black), (Empty, Black), (Empty, Black), (Empty, Black), (Empty, Black), (Empty, Black), (Empty, Black), (Empty, Black)],
            [(Empty, Black), (Empty, Black), (Empty, Black), (Empty, Black), (Empty, Black), (Empty, Black), (Empty, Black), (Empty, Black)],
            [(Pawn, Black), (Pawn, Black), (Pawn, Black), (Pawn, Black), (Pawn, Black), (Pawn, Black), (Pawn, Black), (Pawn, Black)],
            [(Rook, Black), (Knight, Black), (Bishop, Black), (Queen, Black), (King, Black), (Bishop, Black), (Knight, Black), (Rook, Black)]];

        let pieces: Vec<Piece> = vec![];
        let mut instance: Board = Board{board: board, pieces};
        instance.pieces.push(Piece {kind: Rook, color: White, pos: Coord{x:0, y:0}});
        instance.pieces.push(Piece {kind: Rook, color: Black, pos: Coord{x:0, y:7}});
        instance.pieces.push(Piece {kind: Knight, color: White, pos: Coord{x:1, y:0}});
        instance.pieces.push(Piece {kind: Knight, color: Black, pos: Coord{x:1, y:7}});
        instance.pieces.push(Piece {kind: Bishop, color: White, pos: Coord{x:2, y:0}});
        instance.pieces.push(Piece {kind: Bishop, color: Black, pos: Coord{x:2, y:7}});
        instance.pieces.push(Piece {kind: Queen, color: White, pos: Coord{x:3, y:0}});
        instance.pieces.push(Piece {kind: Queen, color: Black, pos: Coord{x:3, y:7}});
        instance.pieces.push(Piece {kind: King, color: White, pos: Coord{x:4, y:0}});
        instance.pieces.push(Piece {kind: King, color: Black, pos: Coord{x:4, y:7}});
        instance.pieces.push(Piece {kind: Bishop, color: White, pos: Coord{x:5, y:0}});
        instance.pieces.push(Piece {kind: Bishop, color: Black, pos: Coord{x:5, y:7}});
        instance.pieces.push(Piece {kind: Knight, color: White, pos: Coord{x:6, y:0}});
        instance.pieces.push(Piece {kind: Knight, color: Black, pos: Coord{x:6, y:7}});
        instance.pieces.push(Piece {kind: Rook, color: White, pos: Coord{x:7, y:0}});
        instance.pieces.push(Piece {kind: Rook, color: Black, pos: Coord{x:7, y:7}});
        for i in 0..8 {
            instance.pieces.push(Piece {kind: Pawn, color: White, pos: Coord{x:i, y:0}});
            instance.pieces.push(Piece {kind: Pawn, color: Black, pos: Coord{x:i, y:7}});
        }

        instance
    }
}

fn main() {
    let board = Board::new();


}
