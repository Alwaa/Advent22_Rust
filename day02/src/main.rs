use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

#[derive(Debug,Copy,Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn main() -> Result<(),Error> {
    let path = std::env::current_dir().expect("CURRENT WORKING DIR");
    println!("{}", path.display());
    let file_path = std::path::Path::new("../input22-02.txt"); //Path relative to proj folder...
    
    let input = File::open(&file_path)?;
    let buffered_reader = BufReader::new(input);

    let mut cum_score = 0;
    let mut strat_cum_score = 0;

    for line in buffered_reader.lines() {
        let line_string = line?; 
        let move_vector: Vec<&str> = line_string.split(" ").collect();

        let opponent = string_to_move(move_vector[0]);
        let my_move = string_to_move(move_vector[1]);
        let my_strat_move = move_and_stringstrat_to_move(&opponent, move_vector[1]);

        let point = point_from_match(&opponent,&my_move);
        let strat_point = point_from_match(&opponent,&my_strat_move);
        //println!("{:#?}  vs.  {:#?}  || {:#?}",opponent, my_move, point);
        //println!("{:#?}  vs.  {:#?}  || {:#?}",opponent, my_strat_move, strat_point);
        cum_score += point;
        strat_cum_score += strat_point;
    }

    println!("Total Score: {}", cum_score);
    println!("Score when using strategy: {}", strat_cum_score);

    Ok(())
}



fn string_to_move(inn: &str) -> Move {
    match inn {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _ => panic!("Not a valid string to move"),
    }
}

fn move_and_stringstrat_to_move(opponent_move: &Move, strat:&str) ->  Move {
    if strat == "Y" {
        return  *opponent_move;
    } else {
        let opponent_int = *opponent_move as i32;
        
        let my_move_int = 
            if strat == "X" {
                ((opponent_int-1) - 1).rem_euclid(3) + 1 
            }else {
                ((opponent_int-1) + 1).rem_euclid(3) + 1 
        };

        if (my_move_int < 1) || (my_move_int > 4){
            panic!("Not valid move transtaltion from modular arithmatic");
        } else {
            //Unsafe assignment to Move enum
            let out_move: Move = unsafe { std::mem::transmute(my_move_int as u8) };
            return out_move;
        }


    }
}

fn point_from_match(move_a: &Move, move_b: &Move) -> i32 {
    let move_a_int = *move_a as i32;
    let move_b_int = *move_b as i32;
    if move_a_int == move_b_int {
        return 3 + move_b_int;
    } else {
        let modulus_move = (move_a_int-move_b_int).rem_euclid(3);
        if modulus_move == 1 {
            return 0 + move_b_int; // I lose
        } else {
            return 6 + move_b_int; //I win
        }
    }
}

