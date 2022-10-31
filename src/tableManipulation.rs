use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};


pub fn convert_to_matrix(arr: &mut [ [usize; 7]; 6], file_name: &str) -> usize {
    let mut turn : usize = 0;
    let mut counter : usize = 0;


    let filename = file_name;
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);


    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        if counter < 6{
            let index_one : usize = line[0..1].parse().unwrap();
            arr[counter][0] = index_one;
            let index_two : usize = line[1..2].parse().unwrap();
            arr[counter][1] = index_two;
            let index_three : usize = line[2..3].parse().unwrap();
            arr[counter][2] = index_three;
            let index_four : usize = line[3..4].parse().unwrap();
            arr[counter][3] = index_four;
            let index_five : usize = line[4..5].parse().unwrap();
            arr[counter][4] = index_five;
            let index_six : usize = line[5..6].parse().unwrap();
            arr[counter][5] = index_six;
            let index_seven : usize = line[6..7].parse().unwrap();
            arr[counter][6] = index_seven;
            //println!("{}. {} {} {} {} {} {} {} ", index + 1, index_one, index_two, index_three, index_four, index_five, index_six, index_seven);
            counter += 1;
            //println!("INSIDE");
        }
        else{

            turn = line[0..1].parse().unwrap();
            //println!("{} {}", index +1 , turn_number);
            break;
        }

    }
    return turn;
}



pub fn write_to_file(arr: &[ [usize; 7]; 6], app_num : usize, file_name: &str){

    //let arr  = [[1, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0]];

    let path = file_name;
    let f = File::create(path).expect("unable to create file");
    let mut f = BufWriter::new(f);

    let mut i : usize = 0;
    let mut j : usize = 0;
    while i < 6 {
        while j < 7 {

            write!(f, "{}", arr[i][j]).expect("unable to write");
            if j == 6{
                write!(f, "\n").expect("unable to write");
            }
            j += 1
        }
        j = 0;
        i += 1;
    }
    write!(f, "{}", app_num).expect("unable to write");
    write!(f, "\n").expect("unable to write");
}


//pub something
pub fn instert_column(table: &mut [[usize; 7]; 6], column: usize, turn: usize){
    
    let mut i : usize = 5;
    while i >= 0 {
        if table[i][column] == 0{
            table[i][column] = turn;
            break;
        }
        i -= 1;
    }
}




pub fn check_if_colum_valid_choice(table: &[[usize; 7]; 6], column: usize) -> bool{
    let mut i: usize = 0;

    while i < 6 {
        if table[i][column] == 0 {
            return true;
        }else{
            //println!("Not valid choice");
            return false;
        }

        i += 1;
    }
    return false;
}


pub fn check_match(table: &[[usize;7];6], r: usize, c: usize) -> i64{
    let rowMin:i128 = 0;
    let rowMax: i128 = 6;
    let columnMin:i128 = 0;
    let columnMax:i128 = 7;

    let mut score : i64 = 0;

    if (c as i128 + 3) < columnMax {
        if table[r][c] == table[r][c+1] && table[r][c] == table[r][c+2] && table[r][c] == table[r][c+3]{
            //print("Right score ");
            score += 10;
        }
    }
    if (r as i128 + 3) < rowMax{
        if table[r][c] == table[r+1][c] && table[r][c] == table[r+2][c] && table[r][c] == table[r+3][c]{
            //rint("down score ")
            score += 10;
        }
    }
    if (r as i128 + 3) < rowMax && (c as i128 + 3) < columnMax{
        if table[r][c] == table[r+1][c+1] && table[r][c] == table[r+2][c+2] && table[r][c] == table[r+3][c+3]{
            //print("down score ")
            score += 10
        }
    }       
    if (r as i128 - 3) >= rowMin && (c as i128 + 3) < columnMax{
        if table[r][c] == table[r-1][c+1] && table[r][c] == table[r-2][c+2] && table[r][c] == table[r-3][c+3]{
            //print("down score ")
            score += 10  ;
        }
    }
//////////////////////////////////////////////////////////////////////
    if (c as i128 + 2) < columnMax {
        if table[r][c] == table[r][c+1] && table[r][c] == table[r][c+2]{
            //print("Right score ");
            score += 4;
        }
    }
    if (r as i128 + 2) < rowMax{
        if table[r][c] == table[r+1][c] && table[r][c] == table[r+2][c]{
            //rint("down score ")
            score += 4;
        }
    }
    if (r as i128 + 2) < rowMax && (c as i128 + 2) < columnMax{
        if table[r][c] == table[r+1][c+1] && table[r][c] == table[r+2][c+2]{
            //print("down score ")
            score += 4
        }
    }       
    if (r as i128 - 2) >= rowMin && (c as i128 + 2) < columnMax{
        if table[r][c] == table[r-1][c+1] && table[r][c] == table[r-2][c+2]{
            //print("down score ")
            score += 4 ;
        }
    }
//////////////////////////////////////////////////////////////////////
    if (c as i128 + 1) < columnMax {
        if table[r][c] == table[r][c+1]{
            //print("Right score ");
            score += 1;
        }
    }
    if (r as i128 + 1) < rowMax{
        if table[r][c] == table[r+1][c]{
            //rint("down score ")
            score += 1;
        }
    }
    if (r as i128 + 1) < rowMax && (c as i128 + 1) < columnMax{
        if table[r][c] == table[r+1][c+1]{
            //print("down score ")
            score += 1
        }
    }       
    if (r as i128 - 1) >= rowMin && (c as i128 + 1) < columnMax{
        if table[r][c] == table[r-1][c+1]{
            //print("down score ")
            score += 1 ;
        }
    }

    return score

}

