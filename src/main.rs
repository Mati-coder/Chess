use std::cmp::max;
use std::fmt::Display;
use std::io;
use std::ops::Not;

use crate::PieceType::Rook;
use crate::PieceType::Bishop;
use crate::PieceType::Queen;
use crate::PieceType::King;
use crate::PieceType::Knight;
use crate::PieceType::Empty;
use crate::PieceType::Pawn;

use crate::Color::White;
use crate::Color::Black;

use crate::Operation::Move;
use crate::Operation::None;
use crate::Operation::Capture;
use crate::Operation::Castle;

/*
    TODO:
    - Pawn capture
    - Improve the error system
    - Check & Checkmate system 
    - Row/Column specification
    - Special moves for the pawn
    + Castling
    - 50 moves rule
    - Stalemate
    - Better display
*/


#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
// Signed integers for easy distance computation, not meant to ever hold negative values
struct Coord {x: i8, y: i8} 

struct Board {
    board: [[(PieceType, Color); 8]; 8],
    pieces: Vec<Piece>,
    kings: [Coord; 2],
    can_castle: [bool; 2],
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
enum Color {
    Black,
    White,
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self == Black {White}
        else {Black}
    }
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
#[derive(Clone)]
enum Operation {
    Move(PieceType, Coord, Color),
    Capture(PieceType, Coord, Color),
    Castle(Color, bool),
    None(Result<(), String>),
}

#[derive(Debug)]
#[derive(Copy, Clone)]
struct Piece {
    kind: PieceType,
    color: Color, 
    pos: Coord,
}

// This function assumes the string given only contains ascii characters
fn to_coord(pos: &str) -> Result<Coord, String> {
    let mut coord = Coord {x:0, y:0};
 
    //numbers are ASCII characters from a to h
    match pos.as_bytes()[0] {
        97 => {coord.x = 0;},
        98 => {coord.x = 1;},
        99 => {coord.x = 2;},
        100 => {coord.x = 3;},
        101 => {coord.x = 4;},
        102 => {coord.x = 5;},
        103 => {coord.x = 6;},
        104 => {coord.x = 7;},
        _ => {return Err(String::from("Invalid column"))}
    }
    coord.y = pos.as_bytes()[1] as i8 - 49; // 48 is 0 in ASCII

    if coord.y > 7 || coord.y < 0 {return Err(String::from("Invalid row"));}
    Ok(coord)
}

fn get_operation(turn: Color) -> Operation {
    let mut buf = String::new();
    // Len returned is 2 more than the amount we want because it counts the carriage return and the end of line characters
    let len = io::stdin().read_line(&mut buf).unwrap() - 2; 

    if len == 2 { 
        let target: Result<Coord, String> = to_coord(&buf);
        let target = match target {
            Ok(trgt) => trgt,
            Err(err) => {return None(Err(err));},
        };
        return Move(Pawn, target, turn);
    }

    if len == 3 {
        if buf == String::from("O-O\r\n") {
            return Castle(turn, false)
        }

        let kind = match buf.as_bytes()[0] as char {
            'K' => King,
            'Q' => Queen,
            'R' => Rook,
            'B' => Bishop,
            'N' => Knight,
            _ => {return None(Err(String::from("Invalid piece type")))}
        };
        let target: Result<Coord, String> = to_coord(&buf[1..3]);
        let target = match target {
            Ok(trgt) => trgt,
            Err(err) => {return None(Err(err));},
        };
        return Move(kind, target, turn);
    }

    if len == 4 {
        if buf.as_bytes()[1] == 'x' as u8 {
            let kind = match buf.as_bytes()[0] as u8 {
                75 => King,
                81 => Queen,
                82 => Rook,
                66 => Bishop,
                78 => Knight,
                _ => {return None(Err(String::from("Invalid piece type")))}
            };

            let target: Result<Coord, String> = to_coord(&buf[2..4]);
            let target = match target {
                Ok(trgt) => trgt,
                Err(err) => {return None(Err(err));},
            };
            return Capture(kind, target, turn);
        }
    }

    if len == 5 {
        if buf == "O-O-O\r\n" {
            return Castle(turn, true)
        }
    }

    None(Err(String::from("Invalid command")))
}

impl Piece {
    // Works for non-special moves only
    fn valid_move(&self, target: &Coord) -> bool {
        if self.pos == *target {return false}

        let delta_x = target.x - self.pos.x;
        let delta_y = target.y - self.pos.y;
        match self.kind {
            King => delta_x.abs() == 1 || delta_y.abs() == 1,
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

    // Checks the path to the target but not the actual target
    fn path_clear(&self, target: &Coord, board: &Board) -> bool {
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
                for n in 1..=max(delta_x.abs(), delta_y.abs())-1 {
                    if !board.is_empty(&Coord{
                        x: self.pos.x + n * sign_dx, 
                        y: self.pos.y + n * sign_dy}) 
                    {return false}
                }
                true
            }
        }
    }

