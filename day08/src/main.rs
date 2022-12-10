use std::io::{Error};


fn main() -> Result<(),Error> {
    let path = std::env::current_dir().expect("CURRENT WORKING DIR");
    println!("{}", path.display());
    let file_path = std::path::Path::new("../input22-test.txt"); //Path relative to proj folder...
    
    let input = std::fs::read_to_string(file_path).unwrap();
    let out_tup = string_to_matrix(input);
    let visible = visible_trees(out_tup);

    Ok(())
}

fn visible_trees(in_tup: (Vec<Vec<i32>>,Vec<Vec<bool>>)) -> Vec<Vec<bool>> {
    let heights = in_tup.0;
    let mut vizible = in_tup.1;
    let x_len = heights[0].len() as usize;
    let y_len = heights.len() as usize;
    let mut curr_tallets = [zeros(x_len), zeros(x_len), zeros(y_len), zeros(y_len)]; 

    for x in 0..x_len{
        let xrev = x_len - 1 - x;
        for y in 0..y_len{
            let yrev = y_len - 1 -x;

            if heights[y][x] > curr_tallets[0][x]{
                curr_tallets[0][x] = heights[y][x];
                vizible[y][x] = true;
            } 
        }
    }
    println!("{:?}", vizible);
    return vizible
}

fn zeros(size: usize) -> Vec<i32> {
    vec![-1; size ]
}

fn string_to_matrix(input: String) -> (Vec<Vec<i32>>,Vec<Vec<bool>>){
    let total = input.len();
    let mut lines = input.lines().peekable();
    const RADIX:u32 = 10;

    let first = lines.peek().unwrap();
    let x_len = first.len();
    let y_len = total/x_len;
    println!("{},{},{}",x_len,y_len,total);
    let mut yvec: Vec<Vec<i32>> = Vec::with_capacity(y_len);
    let mut bool_mat: Vec<Vec<bool>> = Vec::with_capacity(y_len);
    for line in lines {
        let xvec: Vec<i32> = line.chars().map(|ch| {ch.to_digit(RADIX).unwrap() as i32}).collect();
        yvec.push(xvec);
        bool_mat.push(vec![false; x_len])
    }
    return (yvec, bool_mat)
}


