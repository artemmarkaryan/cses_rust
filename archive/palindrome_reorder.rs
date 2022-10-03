// https://cses.fi/problemset/task/1755

use std::collections::{HashMap, VecDeque};
use std::io::repeat;
use std::str::FromStr;
use std::sync::mpsc::RecvTimeoutError;

fn main() {
    let w: String = read_line();
    let word = w.trim();

    let mut lc: HashMap<char, i32> = HashMap::new();

    for l in word.chars() {
        lc.insert(l, if lc.contains_key(&l) { lc[&l] + 1 } else { 1 });
    }

    if lc.keys().len() == 1 {
        println!("{}", word);
        return;
    }

    let mut odd_l: (&char, &i32) = (&' ', &0);
    let mut odd_c: i32 = 0;

    for (k, v) in &lc {
        if v % 2 == 1 {
            odd_l = (k, v);
            odd_c += 1;
        }
    }

    if odd_c >= 2 || word.len() % 2 == 1 && odd_c != 1 {
        println!("NO SOLUTION");
        return;
    }

    let mut r: VecDeque<&char> = VecDeque::new();
    for (k, v) in &lc {
        if k == odd_l.0 { continue; }
        for i in 0..v / 2 {
            r.push_back(k);
            r.push_front(k);
        }
    }

    for i in 0..r.len() {
        print!("{}", r[i]);
        if i + 1== r.len() / 2 {
            for j in 0..*(odd_l.1) {
                print!("{}", odd_l.0);
            }
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