    fn produces_check(&self, target: &Coord, board: &mut Board) -> bool {
        // Move the piece
        board.change(&self.pos, target, &self.kind, &self.color);
        for piece in &board.pieces {
            if piece.color != self.color {
                // Check wheter other pieces can check the piece's color king
                if piece.can_check(&board.kings[self.color as usize], board) {
                    // Move the piece to its original position
                    board.change(target, &self.pos, &self.kind, &self.color);
                    return true;
                }
            }
        }
        // Move the piece to its original position
        board.change(target, &self.pos, &self.kind, &self.color);
        false
    }

    fn can_check(&self, king_coord: &Coord, board: &Board) -> bool {
        return !self.special_cases(king_coord) && self.valid_move(king_coord) && self.path_clear(king_coord, board);
    }
    
    // Check whether the piece can make a movement
    fn can_move(&self, target: &Coord, board: &mut Board) -> Result<(), String> {
        // We don't need to check if pos is in the board because it is already checked before
        if self.special_cases(&target) {return Ok(())}
        if !board.is_empty(&target) {return Err(String::from("There's a piece there"))}
        if !self.valid_move(&target) {return Err(String::from("Invalid move"))}
        if !self.path_clear(&target, board) {return Err(String::from("There is a piece in the way"))}
        if self.produces_check(&target, board) {return Err(String::from("The king is/would be in check!!!"))}
        Ok(())
    }

    //Check wheter the piece can capture a piece
    fn can_capture_piece(&self, target_piece: &Piece, board: &mut Board) -> Result<(), String> {
        if self.special_cases(&target_piece.pos) {return Ok(())}
        if self.color == target_piece.color {return Err(String::from("Can't capture piece of the same color"))};
        if !self.valid_move(&target_piece.pos) {return Err(String::from("Invalid move"));}
        if !self.path_clear(&target_piece.pos, board) {return Err(String::from("There is a piece in the way"))};
        if self.produces_check(&target_piece.pos, board) {return Err(String::from("The king is/would be in check!!!"))};
        Ok(())
    }

    //TODO
    // Return true if the move is a special case
    fn special_cases(&self, _target: &Coord) -> bool {
        false
    }
}

impl Board {
    fn is_empty(&self, target: &Coord) -> bool {
        self.board[target.y as usize][target.x as usize].0 == Empty
    }

    // Move a piece
    fn change(&mut self, prev:&Coord, new: &Coord, piecetype: &PieceType, color: &Color) {
        self.board[prev.y as usize][prev.x as usize] = (Empty, *color);
        self.board[new.y as usize][new.x as usize] = (*piecetype, *color);
        if *piecetype == King {
            self.kings[*color as usize] = *new;
        }
    }
    
