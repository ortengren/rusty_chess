use crate::board;
use crate::board::{Board, Piece, PieceType, Color};
use crate::board::PieceType::{Pawn, Knight, Bishop, Rook, Queen, King};
use crate::board::Color::{White, Black};

struct Game {
    position: Board,
    player_to_move: Color,
    castling_rights: [bool; 4],
    // TODO: add behavior to handle draws by repetition etc.
    // TODO: add behavior to handle en passant
}

impl Game {
    fn new() -> Game {
        // Castling rights in starting position
        let mut white_may_castle_kingside = true;
        let mut white_may_castle_queenside = true;
        let mut black_may_castle_kingside = true;
        let mut black_may_castle_queenside = true;
        let mut castling_rights_arr: [bool; 4] = [
            white_may_castle_kingside, white_may_castle_queenside, 
            black_may_castle_kingside, black_may_castle_queenside ]; 

        Game {
            position: Board::fresh(),
            player_to_move: White,
            castling_rights: castling_rights_arr,
        }
    }
}

mod tests;
