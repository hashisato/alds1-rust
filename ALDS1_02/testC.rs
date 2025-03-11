use std::io;

fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).ok();
    let n: usize = num.trim().parse().unwrap();

    let mut tmps = String::new();
    io::stdin().read_line(&mut tmps).ok();
    let tmps: Vec<Vec<char>> = tmps.split_whitespace().collect::<Vec<&str>>().iter_mut()
                                .map(|c| c.chars().collect()).collect();

    let mut vec: Vec<(char, usize)> = Vec::new();
    for i in 0..n {
        vec.push((tmps[i][0], (tmps[i][1] as usize - 48)))
    }

    let bubble = bubble_sort(n, vec.clone());
    let selection = selection_sort(n, vec.clone());

    vec.sort_by(|a, b| a.1.cmp(&b.1));

    (0..n).for_each(|i| if i == n-1 { println!("{}{}", bubble[i].0, bubble[i].1) } else { print!("{}{} ", bubble[i].0, bubble[i].1)});

    if bubble == vec {
        println!("Stable")
    } else {
        println!("Not stable")
    }

    (0..n).for_each(|i| if i == n-1 { println!("{}{}", selection[i].0, selection[i].1) } else { print!("{}{} ", selection[i].0, selection[i].1)});

    if selection == vec {
        println!("Stable")
    } else {
        println!("Not stable")
    }
}

fn bubble_sort(n: usize, vec: Vec<(char, usize)>) -> Vec<(char, usize)> {
    let mut v = vec.clone();
    for i in 0..n {
        for j in (i+1..n).rev() {
            if v[j].1 < v[j-1].1 {
                let tmp = v[j].clone();
                v[j] = v[j-1];
                v[j-1] = tmp;
            }
        }
    }

    v
}

fn selection_sort(n: usize, vec: Vec<(char, usize)>) -> Vec<(char, usize)> {
    let mut v = vec.clone();
    for i in 0..n {
        let mut min = i;
        for j in i..n {
            if v[j].1 < v[min].1 {
                min = j;
            }
        }
        let tmp = v[i];
        v[i] = v[min];
        v[min] = tmp;
    }

    v
}


