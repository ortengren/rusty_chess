use crate::bitboard::Bitboard;
use crate::position::Position;

#[test]
fn get_king_moves() {
    //TODO: write test
}

#[test]
fn test_from_fen() {
    // test starting position
    let pawns: Bitboard = Bitboard(0xff00000000ff00);
    let knights: Bitboard = Bitboard(0x4200000000000042);
    let bishops: Bitboard = Bitboard(0x2400000000000024);
    let rooks: Bitboard = Bitboard(0x8100000000000081);
    let queens: Bitboard = Bitboard(0x800000000000008);
    let kings: Bitboard = Bitboard(0x1000000000000010);
    let white_pieces: Bitboard = Bitboard(0xffff);
    let black_pieces: Bitboard = Bitboard(0xffff000000000000);
    
    let expected_start_pos: Position = 
        Position { 
            pawns, 
            knights, 
            bishops, 
            rooks, 
            queens, 
            kings, 
            white_pieces, 
            black_pieces
        };

    let fen_pos = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());

    assert_eq!(expected_start_pos, fen_pos);
}
