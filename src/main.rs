use std::convert::TryFrom;
use std::convert::TryInto;

struct ChessGame {
    board : Board,
}

impl ChessGame {
    fn new_game() -> ChessGame {
        let mut b = Board::empty_board();

        let white_pawn_row = [PieceType::White(Piece::Pawn); 8];
        let black_pawn_row = [PieceType::Black(Piece::Pawn); 8];

        let white_row = [
            PieceType::White(Piece::Rook),
            PieceType::White(Piece::Knight),
            PieceType::White(Piece::Bishop),
            PieceType::White(Piece::Queen),
            PieceType::White(Piece::King),
            PieceType::White(Piece::Bishop),
            PieceType::White(Piece::Knight),
            PieceType::White(Piece::Rook),
        ];

        let black_row = [
            PieceType::Black(Piece::Rook),
            PieceType::Black(Piece::Knight),
            PieceType::Black(Piece::Bishop),
            PieceType::Black(Piece::Queen),
            PieceType::Black(Piece::King),
            PieceType::Black(Piece::Bishop),
            PieceType::Black(Piece::Knight),
            PieceType::Black(Piece::Rook),
        ];

        b.b[0] = white_row;
        b.b[1] = white_pawn_row;
        b.b[6] = black_pawn_row;
        b.b[7] = black_row;

        ChessGame { board: b }
    }
}

#[derive(Debug, Clone, Copy)]
struct Board {
    b : [[PieceType; 8]; 8],
}

impl Board {
    fn empty_board() -> Board {
        Board {
            b : [[PieceType::Empty; 8]; 8]
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    row : u8,
    col : u8,
}

impl TryFrom<&str> for Position {
    type Error = Error;
    fn try_from(value: &str) -> Result<Position, Error> {
        if 1 == 1 {
            Ok(Position { row : 2, col : 2 } )
        }
        else {
            Err(Error { no : 121 })
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Piece {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

#[derive(Debug, Clone, Copy)]
enum PieceType {
    White(Piece),
    Black(Piece),
    Empty
}

struct Error {
    no : u16,
}


fn main() {
    let game = ChessGame::new_game();
    let position = "e4".try_into()?;
    println!("Hello, world!");
}
