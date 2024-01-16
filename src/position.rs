use crate::bitboard::Bitboard;
use crate::bitboard::{A_FILE, B_FILE, C_FILE, D_FILE, 
                      E_FILE, F_FILE, G_FILE, H_FILE};
use crate::bitboard::{FIRST_RANK, SECOND_RANK, THIRD_RANK, FOURTH_RANK, 
                      FIFTH_RANK, SIXTH_RANK, SEVENTH_RANK, EIGHTH_RANK};
use crate::board::FILES;
use crate::board::Color;

// TODO: add fields for castling rights, which player's turn, en passant, 
// half moves, and full moves
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
        let position_str = fen.split_ascii_whitespace().next().take().unwrap();

        // FEN starts at A8 (i.e. index 7) and moves right and down
        let mut curr_file = 0;
        let mut curr_rank = 8;
        let mut curr_sq = (FILES[curr_file], curr_rank);

        let mut pawns = Bitboard(0);
        let mut knights = Bitboard(0);
        let mut bishops = Bitboard(0);
        let mut rooks = Bitboard(0);
        let mut queens = Bitboard(0);
        let mut kings = Bitboard(0);
        let mut white_pieces = Bitboard(0);
        let mut black_pieces = Bitboard(0);

        for character in position_str.chars() {

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

            // character represents a square occupied by a piece
            curr_sq = (FILES[curr_file], curr_rank);

            if character.is_ascii_uppercase() {
                white_pieces = white_pieces.add(Bitboard::from_square(curr_sq));
            } else if character.is_ascii_lowercase() {
                black_pieces = black_pieces.add(Bitboard::from_square(curr_sq));
            }

            match character.to_ascii_lowercase() {
                'p' => pawns = pawns.add(Bitboard::from_square(curr_sq)),
                'n' => knights = knights.add(Bitboard::from_square(curr_sq)),
                'b' => bishops = bishops.add(Bitboard::from_square(curr_sq)),
                'r' => rooks = rooks.add(Bitboard::from_square(curr_sq)),
                'q' => queens = queens.add(Bitboard::from_square(curr_sq)),
                'k' => kings = kings.add(Bitboard::from_square(curr_sq)),
                _ => unreachable!(),
            }
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
