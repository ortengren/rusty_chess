use unicode_segmentation::UnicodeSegmentation;

use crate::bitboard::Bitboard;
use crate::bitboard::{A_FILE, H_FILE};
use crate::board::Color;

// TODO: add fields for castling rights, which player's turn, en passant, 
// half moves, and full moves
#[derive(PartialEq, Debug)]
pub struct Position {
    pawns: Bitboard,
    knights: Bitboard,
    bishops: Bitboard,
    rooks: Bitboard,
    queens: Bitboard,
    kings: Bitboard,
    white_pieces: Bitboard,
    black_pieces: Bitboard,
}

impl Position {
    pub fn from_fen(fen: String) -> Position {
        let mut pawns = Bitboard(0);
        let mut knights = Bitboard(0);
        let mut bishops = Bitboard(0);
        let mut rooks = Bitboard(0);
        let mut queens = Bitboard(0);
        let mut kings = Bitboard(0);
        let mut white_pieces = Bitboard(0);
        let mut black_pieces = Bitboard(0);

        // iterator containing strings representing 1st, 2nd, ..., 8th ranks
        let piece_placement: String = fen.split(" ").take(1).collect();
        let ranks = piece_placement.rsplit("/");
        let mut rank_offset: u64 = 0;
        let mut file: u64;
        let mut character: Option<char>;
        for rank in ranks {
            file = 0;
            for letter in rank.graphemes(false) {
                character = letter.chars().next();
                if character.is_none() {break;}
                if character.unwrap().is_digit(10) {
                    file = file + u64::from(character.unwrap().to_digit(10).unwrap());
                    continue;
                }
                match character.unwrap().to_ascii_lowercase() {
                    'p' => pawns.set_bit_in_place(rank_offset + file),
                    'n' => knights.set_bit_in_place(rank_offset + file),
                    'b' => bishops.set_bit_in_place(rank_offset + file),
                    'r' => rooks.set_bit_in_place(rank_offset + file),
                    'q' => queens.set_bit_in_place(rank_offset + file),
                    'k' => kings.set_bit_in_place(rank_offset + file),
                    _ => {
                        println!("Invalid character\n\
                                  expected: 'p', 'n', 'b', 'r', 'q', or 'k' (ignoring case)\n\
                                  got: {}", character.unwrap());
                        unreachable!();
                    }
                }
                if character.unwrap().is_ascii_uppercase() {
                    white_pieces.set_bit_in_place(rank_offset + file);
                } else {
                    black_pieces.set_bit_in_place(rank_offset + file);
                }
                file += 1;
            }
            rank_offset += 8;
        }
        

        Position {
            pawns, 
            knights, 
            bishops, 
            rooks, 
            queens, 
            kings, 
            white_pieces, 
            black_pieces
        }
    }

    pub fn get_king_moves(&self, king_pos: Bitboard, color: Color) -> Bitboard {
        /*
        match square {
            ('A', 1) => Bitboard(0x0000000000000302),
            ('A', 2) => Bitboard(0x0000000000030203),
            ('A', 3) => Bitboard(0x0000000003020300),
            ('A', 4) => Bitboard(0x0000000302030000),
            ('A', 5) => Bitboard(0x0000030203000000),
            ('A', 6) => Bitboard(0x0003020300000000),
            ('A', 7) => Bitboard(0x0302030000000000),
            ('A', 8) => Bitboard(0x0203000000000000),
            ('B', 1) => Bitboard(0x0000000000000705),
            ('B', 2) => Bitboard(0x0000000000070507),
            ('B', 3) => Bitboard(0x0000000007050700),
            ('B', 4) => Bitboard(0x0000000705070000),
            ('B', 5) => Bitboard(0x0000070507000000),
            ('B', 6) => Bitboard(0x0007050700000000),
            ('B', 7) => Bitboard(0x0705070000000000),
            ('B', 8) => Bitboard(0x0507000000000000),
            ('C', 1) => Bitboard(0x0000000000000e0a),
            ('C', 2) => Bitboard(0x00000000000e0a0e),
            ('C', 3) => Bitboard(0x000000000e0a0e00),
            ('C', 4) => Bitboard(0x0000000e0a0e0000),
            ('C', 5) => Bitboard(0x00000e0a0e000000),
            ('D', 5) => Bitboard()
            _ => unreachable!(),
        }
        */
        let mut neighbor_squares: Bitboard = 
            king_pos.shift_east()
                    .or(king_pos.shift_se())
                    .or(king_pos.shift_south())
                    .or(king_pos.shift_sw())
                    .or(king_pos.shift_west())
                    .or(king_pos.shift_nw())
                    .or(king_pos.shift_north())
                    .or(king_pos.shift_ne());

        if king_pos.and(A_FILE).0 > 0 {
            neighbor_squares = neighbor_squares.and(H_FILE.not());
        } else if king_pos.and(H_FILE).0 > 0 {
            neighbor_squares = neighbor_squares.and(A_FILE.not());
        }

        let open_squares: Bitboard = neighbor_squares.and(
            match color {
                Color::White => self.white_pieces.not(),
                Color::Black => self.black_pieces.not(),
            });
        open_squares
    }
}

mod tests;
