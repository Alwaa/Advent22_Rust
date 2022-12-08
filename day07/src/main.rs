use std::io::{Error};
use nom::{
    IResult,
    character::complete::*,
    *
};
use petgraph::graph::Graph;


fn main() -> Result<(),Error> {
    let path = std::env::current_dir().expect("CURRENT WORKING DIR");
    println!("{}", path.display());
    let file_path = std::path::Path::new("../input22-test.txt"); //Path relative to proj folder...
    
    let input = std::fs::read_to_string(file_path).unwrap();

    let file_sus = run_through(&input);

    let mut file_tree = make_vecdir(&input);

    let root = file_tree.node_indices().next().unwrap();
    let node = &mut file_tree[root];
    let name = node.0;
    *node = (name,3);

    Ok(())
}



fn make_vecdir(text: &str) -> Graph<(&str, i32),()> {    
    let mut file_tree = Graph::new();
    let root = file_tree.add_node(("/",0));
    let test_leaf = file_tree.add_node(("a",30));
    file_tree.extend_with_edges(&[
        (root, test_leaf, ())
    ]);
    return file_tree
}


fn run_through(text: &str) -> Graph<(&str, i32),()> {
    let lines = text.lines();
    let mut file_tree = Graph::new();

    let mut curr_dir: Vec<&str> = vec![&"/"];
    let mut parent:&str = curr_dir[0];
    for line in lines{
        let mut words = line.split(" ");
        let w1 = words.next().unwrap();
        let w2 = words.next().unwrap();
        println!("{}",w1);
        
        match w1 {
            "$" => {
                if w2 == "ls" {
                }else {
                    let next_path = words.next().unwrap();
                    if next_path == ".." {
                        unimplemented!();
                    }
                    curr_dir.push(next_path);
                }
            },
            _ => {
                let size = if w1 != "dir" {
                    w1.parse::<i32>().unwrap()
                }else {
                    0
                };
                let _direct = file_tree.add_node((w2,size));
            }
        }

    }

    return file_tree 
}
