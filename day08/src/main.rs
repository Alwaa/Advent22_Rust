use std::io::{Error};


fn main() -> Result<(),Error> {
    let path = std::env::current_dir().expect("CURRENT WORKING DIR");
    println!("{}", path.display());
    let file_path = std::path::Path::new("../input22-08.txt"); //Path relative to proj folder...
    
    let input = std::fs::read_to_string(file_path).unwrap();
    let out_tup = string_to_matrix(input);
    let yes = out_tup.0;
    let max_scenic = visible_max(&yes);
    let visible = visible_trees((yes, out_tup.1));
    let num_viz = count_true(visible);

    println!("Max scenic score is: {}", max_scenic);
    println!("Number of visible trees are: {}", num_viz);

    Ok(())
}

fn count_true(viz_mat: Vec<Vec<bool>>) -> i32 {
    let mut counter = 0;
    for li in viz_mat {
        counter += li.into_iter().filter(|b| *b).count();
    }
    return counter.try_into().unwrap()
}


fn visible_max(heights: &Vec<Vec<i32>>) -> i32 {
    let x_len = heights[0].len() as usize;
    let y_len = heights.len() as usize;
    let mut curr_max: i32 = 0; 

    for x in 0..x_len{
        for y in 0..y_len{
            let curr_height = heights[y][x];
            let mut curr_score = [0;4];

            for yrell in (y+1)..y_len {
                curr_score[0] += 1;
               if heights[yrell][x] >= curr_height {
                   break
               }
            }
            for yrell in (0..y).rev() {
                curr_score[1] += 1;
               if heights[yrell][x] >= curr_height {
                   break
               }
            }

            for xrell in (x+1)..x_len {
                curr_score[2] += 1;
               if heights[y][xrell] >= curr_height {
                   break
               }
            }
            for xrell in (0..x).rev() {
                curr_score[3] += 1;
               if heights[y][xrell] >= curr_height {
                   break
               }
            }

            //println!("{:?}",curr_score);
            let scenic_score = curr_score.into_iter().product();
            if curr_max < scenic_score {
                curr_max = scenic_score;
            }

        }
    }
    return curr_max 
}

fn visible_trees(in_tup: (Vec<Vec<i32>>,Vec<Vec<bool>>)) -> Vec<Vec<bool>> {
    let heights = in_tup.0;
    let mut vizible = in_tup.1;
    let x_len = heights[0].len() as usize;
    let y_len = heights.len() as usize;
    let mut curr_tallets = [zeros(x_len), zeros(x_len), zeros(y_len), zeros(y_len)]; 

    for x in 0..x_len{
        let xrev = (x_len - 1) - x;
        for y in 0..y_len{
            let yrev = (y_len - 1) -y;

            if heights[y][x] > curr_tallets[0][x]{
                curr_tallets[0][x] = heights[y][x];
                vizible[y][x] = true;
            } 
            if heights[yrev][xrev] > curr_tallets[1][xrev]{
                curr_tallets[1][xrev] = heights[yrev][xrev];
                vizible[yrev][xrev] = true;
            } 
            if heights[y][x] > curr_tallets[2][y]{
                curr_tallets[2][y] = heights[y][x];
                vizible[y][x] = true;
            } 
            if heights[yrev][xrev] > curr_tallets[3][yrev]{
                curr_tallets[3][yrev] = heights[yrev][xrev];
                vizible[yrev][xrev] = true;
            } 
        }
    }
    for li in vizible.iter() {
        //println!("{:?}", li);
    }
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
    let mut yvec: Vec<Vec<i32>> = Vec::with_capacity(y_len);
    let mut bool_mat: Vec<Vec<bool>> = Vec::with_capacity(y_len);
    for line in lines {
        let xvec: Vec<i32> = line.chars().map(|ch| {ch.to_digit(RADIX).unwrap() as i32}).collect();
        yvec.push(xvec);
        bool_mat.push(vec![false; x_len])
    }
    return (yvec, bool_mat)
}


