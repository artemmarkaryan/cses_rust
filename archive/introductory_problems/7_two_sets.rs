// https://cses.fi/problemset/task/1092

use std::str::FromStr;

fn main() {
    let mut n: i32 = read().expect("err");

    if n == 1 || n == 2 {
        println!("NO");
        return;
    }

    if (n + i32::pow(n, 2)) % 4 != 0 {
        println!("NO");
        return;
    }

    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    if n % 2 == 1 {
        v1.push(n);
        n -= 1;
    }

    for i in 1..(n / 2) + 1 {
        if i % 2 == 0 {
            v1.push(i);
            v1.push(n + 1 - i);
        } else {
            v2.push(i);
            v2.push(n + 1 - i);
        }
    }

    println!("YES");
    println!("{}", v1.len());
    for i in v1 {
        print!("{} ", i)
    }
    println!("\n{}", v2.len());
    for i in v2 {
        print!("{} ", i)
    }
}

fn read_line() -> String {
    let mut buffer = String::new();
    std::io::stdin()
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