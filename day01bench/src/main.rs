use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

use std::time::{Duration, Instant};

fn main() -> Result<(),Error> {
    let path = std::env::current_dir().expect("CURRENT WORKING DIR");
    println!("{}", path.display());
    
    let start_buff = Instant::now();
    for _i in 0..100 {
        let file_path = std::path::Path::new("../input22-01.txt"); //Path relative to proj folder...
        buff_test(file_path)?;
        let file_path = std::path::Path::new("../input22-test.txt"); //Path relative to proj folder...
    }
    let duration_buff = start_buff.elapsed();



    let start = Instant::now();
    for _i in 0..100 {
        let file_path = std::path::Path::new("../input22-01.txt"); //Path relative to proj folder...
        string_test(file_path)?;
        let file_path = std::path::Path::new("../input22-test.txt"); //Path relative to proj folder...
    }
    let duration = start.elapsed();

    println!("Time elapsed in buff_test is: {:?}", duration_buff);
    println!("Time elapsed in string_test is: {:?}", duration);
    Ok(())
}

fn buff_test(f_path: &std::path::Path) -> Result<(),Error> {
    let input = File::open(f_path)?;
    let buffered_reader = BufReader::new(input);

    let mut curr_max = 0;
    let mut curr_cum = 0;
    let mut top3 = [0,0,0];
    
    //Would be interesting to benchmark agains read_to_string //TODO: 
    for line in buffered_reader.lines() {
        let line_string = line?;
        if &line_string != "" {
            curr_cum += line_string.parse::<i32>().unwrap();
        } else {
            if curr_max < curr_cum {
                curr_max = curr_cum;
            }
            
            if top3[0] < curr_cum {
                top3[0] = curr_cum;
            }
            for i in 0..(top3.len() - 1){
                if top3[i]>top3[i+1] {
                    let temp = top3[i];
                    top3[i] = top3[i+1];
                    top3[i+1] = temp;
                }
            }

            curr_cum = 0;
        }

    };
    
    let sum_top3:i32= top3.iter().sum();
    println!("Max is: {}",curr_max);
    println!("Top 3 are {:#?}", top3);
    println!("Sum of top 3 is {}", sum_top3);
    Ok(())
}

fn string_test(f_path: &std::path::Path) -> Result<(),Error> {
    let string_reader = std::fs::read_to_string(f_path)?;
    let lines = string_reader.split("\n");

    let mut curr_max = 0;
    let mut curr_cum = 0;
    let mut top3 = [0,0,0];
    
    //Would be interesting to benchmark agains read_to_string //TODO: 
    for line in lines {
        let line_string = line;
        if line_string != "" {
            curr_cum += line_string.parse::<i32>().unwrap();
        } else {
            if curr_max < curr_cum {
                curr_max = curr_cum;
            }
            
            if top3[0] < curr_cum {
                top3[0] = curr_cum;
            }
            for i in 0..(top3.len() - 1){
                if top3[i]>top3[i+1] {
                    let temp = top3[i];
                    top3[i] = top3[i+1];
                    top3[i+1] = temp;
                }
            }

            curr_cum = 0;
        }

    };
    
    let sum_top3:i32= top3.iter().sum();
    println!("Max is: {}",curr_max);
    println!("Top 3 are {:#?}", top3);
    println!("Sum of top 3 is {}", sum_top3);
    Ok(())
}
