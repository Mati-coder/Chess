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

macro_rules! empty {
    ($x: expr) =>{
        Piece{kind:Empty, color: White, pos: $x}
    }
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
// Signed integers for easy distance computation, not meant to ever hold negative values
struct Coord {x: i8, y: i8} 

impl Coord {
    fn minus_x(&self, n: i8) -> Coord {
        Coord {x: self.x - n , y: self.y}
    }

    fn minus_y(&self, n: i8) -> Coord {
        Coord {x: self.x , y: self.y - n}
    }
}

struct Board {
    board: [[Piece; 8]; 8],
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

impl Display for Piece {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut piece: u32 = 0x2654; // White king
        if self.kind == Empty {
            piece = 0x20; // Whitespace
        } else {
            // Uses the order of the pieceType enum and the Unicode characters
            piece += self.kind as u32;
            // Uses the order of the Color enum and and the Unicode characters
            piece += self.color as u32 * 6;
        }
        if let Some(c) = char::from_u32(piece) {print!("{c}");}
        Ok(())
    }
}

// This function assumes the string given only contains ascii characters
fn to_coord(pos: &str) -> Result<Coord, String> {
    let mut coord = Coord {x:0, y:0};   
    let mut pos_iterator = pos.chars();
 
    match pos_iterator.next().unwrap(){
        'a' => {coord.x = 0;},
        'b' => {coord.x = 1;},
        'c' => {coord.x = 2;},
        'd' => {coord.x = 3;},
        'e' => {coord.x = 4;},
        'f' => {coord.x = 5;},
        'g' => {coord.x = 6;},
        'h' => {coord.x = 7;},
        _ => {return Err(String::from("Invalid column"))}
    }
    coord.y = pos_iterator.next().unwrap() as i8 - 49; // 49 is 1 in ASCII

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

fn is_empty(coord: &Coord, board: &Board) -> bool {
    return board.board[coord.x as usize][coord.y as usize].kind == Empty
}

// Works for non-special moves only
fn valid_move(piece: &Piece, target: &Coord) -> bool {
    if piece.pos == *target {return false}

    let delta_x = target.x - piece.pos.x;
    let delta_y = target.y - piece.pos.y;
    match piece.kind {
        King => delta_x.abs() == 1 || delta_y.abs() == 1,
        Pawn => {
            if delta_x != 0 || delta_y.abs() > 1 {return false}
            match piece.color {
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
fn path_clear(piece: &Piece, target: &Coord, board: &Board) -> bool {
    // It's important to note that we assume the move is valid
    match piece.kind {
        Knight => true,
        Empty => false,
        // Under the previous assumption this method works for every piece
        _ => {
            let delta_x = target.x - piece.pos.x;
            let delta_y = target.y - piece.pos.y;
            let sign_dx = delta_x.signum();
            let sign_dy = delta_y.signum();
            for n in 1..=max(delta_x.abs(), delta_y.abs())-1 {
                if !is_empty(
                    &Coord{
                    x: piece.pos.x + n * sign_dx, 
                    y: piece.pos.y + n * sign_dy}
                    , board) 
                {return false}
            }
            true
        }
    }
}

// Check wheter moving a piece causes a check on its own king
fn produces_check(piece: &Piece, target: &Coord, board: &mut Board) -> bool {
    // Move the piece
    let prev = move_recover(*piece, target, board, empty!(piece.pos));
    for column in &board.board {
        for some_piece in column {
            if some_piece.kind != Empty && some_piece.color == !piece.color {
                if can_check(some_piece, &board.kings[piece.color as usize], board) {
                    // Recover board to the previous state
                    move_recover(*piece, &prev.pos, board, prev);
                    return true;
                }
            }
        }
    }
    // Recover board to the previous state
    move_recover(*piece, &prev.pos, board, prev);
    false
}

// Return wheter the king would be on check by a certain piece
fn can_check(piece: &Piece, king_coord: &Coord, board: &Board) -> bool {
    if valid_move(piece, king_coord) && path_clear(piece, king_coord, board) {
        println!("{}", *piece);
        return true;
    }

    return false;
}

// Check whether the piece can make a movement
fn can_move(piece: &Piece, target: &Coord, board: &mut Board) -> Result<(), String> {
    // We don't need to check if pos is in the board because it is already checked before
    if !valid_move(piece, &target) {return Err(String::from("Invalid move"))}
    if !path_clear(piece, &target, board) {return Err(String::from("There is a piece in the way"))}
    if !is_empty(target, board) {return Err(String::from("There's a piece there"))}
    if produces_check(piece, &target, board) {return Err(String::from("The king is/would be in check!!!"))}
    Ok(())
}

//Check wheter the piece can capture another piece
fn can_capture(piece: &Piece, target: &Coord, board: &mut Board) -> Result<(), String> {
    if !valid_move(piece, &target) {return Err(String::from("Invalid move"));}
    if !path_clear(piece, &target, board) {return Err(String::from("There is a piece in the way"))};
    if piece.color == board.board[target.x as usize][target.y as usize].color 
    {return Err(String::from("Can't capture piece of the same color"))};
    if produces_check(piece,&target, board) {return Err(String::from("The king is/would be in check!!!"))};
    Ok(())
}

// Deletes the piece in the target position and puts an empty on the current postion
fn move_lose(mut piece: Piece, target: &Coord, board: &mut Board) {
    if piece.kind == King {
        board.kings[piece.color as usize] = *target;
    }
    // Replacement piece
    board.board[piece.pos.x as usize][piece.pos.y as usize] = empty!(piece.pos);
    piece.pos = *target;
    board.board[target.x as usize][target.y as usize] = piece;
}

// Places a specified piece in the current position instead of an Empty, returns piece in the target position
fn move_recover(mut piece: Piece, target: &Coord, board: &mut Board, new_piece: Piece) -> Piece {
    if piece.kind == King {
        board.kings[piece.color as usize] = *target;
    }
    // Replacement piece
    board.board[piece.pos.x as usize][piece.pos.y as usize] = new_piece;
    piece.pos = *target;
    let previous = board.board[target.x as usize][target.y as usize];
    board.board[target.x as usize][target.y as usize] = piece;

    previous
}

fn new_board() -> Board {
    // The piece is used to initialice, all copies are overwritten
    let mut board: [[Piece; 8]; 8] = [[Piece{kind:Empty, color: Black, pos: Coord{x:0, y:0}}; 8]; 8];

    board[0][0] = Piece{kind: Rook, color: White, pos: Coord{x: 0, y: 0}};
    //board[1][0] = Piece{kind: Knight, color: White, pos: Coord{x: 1, y: 0}};
    //board[2][0] = Piece{kind: Bishop, color: White, pos: Coord{x: 2, y: 0}};
    //board[3][0] = Piece{kind: Queen, color: White, pos: Coord{x: 3, y: 0}};
    board[4][0] = Piece{kind: King, color: White, pos: Coord{x: 4, y: 0}};
    board[5][0] = Piece{kind: Bishop, color: White, pos: Coord{x: 5, y: 0}};
    board[6][0] = Piece{kind: Knight, color: White, pos: Coord{x: 6, y: 0}};
    board[7][0] = Piece{kind: Rook, color: White, pos: Coord{x: 7, y: 0}};
    for i in 0..8 {
        board[i][1] = Piece{kind: Pawn, color: White, pos: Coord{x: i as i8, y: 1}};
    }
    for i in 2..6 {
        for j in 0..8 {
            board[j][i] = Piece{kind: Empty, color: White, pos: Coord{x: j as i8, y: i as i8}};
        }
    }
    for i in 0..8 {
        board[i][6] = Piece{kind: Pawn, color: Black, pos: Coord{x: i as i8, y: 6}};
    }
    board[0][7] = Piece{kind: Rook, color: Black, pos: Coord{x: 0, y: 7}};
    board[1][7] = Piece{kind: Knight, color: Black, pos: Coord{x: 1, y: 7}};
    board[2][7] = Piece{kind: Bishop, color: Black, pos: Coord{x: 2, y: 7}};
    board[3][7] = Piece{kind: Queen, color: Black, pos: Coord{x: 3, y: 7}};
    board[4][7] = Piece{kind: King, color: Black, pos: Coord{x: 4, y: 7}};
    board[5][7] = Piece{kind: Bishop, color: Black, pos: Coord{x: 5, y: 7}};
    board[6][7] = Piece{kind: Knight, color: Black, pos: Coord{x: 6, y: 7}};
    board[7][7] = Piece{kind: Rook, color: Black, pos: Coord{x: 7, y: 7}};
    
    Board {
        board,
        kings: [Coord{x: 4, y: 7}, Coord{x:4, y: 0}],
        can_castle: [true, true],
    }
}

fn display(board: &Board, result: Result<(), String>) {
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
            print!("{}", board.board[j][7-i]);
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

    if let Err(msg) = result {print!("{msg}\n");}
}

fn process(board: &mut Board, operation: Operation) -> Result<(), String> {
    match operation {
        Move(kind, target, color) => {
            let mut valids: Vec<Piece> = vec![];
            let mut invalids: Vec<String> = vec![];
            for col in board.board {
                for piece in col {
                    if piece.kind == kind && piece.color == color {
                        match can_move(&piece, &target, board) {
                            Ok(()) => valids.push(piece),
                            Err(msg) => invalids.push(msg),
                        }
                    }
                }
            }

            if valids.len() > 1 { return Err(String::from("Specify the piece you want to move"))}
            if valids.len() == 0  {
                if invalids.len() == 0 { return Err(String::from("There are no more such pieces"));}
                let first = invalids[0].clone();
                if invalids.len() == 1 {return Err(first);}
                for str in invalids{
                    if str != "Invalid move" {return Err(str);}
                }
                return Err(first);
            }

            move_lose(valids[0], &target, board);

            Ok(())
        },
        Capture(kind, target, color) => {
            let mut valids: Vec<Piece> = vec![];
            let mut invalids: Vec<String> = vec![];
            for col in board.board {
                for piece in col {
                    if piece.kind == kind && piece.color == color {
                        match can_capture(&piece, &target, board) {
                            Ok(()) => valids.push(piece),
                            Err(msg) => invalids.push(msg),
                        }
                    }
                }
            }

            if valids.len() > 1 { return Err(String::from("Specify the piece you want to move"))}
            if valids.len() == 0  {
                if invalids.len() == 0 { return Err(String::from("There are no more such pieces"));}
                let first = invalids[0].clone();
                if invalids.len() == 1 {return Err(first);}
                for str in invalids{
                    if str != "Invalid move" {return Err(str);}
                }
                return Err(first);
            }

            move_lose(valids[0], &target, board);

            Ok(())},
        Castle(color, is_long) => {
            if !board.can_castle[color as usize] {return Err(String::from("Can't castle 1"))}
            let king = &board.board[4][7 * color as usize];
            let king_pos = &king.pos.clone();
            if is_long {
                if path_clear(
                    king,
                     &king_pos.minus_x(3), board) {
                    for i in 0..3 {
                        for col in &board.board {
                            for piece in col {
                                if piece.color == !color {
                                    if can_check(piece, &king_pos.minus_x(i), board) {
                                        return Err(String::from("Can't castle 2"))
                                    }
                                }
                            }
                        }
                    }
                    
                    move_lose(*king, &king_pos.minus_x(2), board);
                    move_lose(board.board[0][7*color as usize].clone(), &king_pos.minus_x(3), board);
                }
            }

            Ok(())            
        }
        None(result) => {return result;}
    }
}

fn main() {
    let mut board = new_board();

    display(&board, Ok(()));
    let mut turn = White;
    loop {
        let res = process(&mut board, get_operation(turn));
        if let Ok(()) = res {turn = !turn}
        display(&board, res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn board() {
        let board = new_board();

        for i in 0..8 {
            for j in 0..8 {
                let piece = board.board[i][j];
                assert_eq!(piece.pos.x, i as i8);
                assert_eq!(piece.pos.y, j as i8);
            }
        }
    }
}
