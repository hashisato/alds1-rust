use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut a: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();

    // 降順ソート
    a.sort_by(|x, y| y.cmp(x));

    // ヒープのインデックス順に「幅優先」で値を割り当てる
    let mut heap = vec![0; n + 1];
    let mut idx = 0;
    let mut q = vec![1];
    while let Some(i) = q.pop() {
        if i > n { continue; }
        heap[i] = a[idx];
        idx += 1;
        q.insert(0, i * 2);     // left child
        q.insert(0, i * 2 + 1); // right child
    }

    // 出力
    for i in 1..=n {
        if i > 1 { print!(" "); }
        print!("{}", heap[i]);
    }
    println!();
}