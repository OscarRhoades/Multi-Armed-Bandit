pub use pleco::BitMove;
pub use pleco::Board;
pub use pleco::Player;
pub use pleco::core::GenTypes;
use std::time::{Duration, Instant};

//use pleco::Player;
use rand::Rng;

//eval
//search
//time manager

// fn eval(board: &Board) -> i32 {
//     let mut value = 0;
//     value += board.non_pawn_material(Player::White) - board.non_pawn_material(Player::White);
//     return value;
// }


pub struct UniversalTree{
    sub_trees: Vec<SubTree>,
}

#[derive(Copy, Clone)]
pub struct PositionId{
    
    index: usize,
}

pub struct Position {
    id: PositionId,
    parent: Option<PositionId>,
    sibling_first: Option<PositionId>,
    sibling_last: Option<PositionId>,
    child_first: Option<PositionId>,
    child_last: Option<PositionId>,
    /// The actual data which will be stored within the tree
    pub evaluation: Option<f32>,
    pub position: Board,
}

pub struct SubTree {
    move_tree: Vec<Position>,
}

impl SubTree {
    //sub tree has to have a toor
    pub fn add_root(&mut self, root: Board) {
        let root_id = PositionId {index: 0};
        let root_position = Position {
            id: root_id,
            parent: None,
            sibling_first: None,
            sibling_last: None,
            child_first: None,
            child_last: None,

            evaluation: None,
            position: root,
        };

        self.move_tree.push(root_position);
    }
    
    pub fn add_children(&mut self, parent_index: usize) {
        //adds the new positions to the tree
        //root node should always be first
        let mut moves = vec![];
        if self.move_tree[parent_index].position.in_check() {
            for bit_move in self.move_tree[parent_index].position.generate_moves_of_type(GenTypes::Evasions) {
                moves.push(bit_move);
            }
        }else{
            let captures = self.move_tree[parent_index].position.generate_moves_of_type(GenTypes::Captures);
            for bit_move in captures{
                let mut test_position = self.move_tree[parent_index].position.shallow_clone();
                
                if test_position.see_ge(bit_move, 0){
                    moves.push(bit_move);
                }
            }                
            if moves.is_empty(){
                moves = self.move_tree[parent_index].position.generate_moves_of_type(GenTypes::Quiets);
            }
        }

       

        let sub_layer_first = PositionId{index: self.move_tree.len()};
        let sub_layer_last = PositionId{index: self.move_tree.len() + moves.len()};

        //updates the child positions based on how much longer the total vector would be

        self.move_tree[parent_index].child_first = Some(sub_layer_first);
        self.move_tree[parent_index].child_last = Some(sub_layer_last);
        //Order(moves)
        //must have move ordering to make this work
        let mut index = 0;
        for child_move in moves.iter() {

            let mut child_position = self.move_tree[parent_index].position.shallow_clone();
            child_position.apply_move(*child_move);

            let child_id = PositionId{index: self.move_tree.len() + index,};
            let parent_id = PositionId{index: parent_index};
            let child = Position {
                id: child_id,
                parent: Some(parent_id),
                sibling_first: Some(sub_layer_first),
                sibling_last: Some(sub_layer_last),
                child_first: None,
                child_last: None,

                evaluation: None,
                position: child_position,
            };

            index += 1; //make this an enumerate
            self.move_tree.push(child);
        }
    }

    pub fn create_subtree(&mut self, parent: PositionId, depth: usize){
        self.add_children(parent.index);
        let mut layer = self.move_tree[parent.index].child_first.unwrap().index 
        .. self.move_tree[parent.index].child_last.unwrap().index;
        for _ in 0..depth {
            let begin = self.move_tree.len();
            for index in layer.clone(){
                self.add_children(index);
            }
            let end = self.move_tree.len();

            layer = begin..end;
        }
        


    }
   
}




pub fn init_tree(root_position: Board) {
    let begin = Instant::now();
    
    let mut universal_tree = UniversalTree{sub_trees: vec![]};



    let mut root_tree = SubTree{move_tree: vec![]};
    root_tree.add_root(root_position);
    
    


    
    let end = Instant::now();

    let time = end.duration_since(begin);

    let mut iterations = 0;
    
   
    println!("iterations: {}", iterations);
    println!("time: {:?}", time);
    
}

// pub fn engine(board: &Board) -> BitMove {

// }

pub fn choice(board: &Board) -> BitMove {
    let mut rng = rand::thread_rng();
    let moves = board.generate_moves();
    moves[rng.gen_range(0..moves.len())]
}
