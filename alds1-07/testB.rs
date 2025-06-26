use std::io::{self, BufRead};
use std::str::FromStr;

struct Node {
    id: usize,
    parent: isize,
    siblings: Vec<isize>,
    degree: isize,
    depth: usize,
    height: isize,
    kind: String,
}

fn main() {
    let n: usize = read_usize();
    let mut nodes: Vec<Node> = Vec::new();
    for i in 0..n {
        let node: Node = read_node();
        nodes.push(node);
    }
    //nodes.sort_by(|a, b| a.id.cmp(&b.id));
    for i in 0..nodes.len() {
        let mut count: isize = 0;
        let element1: usize = nodes[i].siblings[0] as usize;
        let element2: usize = nodes[i].siblings[1] as usize;
        if nodes[i].siblings[0]!=-1 {
            count += 1;
            nodes[element1].parent = nodes[i].id as isize;
            nodes[element1].siblings[2] = element2 as isize;
        }
        if nodes[i].siblings[1]!=-1 {
            count += 1;
            
            nodes[element2].parent = nodes[i].id as isize;
            nodes[element2].siblings[2] = element1 as isize;    
        }
        nodes[i].degree = count;
        nodes[i].siblings.remove(0);
        nodes[i].siblings.remove(0);
    }
    let (mut check, mut total, mut tmp): (isize, usize, isize) = (0, 0, 0);
    for i in 0..n {
        nodes[i].depth = 0;
        check = nodes[i].parent;
        while check!=-1 {
            tmp += 1;
            nodes[i].depth += 1;
            check = nodes[check as usize].parent;
        }
        if (total as isize)<tmp { total = tmp as usize; }
        tmp = 0;
    }
    for i in (0..=total).rev() {
        for j in 0..n {
            if nodes[j].height==-1 && nodes[j].depth==i {
                tmp = j as isize;
                let mut c = 0;
                if nodes[tmp as usize].height==-1 {
                    while nodes[tmp as usize].height==-1 {
                        nodes[tmp as usize].height = c;
                        tmp = nodes[tmp as usize].parent;
                        c += 1;
                    }
                }
            }
        }
    }
    for i in &nodes {
        println!("id:{} p:{} s:{} deg:{} dep:{} hei:{}", i.id, i.parent
                                    , i.siblings[0]
                                    , i.degree, i.depth
                                    , i.height);
    }
    /*
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
        while check!=-1 {
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
    */
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
    let mut siblings: Vec<isize> = Vec::new();
    for i in 1..words.len() {
        let sibling: isize = isize::from_str(words[i]).unwrap();
        siblings.push(sibling);
    }
    siblings.push(-1);
    let parent: isize = -1;
    let degree: isize = 0;
    let depth: usize = 0;
    let height: isize = -1;
    let kind: String = String::new();
    Node{id, parent, siblings, degree, depth, height, kind}
}