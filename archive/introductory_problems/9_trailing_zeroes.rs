// https://cses.fi/problemset/task/1618

use std::cmp::min;
use std::str::FromStr;

fn main() {
    let n: u32 = read().expect("err");

    let mut _5 = 0;

    for i in (5..(&n + 1)).step_by(5) {
        let mut ii = i;
        while ii % 5 == 0 {
            _5 += 1;
            ii /= 5;
        }
    }

    println!("{}", _5)
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

