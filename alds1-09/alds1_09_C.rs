use std::io::{self, BufRead};

struct Heap {
    data: Vec<i32>,
}

impl Heap {
    fn new() -> Self {
        Heap { data: vec![-1] }
    }

    fn insert(&mut self, key: i32) {
        self.data.push(key);
        let mut i = self.data.len() - 1;
        while i > 1 && self.data[i / 2] < self.data[i] {
            self.data.swap(i, i / 2);
            i /= 2;
        }
    }

    fn extract_max(&mut self) -> Option<i32> {
        if self.data.len() <= 1 {
            return None;
        }
        let max = self.data[1];
        let last = self.data.pop().unwrap();
        if self.data.len() > 1 {
            self.data[1] = last;
            self.max_heapify(1);
        }
        Some(max)
    }

    fn max_heapify(&mut self, mut i: usize) {
        loop {
            let l = i * 2;
            let r = i * 2 + 1;
            let mut largest = i;
            if l < self.data.len() && self.data[l] > self.data[largest] {
                largest = l;
            }
            if r < self.data.len() && self.data[r] > self.data[largest] {
                largest = r;
            }
            if largest == i {
                break;
            }
            self.data.swap(i, largest);
            i = largest;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut heap = Heap::new();

    for line in stdin.lock().lines().flatten() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        match parts[0] {
            "insert" => {
                if let Some(x) = parts.get(1).and_then(|s| s.parse().ok()) {
                    heap.insert(x);
                }
            }
            "extract" => {
                if let Some(max) = heap.extract_max() {
                    println!("{}", max);
                }
            }
            "end" => break,
            _ => {}
        }
    }
}
