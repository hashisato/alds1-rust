use std::io::{self, BufRead};
use std::io::prelude::*;
use std::collections::HashMap;

const MAX_INDEX: usize = 1e4 as usize + 1;

fn main() {
    let n: usize = read_usize();
    let mut A: Vec<u64> = read_u64_vec(n);
    let mut count: u64 = 0;
    /*
    let key = partition(n-1, &mut A, &mut count);
    println!("{}",count);
    for i in 0..n {
        if i==0 { print!("{}", A[i]); }
        else if i==key { print!(" [{}]", A[i]); }
        else if i==n-1 { println!(" {}", A[i]); }
        else { print!(" {}", A[i]); }
    }
    print(&A);
    */
    let weight = weigh(&A, n);
    println!("{}", weight);
}

fn read_usize() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_u64_vec(n: usize) -> Vec<u64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .take(n)
        .map(|s| s.parse().unwrap() )
        .collect()
}

fn print(vec: &Vec<u64>) {
    for i in vec {
        println!("{} ", i);
    }
}

fn weigh(W: &Vec<u64>, n: usize) -> u64 {
    // indexesは-1で初期化
    let mut indexes: HashMap<u64, isize> = HashMap::new();
    let mut A = W.clone();
    A.sort();
    let lowestwv = A[0];

    // indexes[weight] = index の形式
    for (i, &w) in W.iter().enumerate() {
        indexes.insert(w, i as isize);
    }

    let mut weight = 0;
    // W[0]からW[n-1]まで順に見る
    for i in 0..n {
        // グループをたどった時に使ったものならコスト加算済みなのでとばす
        if *indexes.get(&W[i]).unwrap() < 0 {
            continue;
        }
        // グループの終わりのチェックのため最初の値を保持
        let fwv = W[i];
        let mut av = A[i];
        if fwv == av {
            continue;
        }
        let mut minwv = fwv;
        let mut next_idx = i;
        // グループ内最小値を繰り返し動かした場合の合計コスト
        let mut sum1 = 0;
        // グループ内の要素数のカウント
        let mut count = 1;
        while {
            //println!("{}", W[next_idx]);
            sum1 += W[next_idx];
            if minwv > W[next_idx] {
                minwv = W[next_idx];
            }
            // 一巡してグループの最初に戻るならwhileを抜ける
            if fwv == av {
                false
            } else {
                count += 1;
                // たどる
                next_idx = *indexes.get(&av).unwrap() as usize;
                indexes.insert(av, -1); // 使用済み（グループ済み）マーク
                av = A[next_idx];
                true
            }
        } {}

        // 全体の最小値を繰り返し使うときの合計コスト（sum1はまだグループ総和のまま）
        let sum2 = sum1 + minwv + (count + 1) * lowestwv;
        //println!(" minwv: {} count: {}", minwv, count);
        //println!("{} {} {} {}", sum1, minwv, count, lowestwv);
        // 最終的なsum1を算出
        sum1 += (count - 2) * minwv;
        //println!("sum1: {} sum2: {}", sum1, sum2);
        weight += sum1.min(sum2);
    }
    weight
}