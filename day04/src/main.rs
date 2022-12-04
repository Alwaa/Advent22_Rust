use std::fs::File;
use std::io::{BufReader, BufRead, Error};


fn main() -> Result<(),Error> {
    let path = std::env::current_dir().expect("CURRENT WORKING DIR");
    println!("{}", path.display());
    let file_path = std::path::Path::new("../input22-04.txt"); //Path relative to proj folder...
    
    let input = File::open(&file_path)?;
    let buffered_reader = BufReader::new(input);

    let mut count = 0;
    let mut overlap_count = 0;
    for line in buffered_reader.lines() {
        let line_string = line?; 
        let mut split_line = line_string.split(",");
        let lhs = split_line.next().unwrap();
        let rhs = split_line.next().unwrap();
        //println!("{} -- {}", lhs, rhs);

        if completly_in_either(&lhs,&rhs) {
            count += 1;
        }

        if overlap(&lhs,&rhs) {
            overlap_count += 1;
        }
    }
    println!("Number of contained pairs: {}", count);
    println!("Number of overlapping pairs: {}", overlap_count);

    Ok(())
}


fn completly_in_either(a: &str, b:&str) -> bool {
    let mut a = a.split("-");
    let mut b = b.split("-");
    let a_head = a.next().unwrap().parse::<i32>().unwrap();
    let a_tail = a.next().unwrap().parse::<i32>().unwrap();
    let b_head = b.next().unwrap().parse::<i32>().unwrap();
    let b_tail = b.next().unwrap().parse::<i32>().unwrap();

    let head = a_head - b_head;
    let tail = a_tail - b_tail;
    //if both start or end at same, one must fully contain the other :0
    //hence the (or eq)
    if (head * tail) <= 0 {
        return true
    }

    return false;
}

//copy code insted of creating function for parsing on its own since i wont be doing this
//further...
fn overlap(a: &str, b:&str) -> bool {
    let mut a = a.split("-");
    let mut b = b.split("-");
    let a_head = a.next().unwrap().parse::<i32>().unwrap();
    let a_tail = a.next().unwrap().parse::<i32>().unwrap();
    let b_head = b.next().unwrap().parse::<i32>().unwrap();
    let b_tail = b.next().unwrap().parse::<i32>().unwrap();

    let t_h_a = a_tail - b_head;
    let t_h_b = b_tail - a_head;
    if (t_h_a * t_h_b) >= 0 {
        return true
    }

    return false;
}
