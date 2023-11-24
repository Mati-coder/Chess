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
    Rook,
    Bishop,
    Knight,
    Pawn,
    Empty, //Used for empty spaces on the board
}

#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
enum Operation {
    Move(PieceType, Coord),
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
    fn move_to(&mut self, target: &Coord, board: &mut Board) {
        board.change(&self.pos, &PieceType::Empty, &self.color);
        board.change(target, &self.kind, &self.color);
        self.pos = *target;
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

    // TODO
    fn process(&mut self, operation: Operation) {
        board.display(
            match operation {
                Operation::Move(piecetype, target) => {
                    let mut valids: Vec<&Piece> = vec![];
                    let mut invalids: Vec<Resut<(), String>> = vec![]; 
                    for piece in &mut self.pieces() {
                        if piecetype == piece.kind {
                            if let Err(msg) = piece.can_move() {invalids.push(Err(msg));}
                            else {valids.push(&mut piece);}
                        }
                    }

                    if valids.len() > 1 {Err("Specify the piece you want to move")}
                    else {if invalids.len() == 1 {invalids[0]}}
                    else {valids[0].move_to(target, &mut self)}
                }
            }
        )
    }

    // TODO Partially Done
    fn display(&self, result: Result<(), String>) {
        let top_left = '\u{250c}';
        let horizontal = '\u{2500}';
        let horizontal_top = '\u{252c}';
        let top_right = '\u{2510}';
        let vertical = '\u{2502}';
        let four_way = '\u{253c}';
        let vertical_left = '\u{251c}';
        let vertical_rigt = '\u{2524}';
        let bottom_left = '\u{2514}';
        let horizontal_bottom = '\u{2534}'; 
        let bottom_right = '\u{2518}';

        print!("{top_left}");
        for i in 0..12 {
            if i % 2 {print!("{horizontal_top}")}
            else {print!("{horizontal}")}
        }
        print!("{top_right}\n");

        
        for i in 0..8 {
            if i != 0 && i != 8 {
                print!("{vertical_left}");
                for i in 0..12 {
                    if i % 2 {print!("{horizontal}")}
                    else {print!("{four_way}")} 
                }
                print!("{vertical_right}\n");
            }

            print!("{vertical}");
            for j in 0..8{
                let mut piece: usize = 2654; // White king
                let (kind, color) = self.board[8-i][j];
                if let Empty = kind {
                    piece = 20; // Space
                } else {
                    // Uses the order of the pieceType enum and the Unicode characters
                    piece += kind as usize;
                    // Uses the order of the Color enum and and the Unicode characters
                    piece += color as usize * 6;

                    print!('\u{piece}');
                    print!("{vertical}");
                }
            }
        }

        print!("{bottom_left}");
        for i in 0..12 {
            if i % 2 {print!("{horizontal_bottom}")}
            else {print!("{horizontal}")}
        }
        print!("{bottom_right}\n");

        if let Err(msg) = result {print!(msg)}
    }

    // Done
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
    let mut board = Board::new();

    board.display(Ok(()));
    board,process(Operation::Move(Pawn, Coor{x: 5, y:1}));
}
