use std::io::*;
use std::str::FromStr;

// 標準入力から値を1つ読む
fn rin<T: FromStr>() -> T {
    let s = stdin();
    let s = s.lock();
    let s: String = s.bytes()
        .map(|c| c.expect("failed reading char.") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    s.parse().ok().expect("failed parsing.")
}

fn main() {
    let n: usize = rin();
    let mut a: Vec<i32> = vec![0; n];
    for i in 0..n {
        a[i] = rin();
    }

    // 昇順ソート
    a.sort();

    // ヒープ順に並べ替え
    for i in 0..n-1 {
        let mut j = i;
        while j > 0 {
            let k = (j-1)/2;
            a.swap(j, k);
            j = k;
        }
        a.swap(0, i+1);
    }

    // 出力
    let result: Vec<String> = a.iter().map(|v| v.to_string()).collect();
    println!("{}", result.join(" "));
}
