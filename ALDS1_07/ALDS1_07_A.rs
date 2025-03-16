use std::io::{self, BufRead};
use std::str::FromStr;

struct Node {
    id: usize,
    parent: isize,
    depth: usize,
    kind: String,
    children: Vec<usize>,
}

fn main() {
    let n: usize = read_usize();
    let mut nodes: Vec<Node> = Vec::new();
    for i in 0..n {
        let node: Node = read_node();
        nodes.push(node);
    }
    for i in 0..n {
        for j in 0..nodes[i].children.len() {
            let child = nodes[i].children[j];
            nodes[child].parent = nodes[i].id as isize;
        }
    }
    let mut check: isize = -1;
    for i in 0..n {
        nodes[i].depth = 0;
        check = nodes[i].parent;
        while check != -1 {
            nodes[i].depth += 1;
            check = nodes[check as usize].parent;
        }
    }

    for i in 0..n {
        let node = &nodes[i];
        let kind = if node.parent==-1 { "root" }
        else if node.children.len()==0 { "leaf" }
        else { "internal node" };
        print!("node {}: parent = {}, depth = {}, {}, [", node.id, node.parent, node.depth, kind);
        if node.children.len()!=0 {
            for i in 0..node.children.len() {
                print!("{}", node.children[i]);
                if i<node.children.len()-1 { print!(", "); }
            }
        }
        println!("]");
    }    
}

fn read_usize() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_node() -> Node {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut line = lines.next().unwrap().unwrap();
    let words = line.split_whitespace().collect::<Vec<&str>>();
    let id: usize = usize::from_str(words[0]).unwrap();
    let mut children: Vec<usize> = Vec::new();
    for i in 2..words.len() {
        let child: usize = usize::from_str(words[i]).unwrap();
        children.push(child);
    }
    let parent: isize = -1;
    let depth: usize = 0;
    let kind: String = String::new();
    Node{id, parent, depth, kind, children}
}