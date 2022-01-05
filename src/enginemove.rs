pub use pleco::BitMove;
pub use pleco::Board;
pub use pleco::Player;
pub use pleco::core::GenTypes;
pub use rand::seq::SliceRandom;
pub use rand::thread_rng;
use std::time::{Duration, Instant};

//use pleco::Player;
use rand::Rng;


#[derive(Copy, Clone)]
pub struct PositionId{
    branch: usize,
    index: usize,
}

#[derive(std::cmp::PartialEq)]
enum NodeType {
    root,
    twig,
    leaf,
}

pub struct Position {
    id: PositionId,
    node_type: NodeType,
    parent: Option<PositionId>,
    
    child_first: Option<PositionId>,
    child_last: Option<PositionId>,
    // The actual data which will be stored within the tree
    
    wins: f32,
    simulations: i32,
    pub visits: i32,
    pub route: Option<BitMove>,
    pub position: Board,
}

impl Position {
    pub fn update(&mut self, result: f32){
        self.simulations += 1;
        self.wins += result;
    }
}

pub struct Tree {
    move_tree: Vec<Position>,
   
   
   
}

impl Tree {
    
    pub fn add_root(&mut self, root: Board, branch: usize) {
        let root_id = PositionId {branch: branch, index: 0};
        let root_position = Position {
            id: root_id,
            node_type: NodeType::root,
            parent: None,
            child_first: None,
            child_last: None,

            
            wins: 0.0,
            simulations: 0,
            visits: 0,
            route: None,
            position: root,
        };

        self.move_tree.push(root_position);
    }

    pub fn select(&self, own_turn: bool, parent_index: usize) -> usize {
        
        //everything must have at least one visit and a win ratio
        //doesn't matter if it is white or black
        let exploration: f64 = 10.5; // confidence interval for 95
        let mut child_nodes: Vec<usize> = (self.move_tree[parent_index].child_first.unwrap().index
        .. self.move_tree[parent_index].child_last.unwrap().index).collect();

        child_nodes.shuffle(&mut thread_rng());
        // make sure this is generating a new random thread every function call
        //get the children 

        let mut high_low; 
        
        if own_turn { high_low = (0.0,0) } else  { high_low = (f64::INFINITY, 0)}
            
        for child in child_nodes {
            
            let win_ratio = self.move_tree[child].wins as f64 / self.move_tree[child].simulations as f64;
            

            let visit_term;
            // println!("{:?}", self.move_tree[child].visits);
            if self.move_tree[child].visits == 0 {
                visit_term = f64::INFINITY;
                //this can be optimized later
            }else{
                visit_term = ((
                    (self.move_tree[self.move_tree[child].parent.unwrap().index].visits as f64).ln() / 
                    (self.move_tree[child].visits as f64)
                ).sqrt()) * exploration;
            }

            
            let upper_lower = |turn: bool| -> f64{ if turn {win_ratio + visit_term} else {win_ratio - visit_term} };
            //returns a symetric upper or lower bound

            if own_turn {
                if upper_lower(true) > high_low.0 {
                    high_low = (upper_lower(true), child);
                    // println!("{:?}", high_low);
                    // println!("upper bound");
                }
            }else{
                if upper_lower(false) < high_low.0 {
                    high_low = (upper_lower(false), child);
                    // println!("{:?}", high_low);
                    // println!("lower bound");
                }
            }
        }
        //returns the index of the child with the highest score
        high_low.1
        
    }
    
    pub fn select_from_root(& mut self) -> usize{
       
        let mut node = 0;
        let mut turn = true;
        //this loop runs an infinite amount of times
        while self.move_tree[node].node_type != NodeType::leaf{
            // if a node is reached which does not have a child then it is the leaf node and the program need search no further.
            // nodes will select the root (or maybe their parent)
            node = self.select(turn, node);
            
            self.move_tree[node].visits += 1;
            
            turn = !turn;
        }
        node
    }
    
