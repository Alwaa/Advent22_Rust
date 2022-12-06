use std::io::{Error};
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() -> Result<(),Error> {
    let path = std::env::current_dir().expect("CURRENT WORKING DIR");
    println!("{}", path.display());
    let file_path = std::path::Path::new("../input22-06.txt"); //Path relative to proj folder...
    
    let input = std::fs::read_to_string(file_path).unwrap();

    //PARAMETER
    let seq_len = 14;

    let mut chr_it = input.chars();
    let mut r_buff: VecDeque<char> = VecDeque::with_capacity(seq_len);
    for _i in 0..seq_len {
        r_buff.push_back(
            chr_it.next().unwrap() //must at least have 4 chars
            )
    }
    let mut counter = seq_len;

    for chr in chr_it{
        let sett: HashSet<&char> = r_buff.iter().collect();
        if sett.len() > (seq_len - 1) {
            //UvU
            break;
        }

        counter += 1;
        let _ = r_buff.pop_front();
        r_buff.push_back(
            chr
            );
    }
    println!("Count: {}\n\n{:?}",counter,r_buff);

    Ok(())
}
