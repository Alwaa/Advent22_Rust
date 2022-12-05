use std::io::{Error};

use nom::{
    IResult, 
    character::complete::*,
    sequence::{delimited,preceded},
    multi::*,
    bytes::complete::{tag, take_until,},
    branch::alt,
    *};

fn main() -> Result<(),Error> {
    std::env::set_var("RUST_BACKTRACE", "1");
    let path = std::env::current_dir().expect("CURRENT WORKING DIR");
    println!("{}", path.display());
    let file_path = std::path::Path::new("../input22-05.txt"); //Path relative to proj folder...
    
    let input = std::fs::read_to_string(file_path).unwrap();
    let (rest_input, crates_string) = crates_parse(&input).unwrap();


    let (_, crates_vecs) = crates_parse_total(crates_string).unwrap();
    //println!("{:#?}",crates_vecs);
    let crate_lines_num = crates_vecs.len();
    let mut bott_up_crate_vecs = crates_vecs.iter().rev();

    let (rest_string, col_num) = parse_num_and_line(rest_input).unwrap();

    let mut colums: Vec<Vec<&str>> = vec![Vec::new();col_num];
    for _i in 0..crate_lines_num {
        let mut curr_feed = bott_up_crate_vecs.next().unwrap().iter();

        for col in 0..col_num {
            let crt = curr_feed.next().unwrap();
            if *crt != " " {
                colums[col].push(crt)
            }
        }
    }


    let instructions_block = rest_string.split("\n"); //("\rn") when from test....... should
                                                          //rethink using notepad for test

    let crane9001 = true;
    for instruct_str in instructions_block {
        //println!("{:?}",instruct_str);
        //OMG..THE SPLIT WAS LEAVING AN EMPY STRING AT THE END!!!!
        //Took me a fckn half-our pluss to debugD:
        if instruct_str == "" {
            break;
        }
        let (_,instruct ) = instructions_parse(instruct_str).unwrap();
        //println!("{:#?}",instruct);
        let quant = instruct[0].parse::<usize>().unwrap(); 
        let from = instruct[1].parse::<usize>().unwrap() - 1; 
        let to = instruct[2].parse::<usize>().unwrap() - 1; 

        if crane9001 {
            let mut crane_arm: Vec<&str> = Vec::new(); 
            for _i in 0..quant{
                let curr_crt = colums[from].pop().unwrap();
                crane_arm.push(curr_crt)
            }
            for _i in 0..quant{
                let curr_crt = crane_arm.pop().unwrap();
                colums[to].push(curr_crt)
            }
        }else {
            for _i in 0..quant{
                let curr_crt = colums[from].pop().unwrap();
                colums[to].push(curr_crt)
            }
        }
    }


    println!("",);
    for i in 0..col_num{
        let top_crate = colums[i].pop();
        match top_crate {
            Some(val) => print!("{}",val),
            None => print!(" "),
        }
    }

    println!("",);
    Ok(())
}

// PARSING PART
fn crates_parse(input:&str) -> IResult<&str, &str> {
    take_until("\n 1")(input)
}  

fn crate_single_parse(input: &str) -> IResult<&str,&str> {
    alt((
        delimited(tag(" "),tag(" "),tag(" ")),
        delimited(tag("["),take_until("]"),tag("]"))
        ))(input)
}

fn crate_parse_from_line(input: &str) -> IResult<&str,Vec<&str>> {
    separated_list0(tag(" "), crate_single_parse)(input)

}

fn crates_parse_total(input: &str) -> IResult<&str, Vec<Vec<&str>>>{
    separated_list0(alt((tag("\r\n"),tag("\n"))), crate_parse_from_line)(input)
}

fn parse_num_and_line(input: &str) -> IResult<&str, usize>{
    let (out, input) = alt((take_until("\r\n\r\n"),take_until("\n\n")))(input)?;//Should have been able to use newline??
    let (nums,_) = tag("\n")(input)?;
    let (out, _) = alt((tag("\r\n\r\n"),tag("\n\n")))(out)?;
    let vec_split = nums.split(" ").collect::<Vec<&str>>();
    let mut nums_rev = vec_split.iter().rev();
    let _ = nums_rev.next();
    let number = nums_rev.next().unwrap().parse::<usize>().unwrap();
    Ok((out, number))
}

fn words_parse(input:&str) -> IResult<&str, &str> {
    take_until(" ")(input)
}  
fn until_dig(input:&str) -> IResult<&str, &str> {
    delimited(
        preceded(words_parse,tag(" ")),
        digit1,
        alt((tag(" "),tag("\r\n"),tag("\n"),tag(""),nom::combinator::eof))
        )(input)
}  


fn instructions_parse(input: &str) -> IResult<&str, [&str;3]>{
    let mut buf = ["";3];
    let (rest, ()) = fill(until_dig, &mut buf)(input)?;
    Ok((rest, buf))
}