    pub fn expand(&mut self, parent_index: usize, parent_branch: usize) {
        
        //can expand on either side
        let moves = self.move_tree[parent_index].position.generate_moves();

        //re-examine later
        let first = PositionId{branch: parent_branch, index: self.move_tree.len()};
        let last = PositionId{branch: parent_branch, index: self.move_tree.len() + moves.len()};
        
        self.move_tree[parent_index].child_first = Some(first);
        self.move_tree[parent_index].child_last = Some(last);

        if self.move_tree[parent_index].node_type != NodeType::root {
            self.move_tree[parent_index].node_type = NodeType::twig;
        }
        
        // add the children in
        
        for (index, child_move) in moves.iter().enumerate() {

            let mut child_position = self.move_tree[parent_index].position.shallow_clone();
            child_position.apply_move(*child_move);

            let child_id = PositionId{branch:parent_branch, index: self.move_tree.len() + index,};
            //could this be a problem?
            let parent_id = PositionId{branch: parent_branch, index: parent_index};
            let child = Position {
                id: child_id,
                node_type: NodeType::leaf,
                parent: Some(parent_id),
                
                child_first: None,
                child_last: None,

                wins: 0.0,
                simulations: 0,

                visits: 0,
                route: Some(*child_move),
                position: child_position,
            };

            self.move_tree.push(child);
            
        }
       
    }

    pub fn simulate(&self, node_index: usize, player: pleco::Player) -> i32 {

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

        struct StateInformation {
            board: Board,
            playing: bool,
            total_iterations: usize,
            quiets_in_row: usize,
        }

        impl StateInformation {

            pub fn difference(&self, player_self: pleco::Player) -> i32{

                let opponent: pleco::Player;

                if player_self == pleco::Player::White {
                    opponent = pleco::Player::Black;
                }else{
                    opponent = pleco::Player::White;
                }
                //your material minus your opponenets material
                self.board.non_pawn_material(player_self) - self.board.non_pawn_material(opponent)
            }


            pub fn sim_result(&self, player_self: pleco::Player, initial: i32) -> Option<i32> {
                
                let mut insufficient = false;

                let insufficient_positions: [u64; 5] = [
                    14531074927864995568, //k n vs K
                    12777358361509687304, //k vs K N
                    15001244464087343965, //k vs K B
                    2182324930161781490,  //k b vs K
                    1440794681747902690,  //k vs K
                                        //needs same colored bishops
                ];
                
                if insufficient_positions.contains(&self.board.material_key()) {
                    insufficient = true;
                }
                
                let search_limit = (self.total_iterations > 15) && (self.quiets_in_row > 3); 

                if !self.board.checkmate() && !self.board.stalemate() && !insufficient && !search_limit {
                    return None;
                }else{
                    if search_limit{
                        let final_difference = self.difference(player_self);

                        if final_difference == initial{
                            return Some(0);
                        }else if final_difference > initial{
                            return Some(1);
                        }else{
                            return Some(-1);
                        }                   
                    }else if self.board.checkmate() {
                        if self.board.turn() == player_self {
                            return Some(-1);
                        }else{
                            return Some(1);
                        }
                    }else{
                        return Some(0);
                    }
                }

                // 1 a win for you, 0 a draw, -1 a loss for you

            }
        }
    
        let mut state = StateInformation{
            board: self.move_tree[node_index].position.shallow_clone(),
            playing: true,
            total_iterations: 0,
            quiets_in_row: 0,
        };

        let inital_difference = state.difference(player);

        while state.playing {

            let mut dynamic_move_list: Vec<BitMove> = vec![];

            if state.board.generate_moves().is_empty(){
                return 0; 
            }

            if state.board.in_check() {
                let evasions = state.board.generate_moves_of_type(GenTypes::Evasions);
                // evasions.iter().map(|bit_move| dynamic_move_list.push(*bit_move));

                
                for bit_move in evasions {
                    dynamic_move_list.push(bit_move);
                }

                //println!("dml-ev{:?}", dynamic_move_list);
            }else{
                let captures = state.board.generate_moves_of_type(GenTypes::Captures);

                for bit_move in captures {
                    if state.board.see_ge(bit_move, 0){
                        dynamic_move_list.push(bit_move);
                    }
                }

                //println!("dml-cap{:?}", dynamic_move_list);

                if dynamic_move_list.is_empty(){
                    let quiets = state.board.generate_moves_of_type(GenTypes::Quiets);
                    // quiets.iter().map(|bit_move| dynamic_move_list.push(*bit_move));

                    for bit_move in quiets {
                        dynamic_move_list.push(bit_move);
                    }
                    state.quiets_in_row += 1;

                    //println!("dml-qu{:?}", dynamic_move_list);

                }else{
                    state.quiets_in_row = 0;
                    // if the move list is not empty then that means that there was a capture so the streak of quiets is now zero
                }
            }

        
            // if dynamic_move_list.is_empty(){
            //     let all_moves = state.board.generate_moves();

            //     for bit_move in all_moves {
            //         dynamic_move_list.push(bit_move);
            //     }
            //     //jenky as  solution
            // }

            // if dynamic_move_list.is_empty(){
            //     if state.board.generate_moves().is_empty(){
            //         panic!("this is the fucking problem")
            //     }
            //     panic!("wtf")
            // }
            
            let move_option = dynamic_move_list.choose(&mut rand::thread_rng());
            // idk this is some how still None sometimes

            if move_option.is_none(){
                return 0
            }
            state.board.apply_move(*move_option.unwrap());
            state.total_iterations += 1;

            if !state.sim_result(player, inital_difference).is_none(){
                return state.sim_result(player, inital_difference).unwrap();
            }
        }
        //use a result type
        return 0;
    }

