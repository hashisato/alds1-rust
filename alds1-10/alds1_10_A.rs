fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num: usize = input.trim().parse().unwrap();
    let mut seq = vec![0; num + 1];

    fibonacci(num, &mut seq);
    println!("{}", seq[num]);
}

fn fibonacci(n: usize, seq: &mut Vec<i32>) -> i32 {
    if n == 0 || n == 1 {
        seq[n] = 1;
    }
    if seq[n] != 0 {
        return seq[n];
    }
    seq[n] = fibonacci(n - 2, seq) + fibonacci(n - 1, seq);
    seq[n]
}
