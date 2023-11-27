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
#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
struct Coord {x: i8, y: i8} 

struct Board {
    board: [[(PieceType, Color); 8]; 8],
    pieces: Vec<Piece>,
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
enum Color {
    Black,
    White,
}

#[derive(Debug)]
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

#[derive(Copy, Clone)]
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
                    Color::Black => delta_y.signum() == -1,
                    Color::White => delta_y.signum() == 1,
                }
            },
            Queen => delta_x.abs() == delta_y.abs() || delta_x*delta_y == 0,
            Rook => delta_x * delta_y == 0,
            Bishop => delta_x.abs() == delta_y.abs(),
            Knight => (delta_x * delta_y).abs() == 2,
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
}

impl Board {
    //Done
    fn is_empty(&self, target: &Coord) -> bool {
        self.board[target.y as usize][target.x as usize].0 == PieceType::Empty
    }

    // Done
    fn change(&mut self, prev:&Coord, new: &Coord, piecetype: &PieceType, color: &Color) {
        self.board[prev.y as usize][prev.x as usize] = (Empty, *color);
        self.board[new.y as usize][new.x as usize] = (*piecetype, *color);
    }

    // TODO
    fn process(&mut self, operation: Operation) {
        let result: Result<(), String> = match operation {
                Operation::Move(piecetype, target) => {
                    let mut valids_index: Vec<usize> = vec![];
                    let mut invalids: Vec<Result<(), String>> = vec![];
                    let mut pieces_index: Vec<usize> = vec![];
                    // Finds index of pieces of the same kind
                    for piece_index in 0..self.pieces.len() {
                        if piecetype == self.pieces[piece_index].kind {
                            pieces_index.push(piece_index);
                        }
                    }

                    // Finds whether those pieces can move
                    for piece_index in pieces_index {
                        if let Err(msg) = self.pieces[piece_index].clone().can_move(&target, self) {invalids.push(Err(msg));}
                        else {valids_index.push(piece_index);}
                    }

                    if valids_index.len() > 1 {Err(String::from("Specify the piece you want to move"))}
                    // Return an error if there is only one piece that could've moved
                    else if invalids.len() == 1 {invalids[0].clone()}
                    // Else return the default message
                    else if valids_index.len() == 0 {Err(String::from("Invalid move"))}
                    else {  
                        let valid_piece = self.pieces[valids_index[0]].clone();
                        self.change(&valid_piece.pos, 
                            &target, &piecetype, 
                            &valid_piece.color);

                        let new = Piece {pos: target, ..valid_piece};
                        self.pieces.push(new);
                        self.pieces.swap_remove(valids_index[0]);
                        Ok(())
                    }
            }
        };
        self.display(result)
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
        let vertical_right = '\u{2524}';
        let bottom_left = '\u{2514}';
        let horizontal_bottom = '\u{2534}'; 
        let bottom_right = '\u{2518}';

        print!("{top_left}");
        for i in 0..15 {
            if i % 2 == 1 {print!("{horizontal_top}")}
            else {print!("{horizontal}")}
        }
        print!("{top_right}\n");

        
        for i in 0..8 {
            if i != 0 && i != 8 {
                print!("{vertical_left}");
                for i in 0..15 {
                    if i % 2 == 0 {print!("{horizontal}")}
                    else {print!("{four_way}")} 
                }
                print!("{vertical_right}\n");
            }

            print!("{vertical}");
            for j in 0..8{
                let mut piece: u32 = 0x2654; // White king
                let (kind, color) = self.board[7-i][j];
                if let Empty = kind {
                    piece = 0x20; // Space
                } else {
                    // Uses the order of the pieceType enum and the Unicode characters
                    piece += kind as u32;
                    // Uses the order of the Color enum and and the Unicode characters
                    piece += color as u32 * 6;
                }

                if let Some(c) = char::from_u32(piece) {print!("{c}");}

                print!("{vertical}");
            }
            print!("\n");
        }

        print!("{bottom_left}");
        for i in 0..15 {
            if i % 2 == 1 {print!("{horizontal_bottom}")}
            else {print!("{horizontal}")}
        }
        print!("{bottom_right}\n");

        if let Err(msg) = result {print!("{msg}\n")}
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
            instance.pieces.push(Piece {kind: Pawn, color: White, pos: Coord{x:i, y:1}});
            instance.pieces.push(Piece {kind: Pawn, color: Black, pos: Coord{x:i, y:6}});
        }

        instance
    }
}

fn main() {
    let mut board = Board::new();

    board.display(Ok(()));
    // Juga un rato con esto si queres
    board.process(Operation::Move(Pawn, Coord{x: 0, y:2}));
    board.process(Operation::Move(Pawn, Coord{x: 0, y:3}));
    board.process(Operation::Move(Pawn, Coord{x: 0, y:4}));
    board.process(Operation::Move(Pawn, Coord{x: 0, y:5}));
    board.process(Operation::Move(Pawn, Coord{x: 1, y:5}));
    board.process(Operation::Move(Rook, Coord{x: 0, y:3}));
    board.process(Operation::Move(Rook, Coord{x: 6, y:3}));
    board.process(Operation::Move(Knight, Coord{x: 0, y:2}));
}
