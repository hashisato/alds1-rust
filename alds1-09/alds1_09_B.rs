use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let keys_line = lines.next().unwrap().unwrap();
    let mut heap: Vec<i32> = vec![0];
    heap.extend(keys_line.split_whitespace().map(|s| s.parse::<i32>().unwrap()));

    build_max_heap(&mut heap, n);

    for i in 1..=n {
        print!(" {}", heap[i]);
    }
    println!();
}

fn build_max_heap(heap: &mut Vec<i32>, h: usize) {
    for i in (1..=h / 2).rev() {
        max_heapify(heap, i, h);
    }
}

fn max_heapify(heap: &mut Vec<i32>, i: usize, h: usize) {
    let l = i * 2;
    let r = i * 2 + 1;
    let mut largest = i;
    if l <= h && heap[l] > heap[i] {
        largest = l;
    }
    if r <= h && heap[r] > heap[largest] {
        largest = r;
    }
    if largest != i {
        heap.swap(i, largest);
        max_heapify(heap, largest, h);
    }
}