    pub fn backpropagate(&mut self, result: f32, child_index: usize){
        
        // goes to the parent and updates its
        // child index to parent node_inde
        self.move_tree[child_index].update(result);
        
        let mut node = self.move_tree[child_index].parent;

        while !node.is_none() {
            //while traversed node is not the root
            //mutate the parent
            //this solution does not work. change this backpropagation
            //println!("back");
            self.move_tree[node.unwrap().index].update(result);
            //increase the win ratio 
            
            //increases the visit count
            
            //node is now the id of its parent

            let node_index = node.unwrap().index;
            node = self.move_tree[node_index].parent;
        
            // if !node.is_none() {
            //     println!("node {:?}", node.unwrap().index);
                
            // }
            // the new child node is the old parent node and the new parent is the parents parent
        }


    }


}


pub fn engine(root_position: &Board, player: pleco::Player) -> BitMove {

    
    // let begin = Instant::now();

    let root_clone = root_position.shallow_clone();
    
    let mut tree = Tree{move_tree: vec![]};

    tree.add_root(root_clone, 0);

    //there must be an infinite loop somewhere in my code.
    let mut leaf = 0;
    for _iteration_ in 0..10000 {
        tree.expand(leaf,0);
        
        let child_indexes = tree.move_tree[leaf].child_first.unwrap().index..tree.move_tree[leaf].child_last.unwrap().index;
        for child in child_indexes{
            
            let result = tree.simulate(child, player) as f32;
            // println!(" sim results{:?}", result);
            
            tree.backpropagate(result, child);
            
        }

        leaf = tree.select_from_root();

    }

    // this will need to me changed when the tree is parallelized
    let final_indexes = tree.move_tree[0].child_first.unwrap().index..tree.move_tree[0].child_last.unwrap().index;

    let mut highest = (0,0);
    // (simulations, index)
    for index in final_indexes {
        // println!("index {:?}", index);
        // println!("simulations {:?}", tree.move_tree[index].simulations);
        if tree.move_tree[index].simulations > highest.1 {
            highest.0 = index;
            highest.1 = tree.move_tree[index].simulations;
        }
    }


    let best_move = tree.move_tree[highest.0].route.unwrap();
    println!("{:?}", best_move.stringify());

    // let end = Instant::now();

    // let time = end.duration_since(begin);

    // println!("total: {}", tree.move_tree.len());
    
    // println!("time: {:?}", time);

    return best_move
    
}



