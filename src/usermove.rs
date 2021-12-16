pub use pleco::Board;
pub use pleco::BitMove;



pub fn choice(board: &Board) -> BitMove {
    
    let moves = board.generate_moves();
    
    let mut number = 0;
    for x in moves.iter() {
        println!("{}: {}",number, x);
        number += 1;
    }

    let some_move: usize = read!();

    moves[some_move]
    
}