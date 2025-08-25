use std::io::{self, BufRead};

const NUM: usize = 3;
const DX: [i32; 4] = [0, 1, 0, -1];
const DY: [i32; 4] = [1, 0, -1, 0];

fn main() {
    let stdin = io::stdin();
    let mut data = [[0; NUM]; NUM];
    let mut x = 0;
    let mut y = 0;
    for i in 0..NUM {
        let line = stdin.lock().lines().next().unwrap().unwrap();
        for (j, s) in line.split_whitespace().enumerate() {
            data[i][j] = s.parse().unwrap();
            if data[i][j] == 0 {
                x = i;
                y = j;
            }
        }
    }
    let mut l = 0;
    loop {
        let result = dfs(&mut data, x, y, 0, l);
        if result != -1 {
            println!("{}", result);
            break;
        }
        l += 1;
    }
}

fn cal(data: &[[i32; NUM]; NUM]) -> i32 {
    let mut sum = 0;
    for i in 0..NUM {
        for j in 0..NUM {
            let n = data[i][j] - 1;
            if n != -1 {
                let x = (n % NUM as i32) - j as i32;
                let y = (n / NUM as i32) - i as i32;
                sum += x.abs() + y.abs();
            }
        }
    }
    sum
}

fn dfs(data: &mut [[i32; NUM]; NUM], x: usize, y: usize, d: i32, l: i32) -> i32 {
    let h = cal(data);
    if h == 0 {
        return d;
    }
    if d + h > l {
        return -1;
    }
    for dir in 0..4 {
        let nx = x as i32 + DX[dir];
        let ny = y as i32 + DY[dir];
        if nx < 0 || nx >= NUM as i32 || ny < 0 || ny >= NUM as i32 {
            continue;
        }
        let nx = nx as usize;
        let ny = ny as usize;
        data[x][y] = data[nx][ny];
        data[nx][ny] = 0;
        let res = dfs(data, nx, ny, d+1, l);
        data[nx][ny] = data[x][y];
        data[x][y] = 0;
        if res != -1 {
            return res;
        }
    }
    -1
}
