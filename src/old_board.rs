use core::fmt;
use std::fmt::Debug;

pub const FILES: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
pub const RANKS: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

pub fn name_to_idx(square: (char, u8)) -> u8 {
    let file: u8;
    assert!(square.1 >= 1 && square.1 <= 8);
    let rank: u8 = square.1 - 1;
    match square.0 {
        'A' => file = 0,
        'B' => file = 8,
        'C' => file = 16,
        'D' => file = 24,
        'E' => file = 32,
        'F' => file = 40,
        'G' => file = 48,
        'H' => file = 56,
        _ => panic!("Invalid input"),
    }
    file + rank
}

#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    fn new(color: Color, piece_type: PieceType) -> Piece {
        Piece {
            piece_type,
            color,
        }
    }
}

impl Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_desc: (Color, PieceType) = (self.color, self.piece_type);
        let letter_representation = 
            match piece_desc {
                (Color::White, PieceType::Pawn) => "P",
                (Color::White, PieceType::Knight) => "N",
                (Color::White, PieceType::Bishop) => "B",
                (Color::White, PieceType::Rook) => "R",
                (Color::White, PieceType::Queen) => "Q",
                (Color::White, PieceType::King) => "K",
                (Color::Black, PieceType::Pawn) => "p",
                (Color::Black, PieceType::Knight) => "n",
                (Color::Black, PieceType::Bishop) => "b",
                (Color::Black, PieceType::Rook) => "r",
                (Color::Black, PieceType::Queen) => "q",
                (Color::Black, PieceType::King) => "k",
            };
        write!(f, "{}", letter_representation)
    }
}

type Tile = Option<Piece>;

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    board: [Tile; 64],
}

