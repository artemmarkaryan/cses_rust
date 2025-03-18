// https://cses.fi/problemset/task/2165

use std::collections::hash_map::Drain;
use std::str::FromStr;

fn main() {
    let n: i32 = read().expect("err");

    let mut a: Vec<i32> = (1..n + 1).rev().collect();
    let mut b: Vec<i32> = Vec::new();
    let mut c: Vec<i32> = Vec::new();

    println!("{}", i32::pow(2, n as u32) - 1);

    if &n % 2 == 0 {
        loop {
            pd(m(&mut a, &mut b), 1, 2);
            if (&c).len() == n as usize { break; };
            pd(m(&mut a, &mut c), 1, 3);
            if (&c).len() == n as usize { break; };
            pd(m(&mut b, &mut c), 2, 3);
            if (&c).len() == n as usize { break; };
        }
    } else {
        loop {
            pd(m(&mut a, &mut c), 1, 3);
            if (&c).len() == n as usize { break; };
            pd(m(&mut a, &mut b), 1, 2);
            if (&c).len() == n as usize { break; };
            pd(m(&mut b, &mut c), 2, 3);
            if (&c).len() == n as usize { break; };
        }
    }
}

enum Direction {
    FirstToSecond,
    SecondToFirst,
    None,
}

fn pd(d: Direction, f: i32, s: i32) {
    match d {
        Direction::FirstToSecond => { println!("{} {}", f, s); }
        Direction::SecondToFirst => { println!("{} {}", s, f); }
        Direction::None => {}
    }
}

fn m(p: &mut Vec<i32>, q: &mut Vec<i32>) -> Direction {
    if p.len() == 0 && q.len() == 0 {
        return Direction::None;
    }

    if p.len() == 0 {
        p.push(q.pop().unwrap());
        return Direction::SecondToFirst;
    }

    if q.len() == 0 {
        q.push(p.pop().unwrap());
        return Direction::FirstToSecond;
    }

    return if p.last().unwrap() < q.last().unwrap() {
        q.push(p.pop().unwrap());
        Direction::FirstToSecond
    } else {
        p.push(q.pop().unwrap());
        Direction::SecondToFirst
    };
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

