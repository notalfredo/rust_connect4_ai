use std::collections::{HashMap, hash_map};
#[derive(Debug)]
pub struct GameState{
    pub action : usize,
    pub game_state : [ [usize; 7]; 6],
    pub player_num : usize,
    pub action_map : HashMap<usize, i64>,
    pub opp_number : usize,
}




impl GameState {
    pub fn initialize(set_action : usize, set_game_state: [[usize; 7]; 6], set_player_num : usize, set_action_map : HashMap<usize, i64> ) -> GameState {
        GameState {
            action : set_action,
            game_state : set_game_state,
            player_num : set_player_num,
            action_map : set_action_map,
            opp_number : if set_action == 1 {2} else {1}
        }
    }

    pub fn set_game_state(&mut self, row:usize, column: usize, value: usize){
        self.game_state[row][column] = value;
    }
    pub fn get_game_state(&self) -> [ [usize; 7]; 6]{
        return self.game_state;
    }
    pub fn set_entire_matrix(&mut self, matrix: &[[usize;7];6] ){
        self.game_state = *matrix;
    }   


    pub fn get_action(&self) -> usize{
        return self.action;
    }

    pub fn set_action_dict(&mut self, key: usize, value: i64) {
        self.action_map.insert(key , value);
    }
    pub fn get_action_dict(&self) -> HashMap<usize, i64>{
        return self.action_map.clone();
    }

    pub fn set_player_num(&mut self, num: usize){
        self.player_num = num;
    }
    pub fn get_player_num(&self) -> usize{
        return self.player_num;
    }


    pub fn set_opp_number(&mut self, num: usize){
        self.opp_number = num
    }
    pub fn get_opp_number(&self) -> usize {
        return self.opp_number;
    }

        

    
}

