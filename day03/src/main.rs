use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

use std::collections::HashSet;

fn main() -> Result<(),Error> {
    let path = std::env::current_dir().expect("CURRENT WORKING DIR");
    println!("{}", path.display());
    let file_path = std::path::Path::new("../input22-03.txt"); //Path relative to proj folder...
    
    let input = File::open(&file_path)?;
    let buffered_reader = BufReader::new(input);
    let mut score_cum = 0;

    let mut counter: i32 =0;
    let mut group: Vec<String> = vec!["".to_string() ; 3];
    let mut group_score_cum = 0;
    for line in buffered_reader.lines() {
        let line_string = line?; 
        let lenght_sack = line_string.len();

        let common_item = common_char(&line_string);
        score_cum += score(&common_item);
        //println!("{} -- {}", lenght_sack, common_item);
        let three_count = (counter%3) as usize;
        group[three_count] = line_string;

        //println!("{:#?}", &group);
        if three_count as u8 == 2 {
            let common_letter = common_group(&group);
            //println!("{}", common_letter);
            group_score_cum += score(&common_letter);
        } 
        
        counter += 1;
    }
    println!("Sum of priorities: {}", score_cum);
    println!("Sum badge priorities: {}", group_score_cum);

    Ok(())
}



fn common_char(inn_string: &str) -> char { 
    let lenght_sack = inn_string.len();
    let mut char_iterator = inn_string.chars();
    
    let partition1: HashSet<char> = char_iterator
            .by_ref().take(lenght_sack/2)
            .collect();
    let partition2: HashSet<char> = char_iterator.collect();
    
    //println!("{:?}  {:?}",partition1,partition2);
    let mut intersect = partition1.intersection(&partition2);

    return intersect.next().unwrap().clone();
}

fn common_group(group_vec: &Vec<String>) -> char {

    let partition1: HashSet<char> = group_vec[0].chars().collect();
    let partition2: HashSet<char> = group_vec[1].chars().collect();
    let partition3: HashSet<char> = group_vec[2].chars().collect();

    let intersect_inter: HashSet<char> = intersection_own_a(partition1,&partition2);
    let mut intersect = partition3.intersection(&intersect_inter);

    return intersect.next().unwrap().clone();
}

use std::hash::Hash;
fn intersection_own_a<T: Eq + Hash>(a: HashSet<T>, b: &HashSet<T>) -> HashSet<T> {
    a.into_iter().filter(|e| b.contains(e)).collect()
}

fn score(letter: &char) -> i32 {
    let alph_string = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    
    for (i, l) in alph_string.chars().enumerate() {
        if l == *letter {
            return (i + 1).try_into().unwrap() 
        }
    }
    return 0;
}







