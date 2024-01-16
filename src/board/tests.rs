use crate::board::Board;
use crate::board::{Color::White, PieceType::Pawn, Piece};

#[test]
fn basics() {
    let test_board = Board { board: [None; 64] };
    assert!(test_board.get_tile(('A', 1))
           .unwrap()
           .is_none()
    );
}

#[test]
fn test_new() {
    let test_board = Board::new();
    for tile in test_board.board {
        assert!(tile.is_none());
    }
}

#[test]
fn test_add_piece() {
    let mut test_board = Board::new();
    test_board.add_piece(Piece { piece_type: Pawn, color: White }, ('A', 1));
    assert!(test_board.get_tile(('A', 1)).unwrap().unwrap() 
            == Piece { piece_type: Pawn, color: White });
}

#[test]
fn test_move_piece_in_place() {
    let mut test_board = Board::new();
    test_board.add_piece(Piece { piece_type: Pawn, color: White }, ('A', 1));
    test_board.move_piece_in_place(('A', 1), ('B', 2));
    assert!(test_board.get_tile(('A', 1)).unwrap().is_none());
    assert!(test_board.get_tile(('B', 2)).unwrap().unwrap() 
            == Piece { piece_type: Pawn, color: White });
}

#[test]
fn test_load_from_fen() {
    // Test that FEN of starting game generates correct starting position
    let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    assert_eq!
        (Board::load_position_from_fen(start_fen), Board::fresh());
}
