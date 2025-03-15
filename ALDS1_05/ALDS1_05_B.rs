use std::io::{self, BufRead};

fn main() {
    let n = read_usize();
    let mut S = read_u64_vec(n);
    let mut count: usize = 0;
    mergeSort(&mut S, 0, n, &mut count);
    for i in 0..S.len() {
        if i!=S.len()-1 { print!("{} ",S[i]); }
        else if i==S.len()-1 { println!("{}",S[i]); }
    }
    println!("{}",count);
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
        .map(|s| s.parse().unwrap())
        .collect()
}

fn merge(A: &mut Vec<u64>, left: usize, mid: usize, right: usize, count: &mut usize) {
    let n1 = mid - left;
    let n2 = right - mid;
    let mut L: Vec<u64> = Vec::new();
    let mut R: Vec<u64> = Vec::new();
    for i in 0..n1 { L.push(A[left+i]); }
    for i in 0..n2 { R.push(A[mid+i]); }
    let inf = std::f64::INFINITY as u64;
    L.push(inf);
    R.push(inf);
    let (mut i, mut j) = (0, 0);
    for k in left..right {
        if L[i]<=R[j] {
            A[k] = L[i];
            i += 1;
            *count += 1;
        }else {
            A[k] = R[j];
            j += 1;
            *count += 1;
        }
    }
}

fn mergeSort(A: &mut Vec<u64>, left: usize, right: usize, count: &mut usize) {
    if left+1<right {
        let mid = (left + right) / 2;
        mergeSort(A, left, mid, count);
        mergeSort(A, mid, right, count);
        merge(A, left, mid, right, count);
    }
}