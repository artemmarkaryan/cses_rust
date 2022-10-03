// https://cses.fi/problemset/task/1072

use std::{u128, u64};
use std::io;
use std::str::FromStr;

fn main() {
    let n: u64 = read().expect("err");
    println!("{}", 0);
    if n == 1 { return; };
    println!("{}", 6);
    if n == 2 { return; };
    for k in 3..n + 1 {
        let t = (pow(k, 4) - pow(k, 2)) / 2;
        let b: u64 = (k - 2) * (k - 1) * 4;
        println!("{}", t - b)
    }
}

fn pow(a: u64, b: u32) -> u64 { u64::pow(a, b) }

fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read line");

    buffer
}

fn read<T: FromStr>() -> Result<T, T::Err> {
    read_line().trim().parse::<T>()
}

fn read_vec<T: FromStr>() -> Result<Vec<T>, T::Err> {
    read_line().split_whitespace().map(|x| x.parse::<T>()).collect()
}