pub use pleco::Board;
use pleco::Player;
use std::time::{Duration, Instant};
#[macro_use]
extern crate text_io;
//project files
mod enginemove;
mod usermove;
mod clock;




fn game_over(board: &Board, history: &mut Vec<u64>, clock: &clock::Clock) -> bool {
    let mut threefold = false;
    let mut insufficient = false;

    let insufficient_positions: [u64; 5] = [
        14531074927864995568, //k n vs K
        12777358361509687304, //k vs K N
        15001244464087343965, //k vs K B
        2182324930161781490,  //k b vs K
        1440794681747902690,  //k vs K
                              //needs same colored bishops
    ];

    if board.moves_played() > 40 { // should check this 40 move thing first and only then should it check contains
        if insufficient_positions.contains(&board.material_key()) {
            insufficient = true;
        }
    }

    if board.rule_50() <= 1 {
        history.push(board.zobrist());
        if history.iter().filter(|&n| *n == board.zobrist()).count() == 3 { //idk how the fuck this works. counts to see if there are three of the same positions in history.
            threefold = true;
        }
    } else {
        history.clear();
    }

    let flag = clock.white_flag() || clock.black_flag();

    !(!board.checkmate() && !board.stalemate() && !threefold && !insufficient && !flag)
}

fn play(mut board: Board) {
    let mut history = vec![];

    let mut clock = clock::Clock{
        increment: Duration::from_secs(5),
        white_time_left: Duration::from_secs(300),
        black_time_left: Duration::from_secs(300),
    };

    while !game_over(&board, &mut history, &clock) {
        if board.turn() == Player::White {
            let white_begin = Instant::now();
            board.apply_move(enginemove::choice(&board));
            let white_end = Instant::now();
            let white_movetime = white_end.duration_since(white_begin);
            clock.white_update(white_movetime);

        } else {
            let black_begin = Instant::now();
            board.apply_move(usermove::choice(&board));
            let black_end = Instant::now();
            let black_movetime = black_end.duration_since(black_begin);
            clock.black_update(black_movetime);
        }
        board.pretty_print();
        clock.print_time();
    }
}

fn main() {
    let board = Board::start_pos();
    enginemove::init_tree(board);
    // play(board);
}


