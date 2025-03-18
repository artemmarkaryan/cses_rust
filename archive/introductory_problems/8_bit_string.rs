// https://cses.fi/problemset/task/1617

use std::str::FromStr;

fn main() {
    const MOD: u128 = 1000000007;

    let n: u32 = read().expect("err");
    let mut a: u128 = 1;
    for _ in 1..n+1 {
        a *= 2;
        a %= MOD;
    }

    println!("{}", a)
}

fn read<T: FromStr>() -> Result<T, T::Err> {
    read_line().trim().parse::<T>()
}

fn read_line() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read line");

    buffer
}

