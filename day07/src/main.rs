use std::io::{Error};
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::{Dfs, Bfs};

fn main() -> Result<(),Error> {
    std::env::set_var("RUST_BACKTRACE", "1");
    let path = std::env::current_dir().expect("CURRENT WORKING DIR");
    println!("{}", path.display());
    let file_path = std::path::Path::new("../input22-07.txt"); //Path relative to proj folder...
    
    let input = std::fs::read_to_string(file_path).unwrap();

    let file_sus = run_through(&input);
    let file_system = update_dir_sizes(file_sus);
    let root = file_system.node_indices().next().unwrap(); 

    let mut file_counter = 0;
    let mut dfs = Dfs::new(&file_system,root);
    while let Some(node) = dfs.next(&file_system) {
        if file_system[node].1 <= 100000 && file_system[node].2{
            file_counter += file_system[node].1;
        }
    }

    //println!("{:#?}", file_system);
    println!("Total size of small files double count: {:?}",file_counter);
    
    let space_needed = 30000000 - (70000000 - file_system[root].1);
    println!("{}, {}", space_needed, file_system[root].1);
    let mut large_dirs: Vec<i32> = Vec::new();
    let mut bfs = Bfs::new(&file_system,root);
    while let Some(node) = bfs.next(&file_system) {
        if file_system[node].1 >= space_needed {
            large_dirs.push(file_system[node].1)
        }
    }
    println!("{:#?}",large_dirs);
    println!("Smallest file to cover delete req: {}",large_dirs.iter().min().unwrap());
    Ok(())
}

//Wrote non-recursive depth-first search insted of using the buildtin function
fn update_dir_sizes(mut file_graph: Graph<(&str, i32, bool),()>) -> Graph<(&str, i32, bool),()> {
    let mut depth_stack: Vec<NodeIndex> = Vec::new();
    let mut neigh_stack: Vec<Vec<NodeIndex>> = Vec::new();
    let root = file_graph.node_indices().next().unwrap(); 
    depth_stack.push(root);

    while depth_stack.len() > 0 {
        let curr = depth_stack[depth_stack.len()-1];
        if depth_stack.len() > neigh_stack.len() {
            let mut neighbors: Vec<NodeIndex> = file_graph.neighbors(curr).collect();
            neigh_stack.push(neighbors);
        }else {
            let leng = neigh_stack.len();
            let mut neigh_last = &mut neigh_stack[leng -1];
            let leng_ns = neigh_last.len();
            if leng_ns == 0{
                if neigh_stack.len() <= 1 {
                    break;
                }
                let parent = depth_stack[depth_stack.len() - 2];
                file_graph[parent].1 += file_graph[curr].1;
                //println!("{:#?}", curr);
                neigh_stack.pop();
                depth_stack.pop();
                neigh_stack[leng-2].pop();
            }else {
                let next_down = neigh_last[leng_ns - 1];
                depth_stack.push(next_down);
            }
        }
        //println!("{:#?}",neigh_stack);
        //println!("{:#?}",depth_stack);
    }

    return file_graph
}

fn run_through(text: &str) -> Graph<(&str, i32, bool),()> {
    let lines = text.lines();
    let mut file_tree = Graph::new();

    let root = file_tree.add_node(("/", 0, true));
    let mut curr_dir: Vec<NodeIndex> = vec![root];
    for line in lines{
        let parent = curr_dir[curr_dir.len() - 1];
        let mut words = line.split(" ");
        let w1 = words.next().unwrap();
        let w2 = words.next().unwrap();
        //println!("{:#?}",file_tree);
        
        match w1 {
            "$" => {
                if w2 == "ls" {
                }else {
                    let next_path = words.next().unwrap();
                    if next_path == ".." {
                        _ = curr_dir.pop();
                    } else {
                        for sub_node in file_tree.neighbors(parent) {
                            if file_tree[sub_node].0 == next_path {
                                curr_dir.push(sub_node);
                            }                        
                        }
                    }
                }
            },
            _ => {
                let size = if w1 != "dir" {
                    w1.parse::<i32>().unwrap()
                }else {
                    0
                };
                let direct = file_tree.add_node((w2,size,w1=="dir"));
                file_tree.extend_with_edges(&[
                                    (parent, direct, ())
                ]);
            }
        }

    }
    return file_tree 
}
