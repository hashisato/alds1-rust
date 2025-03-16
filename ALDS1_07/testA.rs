#![allow(unused_imports)]
use std::{collections::VecDeque, io};

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("入力エラー");
    let n: usize = n.trim().parse().expect("変換エラー");
    let mut tree = vec![(-1, Vec::<usize>::new(), 0); n];
    for _ in 0..n {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("入力エラー");
        let v: Vec<String> = buf.split_whitespace().map(|j| (j.to_string())).collect();
        let i = v[0].parse::<usize>().unwrap();
        let m = v[1].parse::<usize>().unwrap();
        for j in 2..m + 2 {
            let child = v[j].parse::<usize>().unwrap();
            tree[i].1.push(child);
            tree[child].0 = i as isize;
        }
    }
    let mut que = VecDeque::new();
    que.push_back(tree.iter().position(|(p, _, _)| *p == -1).unwrap());
    while let Some(current) = que.pop_front() {
        let parent = tree[current].0;
        if parent >= 0 {
            tree[current].2 = tree[parent as usize].2 + 1;
        }
        for &child in tree[current].1.iter() {
            que.push_back(child);
        }
    }
    let ans = tree
        .iter()
        .enumerate()
        .map(|(idx, (parent, children, depth))| {
            let node_type = if *parent == -1 {
                "root"
            } else if children.is_empty() {
                "leaf"
            } else {
                "internal node"
            };
            let s = children.iter().map(|x|x.to_string()).collect::<Vec<String>>().join(", ");
            format!("node {}: parent = {}, depth = {}, {}, [{}]", idx, parent,depth,node_type,s)
        }).collect::<Vec<String>>().join("\n");
    println!("{}",ans);
}

