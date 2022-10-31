use crate::tableManipulation;
use crate::{state::GameState, tableManipulation::check_if_colum_valid_choice};
use std::collections::HashMap;
use std::vec;
use crate::tableManipulation::*;
use crate::state::*;



pub fn alpha_beta_decision(state: &mut GameState, depth_limit: i8)->usize{
    //let 
    let a = max_value(&mut *state, std::i64::MIN, std::i64::MAX, depth_limit);

    //let count_vec: HashMap<usize, i64> = state.get_action_dict().iter().collect();
    
    //println!("{:?}", state.get_action_dict());
    //println!("{:?}",get_max_index_value_from_dict(&state.get_action_dict()));

    return get_max_index_value_from_dict(&state.get_action_dict());
}


pub fn max_value(state: &mut GameState, alpha: i64, beta: i64, depth_limit: i8) -> i64{
    let mut mut_alpha = alpha;
    if depth_limit == 0 {
        return utility(&state.get_game_state(), state.get_player_num());
    }

    let mut v = std::i64::MIN;
    let mut succ_array = successors(&state, state.get_player_num());
    let mut i : usize = 0;

    while i < succ_array.len(){
        v = max(v, min_value(&mut succ_array[i], mut_alpha, beta, depth_limit-1));

        state.set_action_dict(succ_array[i].get_action(), v);

        if v >= beta{
            return v;
        }
        mut_alpha = max(mut_alpha, v);
        i +=1;
    }

    return v;
}

pub fn min_value(state: &mut GameState, alpha: i64, beta: i64, depth_limit: i8) -> i64{
    let mut mut_beta = beta;
    if depth_limit == 0{
        return utility(&state.get_game_state(), state.get_player_num());
    }

    let mut v = std::i64::MAX;
    let mut succ_array = successors(&state, state.get_opp_number());
    let mut i : usize = 0;

    while i < succ_array.len(){
        v = min(v, max_value(&mut succ_array[i], alpha, mut_beta, depth_limit-1));

        state.set_action_dict(succ_array[i].get_action() , v);
        if v <= alpha{
            return v;
        }
        mut_beta = min(mut_beta, v);
        i+=1;
    }

    return v;
}

pub fn successors(state: &GameState, turn: usize) -> Vec<GameState>{
    let mut sucessor_vec: Vec<GameState> = Vec::new();

    let mut i : usize = 0;
    while i < 7{
            if check_if_colum_valid_choice(&state.get_game_state(), i){
                let mut new_state = GameState{
                    action: i,
                    game_state: tableManipulation::instert_column_new_array(&state.get_game_state(), i, turn),
                    player_num: state.get_player_num(),
                    action_map : HashMap::new(),
                    opp_number : if state.get_player_num() == 1 {2} else {1},
                };
                sucessor_vec.push(new_state);
            }        
        i += 1;
    }
    return sucessor_vec;
}

pub fn utility(table : &[[usize; 7];6], player_num: usize) -> i64{
    let mut second_player : usize = 0;
    let mut score_1 : i64 = 0;
    let mut score_2 : i64 = 0;
    if player_num == 1{
        second_player = 2;
    }
    else if player_num == 2{
        second_player = 1;
    }

    let mut i : usize = 0;
    let mut j : usize = 0;
    while i < 6{
        while j < 7{
            if table[i][j] == player_num{
                score_1 += tableManipulation::check_match(table, i, j)
            }
            if table[i][j] == second_player{
                score_2 += tableManipulation::check_match(table, i, j)
            }
            j += 1;
        }
        j = 0;
        i += 1;
    }

    return score_1 - score_2;
}

pub fn max(alpha: i64, v: i64) -> i64{
    if v > alpha{
        return v;
    }
    return alpha;
}

pub fn min(beta: i64, v: i64) -> i64{
    if v < beta{
        return v;
    }
    return beta;
}
//get_max_index_value_from_dict
pub fn get_max_index_value_from_dict(map: &HashMap<usize, i64>)-> usize{
    let mut value : i64 = 0;
    let mut key : usize = 0;
    let mut count : usize = 0;

    for (map_key, map_value) in map.into_iter() {
        if count == 0{
            value = *map_value;
            key = *map_key;
            count += 1;
        }
        if *map_value > value{
            value = *map_value;
            key = *map_key;
        }
        if *map_value == value && *map_key < key{
            value = *map_value;
            key = *map_key;
        }       
        println!("{} / {}", map_key, map_value);
    }
    return key;
}


