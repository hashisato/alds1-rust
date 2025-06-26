use std::io::{self, BufRead};
use std::f64::consts::PI;

fn main() {
    let n: usize = read_usize();
    let mut origin: (f64, f64) = (0.0, 0.0);
    let mut last: (f64, f64) = (100.0, 0.0);
    println!("{} {}", origin.0, origin.1);
    kock(n, origin, last);
    println!("{} {}", last.0, last.1);
}

fn read_usize() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn kock(n: usize, p1: (f64, f64), p2: (f64, f64)) {
    if n==0 { return; }
    let rad: f64 = PI / 3.0;
    let s: (f64, f64) = (
        (2.0*p1.0 + 1.0*p2.0) / 3.0,
        (2.0*p1.1 + 1.0*p2.1) / 3.0
    );
    let t: (f64, f64) = (
        (1.0*p1.0 + 2.0*p2.0) / 3.0,
        (1.0*p1.1 + 2.0*p2.1) / 3.0
    );
    let u: (f64, f64) = (
        (t.0 - s.0)*rad.cos() - (t.1 - s.1)*rad.sin() + s.0,
        (t.0 - s.0)*rad.sin() + (t.1 - s.1)*rad.cos() + s.1
    );
    kock(n-1, p1, s);
    println!("{} {}", s.0, s.1);
    kock(n-1, s, u);
    println!("{} {}", u.0, u.1);
    kock(n-1, u, t);
    println!("{} {}", t.0, t.1);
    kock(n-1 , t, p2);
}