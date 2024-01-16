use rusty_chess::board::Board;

fn main() {
    let board1: Board = Board::fresh();
    let board_str1: String = board1.get_board_string();
    print!("{}", board_str1);

    let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    let board2: Board = Board::load_position_from_fen(start_fen);
    let board_str2: String = board2.get_board_string();
    print!("{}", board_str2);
}