//pub
pub fn print_table(table: &[[usize;7];6]){
    println!("Table : ");
    println!("---------------------");
    let mut i : usize = 0;
    let mut j : usize = 0;

    while i < 6{
        while j < 7{
            print!("{} |", table[i][j]);
            j += 1;
        }
        j = 0;
        i += 1;
        print!("\n");
        println!("---------------------");
    }


}

pub fn check_if_game_is_over(table: &[[usize;7];6]) -> bool{
    let mut i : usize = 0;
    let mut j : usize = 0;

    while i < 6{
        while j <7{
            if table[i][j] == 0{
                return false
            }
            j += 1;
        }
        j = 0;
        i += 1;
    }
    return true

}

pub fn check_curr_score(game: &[[usize;7];6], first_player: & str){
    let mut score : usize = 0;
    let mut score2: usize = 0;

    let mut i : usize = 0;
    let mut j : usize = 0;

    while i < 6{
        while j < 7{
            if game[i][j] == 1{
                score += check_four_in_row(*game, i, j)
            }
            else if game[i][j] == 2{
                score2 += check_four_in_row(*game, i, j)
            }
            j += 1;
        }
        j = 0;
        i +=1;
    }
    println!("Score : {} - {}", score, score2);
    println!("({})" , first_player);

}

pub fn check_four_in_row(table: [[usize;7];6], r:usize, c:usize,) -> usize{
    let rowMin:i128 = 0;
    let rowMax: i128 = 6;
    let columnMin:i128 = 0;
    let columnMax:i128 = 7;

    let mut score : usize = 0;

    if (c as i128 + 3) < columnMax {
        if table[r][c] == table[r][c+1] && table[r][c] == table[r][c+2] && table[r][c] == table[r][c+3]{
            //print("Right score ");
            score += 1;
        }
    }
    if (r as i128 + 3) < rowMax{
        if table[r][c] == table[r+1][c] && table[r][c] == table[r+2][c] && table[r][c] == table[r+3][c]{
            //rint("down score ")
            score += 1;
        }
    }
    if (r as i128 + 3) < rowMax && (c as i128 + 3) < columnMax{
        if table[r][c] == table[r+1][c+1] && table[r][c] == table[r+2][c+2] && table[r][c] == table[r+3][c+3]{
            //print("down score ")
            score += 1
        }
    }       
    if (r as i128 - 3) >= rowMin && (c as i128 + 3) < columnMax{
        if table[r][c] == table[r-1][c+1] && table[r][c] == table[r-2][c+2] && table[r][c] == table[r-3][c+3]{
            //print("down score ")
            score += 1;
        }
    }
    return score;
}


pub fn pass_array_by_value(table: &[[usize; 7]; 6]) ->[[usize; 7]; 6]{
    let mut new_arary : [[usize;7];6] = [[0;7];6];
    let mut i :usize = 0;
    let mut j :usize = 0;

    while i < 6{
        while j < 7{
            new_arary[i][j] = table[i][j];
            j += 1;
        }
        j = 0;
        i += 1;
    }
    return new_arary;
}


pub fn instert_column_new_array(table: &[[usize; 7]; 6], column: usize, turn: usize)->[[usize;7];6]{
    let mut new_array : [[usize;7];6] = [[0;7];6];
    let mut i :usize = 0;
    let mut j :usize = 0;

    while i < 6{
        while j < 7{
            new_array[i][j] = table[i][j];
            j += 1;
        }
        j = 0;
        i +=1;
    }

    i = 5;
    while i >= 0 {
        if new_array[i][column] == 0{
            new_array[i][column] = turn;
            break;
        }
        i -= 1;
    }
    return new_array;
}

pub fn get_user_input()->usize{
    loop{
        println!("Enter integer range 1-7");
        let mut x = String::new();
        match std::io::stdin().read_line(&mut x) {
            Ok(_) => print!(""),
            Err(_) => {
                print!("Failed to read input.");
                return 0;
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
        if x < 1 || x > 7{
            println!("Enter integer range 1-7");
            continue;
        }
        return x
    }
}