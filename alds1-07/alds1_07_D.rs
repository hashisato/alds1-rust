use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let n: usize = read_usize();
    let preorder: Vec<usize> = read_vec(n);
    let inorder: Vec<usize> = read_vec(n);

    let mut postorder: Vec<usize> = Vec::new();
    reconstruct_postorder(&preorder, &inorder, &mut postorder);

    // Postorder output
    for &value in &postorder {
        print!("{}", value);
        if value != *postorder.last().unwrap() {
            print!(" ");
        }
    }
    println!();
}

fn read_usize() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_vec(n: usize) -> Vec<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim()
         .split_whitespace()
         .map(|s| usize::from_str(s).unwrap())
         .collect::<Vec<usize>>()
}

fn reconstruct_postorder(preorder: &[usize], inorder: &[usize], postorder: &mut Vec<usize>) {
    if preorder.is_empty() || inorder.is_empty() {
        return;
    }
    let root = preorder[0];
    let root_index_inorder = inorder.iter().position(|&x| x == root).unwrap();
    let left_inorder = &inorder[..root_index_inorder];
    let right_inorder = &inorder[root_index_inorder + 1..];
    let left_preorder = &preorder[1..1 + left_inorder.len()];
    let right_preorder = &preorder[1 + left_inorder.len()..];
    reconstruct_postorder(left_preorder, left_inorder, postorder);
    reconstruct_postorder(right_preorder, right_inorder, postorder);
    postorder.push(root);
}