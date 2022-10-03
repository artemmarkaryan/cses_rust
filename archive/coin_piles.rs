// https://cses.fi/problemset/task/1754

use std::cmp::{max, min};
use std::io;
use std::str::FromStr;

fn main() {
    let t: u32 = read().expect("err");
    for _ in 0..t {
        let l = read_vec::<u64>().expect("err");
        let (mut a, mut b) = if l[0] > l[1] { (l[0], l[1]) } else { (l[1], l[0]) }; // a > b

        if 2 * b < a {
            println!("NO");
            continue;
        }
        a %= 3;
        b %= 3;

        if a == 2 && b == 1 || a == 1 && b == 2 || a == 0 && b == 0 {
            println!("YES")
        } else {
            println!("NO");
        }
    }
}


fn read<T: FromStr>() -> Result<T, T::Err> {
    read_line().trim().parse::<T>()
}

fn read_vec<T: FromStr>() -> Result<Vec<T>, T::Err> {
    read_line().split_whitespace().map(|x| x.parse::<T>()).collect()
}

fn read_line() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read line");

    buffer
}

