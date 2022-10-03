// https://cses.fi/problemset/task/1755

use std::str::FromStr;

fn main() {}

fn rr(mut v: Vec<String>) -> Vec<String> {
    let mut r: Vec<String> = Vec::new();
    for i in 0..v.len() {
        r.push(format!("0{}", &v[i]))
    }
    for i in (0..v.len()).rev() {
        r.push(format!("1{}", &v[i]))
    }

    return r;
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