impl Board {
    fn get_mut_tile(&mut self, square: (char, u8)) -> Result<&mut Tile, &'static str> {
        let sq_idx: usize = name_to_idx(square).into();
        Ok(&mut self.board[sq_idx])
    }

    fn get_tile(&self, square: (char, u8)) -> Result<&Tile, &'static str> {
        if square.1 > 8 {
            return Err("Invalid input")
        }
        let sq_idx: usize = name_to_idx(square).into();
        Ok(&self.board[sq_idx])
    }

    fn move_piece_in_place(&mut self, start_sq: (char, u8), end_sq: (char, u8)) {
        let mut piece: Piece = self.get_mut_tile(start_sq).unwrap().take().unwrap();
        self.add_piece(piece, end_sq);
    }

    fn move_piece(&self, start_sq: (char, u8), end_sq: (char, u8)) -> Board {
        let mut new_board: Board = self.clone();
        new_board.move_piece_in_place(start_sq, end_sq);
        new_board
    }

    fn new() -> Board {
        Board {
            board: [None; 64],
        }
    }

    pub fn fresh() -> Board {
        let mut new_board = Board::new();
        let files: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
        // add pawns
        for file in files {
            new_board.add_piece(
                Piece { piece_type: PieceType::Pawn, color: Color::White }, (file, 2));
            new_board.add_piece(
                Piece { piece_type: PieceType::Pawn, color: Color::Black }, (file, 7));
        }

        // add rooks
        new_board.add_piece(
            Piece { piece_type: PieceType::Rook, color: Color::White }, ('A', 1));
        new_board.add_piece(
            Piece { piece_type: PieceType::Rook, color: Color::White }, ('H', 1));
        new_board.add_piece(
            Piece { piece_type: PieceType::Rook, color: Color::Black }, ('A', 8));
        new_board.add_piece(
            Piece { piece_type: PieceType::Rook, color: Color::Black }, ('H', 8));

        // add knights
        new_board.add_piece(
            Piece { piece_type: PieceType::Knight, color: Color::White }, ('B', 1));
        new_board.add_piece(
            Piece { piece_type: PieceType::Knight, color: Color::White }, ('G', 1));
        new_board.add_piece(
            Piece { piece_type: PieceType::Knight, color: Color::Black }, ('B', 8));
        new_board.add_piece(
            Piece { piece_type: PieceType::Knight, color: Color::Black }, ('G', 8));

        // add bishops
        new_board.add_piece(
            Piece { piece_type: PieceType::Bishop, color: Color::White }, ('C', 1));
        new_board.add_piece(
            Piece { piece_type: PieceType::Bishop, color: Color::White }, ('F', 1));
        new_board.add_piece(
            Piece { piece_type: PieceType::Bishop, color: Color::Black }, ('C', 8));
        new_board.add_piece(
            Piece { piece_type: PieceType::Bishop, color: Color::Black }, ('F', 8));

        // add queens
        new_board.add_piece(
            Piece { piece_type: PieceType::Queen, color: Color::White }, ('D', 1));
        new_board.add_piece(
            Piece { piece_type: PieceType::Queen, color: Color::Black }, ('D', 8));

        // add kings
        new_board.add_piece(
            Piece { piece_type: PieceType::King, color: Color::White }, ('E', 1));
       new_board.add_piece(
            Piece { piece_type: PieceType::King, color: Color::Black }, ('E', 8));

       new_board
    }

    fn add_piece(&mut self, piece: Piece, square: (char, u8)) {
        let idx: usize = name_to_idx(square).into();
        self.board[idx] = Some(piece);
    }

    pub fn load_position_from_fen(fen_str: &str) -> Board {
        use PieceType::{Pawn, Knight, Bishop, Rook, Queen, King};
        use Color::{Black, White};
        let mut new_board = Board::new();
        let position_str = 
            fen_str.split_ascii_whitespace().next().take().unwrap();

        // FEN starts at A8 (i.e. index 7) and moves right and down
        let mut curr_file = 0;
        let mut curr_rank = 8;
        let mut curr_sq = (FILES[curr_file], curr_rank);

        for character in position_str.chars() {
            print!("{}", character);
            if character.is_digit(10) {
                // move forward `character` files, leaving blank tiles
                curr_file = curr_file + character.to_digit(10).unwrap() as usize % 8;
                curr_sq.0 = FILES[curr_file];
                continue;
            }

            if character == '/' {
                curr_rank = curr_rank - 1;
                curr_file = 0;
                curr_sq = (FILES[curr_file], curr_rank);
                continue;
            }

            curr_sq = (FILES[curr_file], curr_rank);
            new_board.add_piece(
                match character {
                    'p' => Piece::new(Color::Black, PieceType::Pawn),
                    'n' => Piece::new(Color::Black, PieceType::Knight),
                    'b' => Piece::new(Color::Black, PieceType::Bishop),
                    'r' => Piece::new(Color::Black, PieceType::Rook),
                    'q' => Piece::new(Color::Black, PieceType::Queen),
                    'k' => Piece::new(Color::Black, PieceType::King),
                    'P' => Piece::new(Color::White, PieceType::Pawn),
                    'N' => Piece::new(Color::White, PieceType::Knight),
                    'B' => Piece::new(Color::White, PieceType::Bishop),
                    'R' => Piece::new(Color::White, PieceType::Rook),
                    'Q' => Piece::new(Color::White, PieceType::Queen),
                    'K' => Piece::new(Color::White, PieceType::King),
                    _ => panic!("Invalid FEN character")}, curr_sq);
            curr_file = curr_file + 1;
        }
        print!("\n");
        new_board
    }

    fn get_tile_string(&self, square: (char, u8)) -> String {
        let ok_tile: &Tile = self.get_tile(square).unwrap();
        if ok_tile.is_none() {
            ".".to_string()
        } else {
            let piece = ok_tile.unwrap();
            format!("{piece:?}")
        }
    }

    fn get_rank_string(&self, rank: u8) -> String {
        let mut rank_string: String = "".to_string();
        for file in FILES {
            rank_string = format!(
                "{} {}", rank_string, self.get_tile_string((file, rank)));
        }
        format!("{}\n", rank_string)
    }

    pub fn get_board_string(&self) -> String {
        let mut board_string: String = "\n".to_string();
        for rank in RANKS {
            board_string = format!("{}{}", self.get_rank_string(rank), board_string);
        }
        board_string
    }
}

#[cfg(test)]
mod tests;