    // TODO
    fn process(&mut self, operation: Operation) -> Result<(), String> {
        match operation {
                Move(piecetype, target, turn) => {
                    let mut valids_index: Vec<usize> = vec![];
                    let mut invalids: Vec<Result<(), String>> = vec![];
                    let mut pieces_index: Vec<usize> = vec![];
                    // Finds index of pieces of the same kind and of the color which has to play
                    for piece_index in 0..self.pieces.len() {
                        if piecetype == self.pieces[piece_index].kind && turn == self.pieces[piece_index].color {
                            pieces_index.push(piece_index);
                        }
                    }

                    // Finds whether those pieces can move
                    for piece_index in pieces_index {
                        let piece = self.pieces[piece_index].clone();
                        let res = piece.can_move(&target, self);
                        if let Err(msg) = res {invalids.push(Err(msg));}
                        else {valids_index.push(piece_index);}
                    }


                    if valids_index.len() > 1 {Err(String::from("Specify the piece you want to move"))}
                    // If no piece could've moved
                    else if valids_index.len() == 0 {
                        let mut ermsg: String = String::from("");
                        for err in invalids {
                            if let Err(msg) = err {if msg != "Invalid move" {ermsg = msg;}}
                        }
                        if ermsg == "" {ermsg = String::from("Invalid move (default message)");}
                        return Err(ermsg)
                    }
                    else {  
                        // Clone for readability
                        let valid_piece = self.pieces[valids_index[0]].clone();
                        // Change of the piece in the board
                        self.change(&valid_piece.pos, 
                            &target, &piecetype, 
                            &valid_piece.color);

                        // Update the variable used to castle
                        if valid_piece.kind == King || valid_piece.kind == Rook {
                            self.can_castle[valid_piece.color as usize] = false;
                        }

                        // Change of the piece's position
                        self.pieces[valids_index[0]].pos = target;
                        return Ok(())
                    }
            },
            Capture(piecetype, target, turn) => {
                let mut valids_index: Vec<usize> = vec![];
                let mut invalids: Vec<Result<(), String>> = vec![];
                let mut pieces_index: Vec<usize> = vec![];
                let mut piece_to_be_captured_index: usize = 255; // Initialized to a value it can never take
                // Finds index of pieces of the same kind and of the color which has to play, and of the piece to be captured
                for piece_index in 0..self.pieces.len() {
                    if piecetype == self.pieces[piece_index].kind && turn == self.pieces[piece_index].color {
                        pieces_index.push(piece_index);
                    }
                    if self.pieces[piece_index].pos == target {
                        piece_to_be_captured_index = piece_index;
                    }
                }
                
                // If the initialized value is not overwritten then throw an error, it means there isnt a piece in the location
                if piece_to_be_captured_index == 255 {
                    return Err(String::from("There isn't a piece there"));
                }
                let piece_to_be_captured = self.pieces[piece_to_be_captured_index].clone();
                if piece_to_be_captured.kind == King {return Err(String::from("Can't capture the king"))}

                // Finds whether those pieces can capture
                for piece_index in pieces_index {
                    let piece = self.pieces[piece_index].clone();
                    let res = piece.can_capture_piece(&piece_to_be_captured,  self);
                    if let Err(msg) = res {invalids.push(Err(msg));}
                    else {valids_index.push(piece_index);}
                }
                

                if valids_index.len() > 1 {Err(String::from("Specify the piece you want to move"))}
                // If no piece could've moved
                else if valids_index.len() == 0 {
                    let mut ermsg: String = String::from("");
                    for err in &invalids {
                        if let Err(msg) = err {if msg != "Invalid move" {ermsg = msg.clone().to_string(); println!("{:?}", invalids);}}
                    }
                    if ermsg == "" {ermsg = String::from("No piece can move there"); println!("{:?}", invalids);}
                    return Err(ermsg)
                }
                else {  
                    let valid_piece = self.pieces[valids_index[0]].clone();
                    // Change of the piece in the board
                    self.change(&valid_piece.pos, 
                        &target, &piecetype, 
                        &valid_piece.color);

                    // Change of the piece's position
                    self.pieces[valids_index[0]].pos = target;

                    // Update the variable used to castle
                    if valid_piece.kind == King || valid_piece.kind == Rook {
                        self.can_castle[valid_piece.color as usize] = false;
                    }

                    // By making the type of the piece Empty, it won't be taken into account for any checks. This is computationally less expensive
                    // than removing the piece from the list
                    self.pieces[piece_to_be_captured_index].kind = Empty;
                    return Ok(())
                }
            },
            Castle(color, is_long) => {
                if !self.can_castle[color as usize] {return Err(String::from("The king or the rook have moved before"))}
                
                let king_index = 8 + color as usize; 

                if is_long {
                    let rook_index = color as usize;

                    if !self.pieces[king_index].path_clear(&self.pieces[rook_index].pos, self) {
                        return Err(String::from("There is a piece in the way"))
                    }

                    for piece in &self.pieces {
                        if piece.can_check(&self.kings[color as usize], self)
                        || piece.can_check(&Coord {x: 3, y: (color as i8) * -7 + 7}, self)
                        || piece.can_check(&Coord {x: 2, y: (color as i8) * -7 + 7}, self)
                        || piece.can_check(&Coord {x: 1, y: (color as i8) * -7 + 7}, self)
                        {
                            println!("{:?}", *piece);
                            return Err(String::from("One of the squares is threatened"))
                        }
                    }

                    let king_prev = self.kings[color as usize].clone();
                    let king_new = Coord{x: 2, y: color as i8 * -7 + 7};
                    let rook_prev = self.pieces[rook_index].pos.clone();
                    let rook_new = Coord{x: 3, y: color as i8 * -7 + 7};

                    // Move the pieces on the board
                    self.change(&king_prev, &king_new, &King, &color);
                    self.change(&rook_prev, &rook_new, &Rook, &color);
                    
                    // Update the pieces' position
                    self.pieces[king_index].pos = king_new;
                    self.pieces[rook_index].pos = rook_new;

                    // Update the king's position
                    self.kings[color as usize] = king_new;

                } else {
                    let rook_index = 14 + color as usize;

                    if !self.pieces[king_index].path_clear(&self.pieces[rook_index].pos, self) {
                        return Err(String::from("There is a piece in the way"))
                    }

                    for piece in &self.pieces {
                        if piece.can_check(&self.kings[color as usize], self)
                        || piece.can_check(&Coord {x: 5, y: (color as i8) * -7 + 7}, self)
                        || piece.can_check(&Coord {x: 6, y: (color as i8) * -7 + 7}, self)
                        {
                            return Err(String::from("One of the squares is threatened"))
                        }
                    }

                    let king_prev = self.kings[color as usize].clone();
                    let king_new = Coord{x: 6, y: color as i8 * -7 + 7};
                    let rook_prev = self.pieces[rook_index].pos.clone();
                    let rook_new = Coord{x: 5, y: color as i8 * -7 + 7};

                    // Move the pieces on the board
                    self.change(&king_prev, &king_new, &King, &color);
                    self.change(&rook_prev, &rook_new, &Rook, &color);
                    
                    // Update the pieces' position
                    self.pieces[king_index].pos = king_new;
                    self.pieces[rook_index].pos = rook_new;
                }
                

                return Ok(());
            }
            None(result) => return result,
        }
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
                    piece = 0x20; // Whitespace
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

    // Constructor
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

        let mut pieces: Vec<Piece> = vec![];
        pieces.reserve(32); // Memory optimization
        let mut instance: Board = Board{board: board, pieces, kings: [Coord{x:4, y:7}, Coord{x:4, y: 0}], can_castle: [true, true]};
        instance.pieces.push(Piece {kind: Rook, color: Black, pos: Coord{x:0, y:7}});
        instance.pieces.push(Piece {kind: Rook, color: White, pos: Coord{x:0, y:0}});
        instance.pieces.push(Piece {kind: Knight, color: White, pos: Coord{x:1, y:0}});
        instance.pieces.push(Piece {kind: Knight, color: Black, pos: Coord{x:1, y:7}});
        instance.pieces.push(Piece {kind: Bishop, color: White, pos: Coord{x:2, y:0}});
        instance.pieces.push(Piece {kind: Bishop, color: Black, pos: Coord{x:2, y:7}});
        instance.pieces.push(Piece {kind: Queen, color: White, pos: Coord{x:3, y:0}});
        instance.pieces.push(Piece {kind: Queen, color: Black, pos: Coord{x:3, y:7}});
        instance.pieces.push(Piece {kind: King, color: Black, pos: Coord{x:4, y:7}});
        instance.pieces.push(Piece {kind: King, color: White, pos: Coord{x:4, y:0}});
        instance.pieces.push(Piece {kind: Bishop, color: White, pos: Coord{x:5, y:0}});
        instance.pieces.push(Piece {kind: Bishop, color: Black, pos: Coord{x:5, y:7}});
        instance.pieces.push(Piece {kind: Knight, color: White, pos: Coord{x:6, y:0}});
        instance.pieces.push(Piece {kind: Knight, color: Black, pos: Coord{x:6, y:7}});
        instance.pieces.push(Piece {kind: Rook, color: Black, pos: Coord{x:7, y:7}});
        instance.pieces.push(Piece {kind: Rook, color: White, pos: Coord{x:7, y:0}});
        for i in 0..8 {
            instance.pieces.push(Piece {kind: Pawn, color: White, pos: Coord{x:i, y:1}});
            instance.pieces.push(Piece {kind: Pawn, color: Black, pos: Coord{x:i, y:6}});
        }

        instance
    }
}

impl Display for Board {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(Ok(()));
        Ok(())
    }
}

fn main() {
    let mut board = Board::new();

    board.display(Ok(()));
    let mut turn = White;
    loop {
        let res = board.process(get_operation(turn));
        if let Ok(()) = res {turn = !turn}
        board.display(res);
    }
}
