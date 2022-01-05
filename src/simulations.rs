pub use pleco::BitMove;
pub use pleco::Board;
pub use pleco::Player;
pub use pleco::core::GenTypes;

pub fn simulation(board: Board) -> i32 {

    // generates moves
    // if the see is greater than 0 then the move is played,
    // else a move random quiet move is moves
    // this is repeated until either
        // it is 10 moves out & the last four ply have been quiet
        // there is a checkmate or draw
        // if by the 10th move there have not been four quiet prior moves
        // keep playing until there are 4 quiets in a row (or mate or draw)
        

    // there player who captured more material in the 10 move loop will be the one who won

    // if there material is the same as it started with then its a draw

    // win for maximizing player is 1 draw is 0 and -1 is win for opp

    let captures = board.generate_moves_of_type(GenTypes::Captures);
    board.generate_moves_of_type(GenTypes::Quiets);
    board.generate_moves_of_type(GenTypes::Evasions);


}