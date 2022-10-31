use std::{collections::{HashMap, hash_map}, hash::Hash, env, os::unix::process, convert};
use std::any::type_name;


mod state;
mod tableManipulation;
mod alpha_beta;

use state::*;
use tableManipulation::*;
use alpha_beta::*;


fn interactive_mode(input_file: &str, next: &str, depth: &str){
    //println!("{} {} {}", input_file, next, depth.parse::<usize>().unwrap());
    let mut start : &str = ""; 
    let mut human_num : usize = 0;
    let mut computer_num : usize = 0;
    let mut go_to_number : usize = 0;

    if next == "computer-next"{
        start = "Computer";
        computer_num = 1;
        human_num = 2;
        go_to_number = 2;
        println!("computer WAS NEXT");
    }

    if next == "human-next"{
        start = "Human";
        human_num = 1;
        computer_num = 2;
        go_to_number = 5;
        println!("HUMAN WAS NEXT");
    }

    let mut game_table : [[usize;7];6] = [[0; 7] ; 6];
    let mut turn = convert_to_matrix(&mut game_table, input_file);

    println!("{:?}" , game_table);
    println!("{}", turn);

    loop{

        if go_to_number == 2{
            print_table(&game_table);
            check_curr_score(&game_table, start);

            if check_if_game_is_over(&game_table){
                println!("The board is full thus game is over");
                break;
            }
            println!("Computer is thinking ...");
            
            let my_hash_map: HashMap <usize, i64> = HashMap::new();
            let mut test_struct = state::GameState{
                action: 0,//
                game_state: pass_array_by_value(&game_table),//----
                player_num: computer_num,//
                action_map : my_hash_map,//
                opp_number : if  computer_num== 1 {2} else {1},//
            };
            
            let column_insertion = alpha_beta_decision(&mut test_struct, depth.parse::<i8>().unwrap());
            instert_column(&mut game_table, column_insertion, test_struct.get_player_num());
            write_to_file(&game_table, test_struct.get_opp_number(), input_file);
            go_to_number = 5;
        }

        if go_to_number == 5{
            print_table(&game_table);
            check_curr_score(&game_table, start);
            if check_if_game_is_over(&game_table){
                println!("Board is full");
                break;
            }

            //////////////////////////////////////////////////////////
            
            loop
            {
                let mut x = String::new();
                println!("Enter integer range 1-7");
                match std::io::stdin().read_line(&mut x) {
                    Ok(_) => print!(""),
                    Err(_) => {
                        print!("Failed to read input.");
                    },
                };
                let x: usize = match x.trim().parse() {
                    Ok(n) => {
                        print!("");
                        n
                    },
                    Err(_) => {
                        println!("Enter integer range 1-7");
                        continue;
                    },
                };
                if x <= 0 || x >= 8{
                    println!("Enter integer range 1-7");
                    continue;
                }
                else if check_if_colum_valid_choice(&game_table, x-1) == false{
                    println!("Enter a valid column choice {} , is full", x);
                    continue;
                }
                else{
                    println!("I AM SINDE OF here");
                    instert_column(&mut game_table, x-1, human_num);
                    break;
                }

            }
            println!("I AM OUTISDE");
            ////////////////////////////////////////////////////////
            //write_to_file(&game_table, input_file, depth.parse::<usize>().unwrap());
            write_to_file(&game_table, computer_num, input_file);
            go_to_number = 2;
        }   
    }
}


fn one_mode(input_file: &str, output_file: &str, depth: &str){
    let mut start : &str = "";

    let mut game_table : [[usize;7];6] = [[0; 7] ; 6];
    let mut turn = convert_to_matrix(&mut game_table, input_file);

    if turn == 1{
        start = "Computer";
    }
    else if turn == 2{
        start = "Human";
    }

    print_table(&game_table);
    check_curr_score(&game_table, start);
    if check_if_game_is_over(&game_table) == true{
        println!("Board is full game over");
        return;
    }

    let my_hash_map: HashMap <usize, i64> = HashMap::new();
    let mut test_struct = state::GameState{
        action: 0,//
        game_state: pass_array_by_value(&game_table),//----
        player_num: turn,//
        action_map : my_hash_map,//
        opp_number : if turn == 1 {2} else {1},//
    };

    println!("MY NUM {} OPP NUM {}", test_struct.get_player_num(), test_struct.get_opp_number());
    let column_insertion = alpha_beta_decision(&mut test_struct, depth.parse::<i8>().unwrap());
    instert_column(&mut game_table, column_insertion, test_struct.get_player_num());
    write_to_file(&game_table, test_struct.get_opp_number(), output_file)



}

fn main() {
    /* 
    let myaction = 1;
    let mut mystate = [[0; 7] ; 6];
    let myplayer_num = 1;
    let mytest_map: HashMap <usize, i64> = HashMap::new();
    
    let mut test_struct = state::GameState{
        action: myaction,
        game_state: mystate,
        player_num: myplayer_num,
        action_map : mytest_map,
        opp_number : if myaction == 1 {2} else {1},
    };

    println!("action = {:?}", test_struct.get_action());
    println!("action dict = {:?}", test_struct.get_action_dict());
    println!("game = {:?}", test_struct.get_game_state());
    println!("opp num = {:?}", test_struct.get_opp_number());
    println!("");

    tableManipulation::convert_to_matrix(&mut mystate , "input1.txt");

    //tableManipulation::write_to_file(&mystate, "test.txt", test_struct.get_player_num() );
    test_struct.set_entire_matrix(&mystate);
    
    //tableManipulation::instert_column(&mut mystate, 0, 2);
    println!("{:?}" , mystate);
    println!("{:?}", test_struct.get_game_state());

    
    tableManipulation::write_to_file(&test_struct.get_game_state(), "test.txt", test_struct.get_player_num() );


    println!("{}", tableManipulation::check_if_colum_valid_choice(&mystate, 0));
    tableManipulation::print_table(&mystate);
    println!("{}", tableManipulation::check_if_game_is_over(&mystate));
    tableManipulation::check_curr_score(&mystate, 1);
    //tableManipulation::instert_column(table, 0, 1)

    alpha_beta::alpha_beta_decision(&mut test_struct, 9);


    let test_suc = alpha_beta::successors(&test_struct, 1);

    println!("{:?}" , test_suc[0]);
    println!("{:?}" , test_suc[1]);
    println!("{:?}" , test_suc[2]);
    println!("{:?}" , test_suc[3]);
    println!("{:?}" , test_suc[4]);
    println!("{:?}" , test_suc[5]);
    println!("{:?}" , test_suc[6]);
    println!("{}", test_suc.len());

    println!("utility {}" , alpha_beta::utility(&test_struct.get_game_state(), test_struct.get_player_num()));
    */

    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    //println!("mode {}", query);
    //println!("In file {}", file_path);
    println!("lengh of arg = {} ", args.len());
    if args.len() < 5{
        print!("Not enough command line arguments exiting...");
        std::process::exit(1)
    }

    if &args[2] == "interactive"{
        interactive_mode( &args[2],  &args[3],  &args[4]);
    }
    
    if &args[2] == "one-mode"{
        println!("enter one mode");
        one_mode(&args[2],  &args[3],  &args[4])
    }
    one_mode(&args[2],  &args[3],  &args[4])

    //let x = get_user_input();
    

}
