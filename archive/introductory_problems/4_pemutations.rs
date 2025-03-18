use std::io;
use std::io::Read;

fn main() {
    let mut s = "".to_string();
    io::stdin().read_line(&mut s).expect("err");

    let n = s.trim().parse::<u128>().expect("err");

    if n == 1 {
        print!("{}", 1);
        return;
    }

    if n < 4 {
        print!("NO SOLUTION");
        return;
    }

    for n in (2..(n + 1)).step_by(2) { print!("{} ", n) }
    for n in (1..(n + 1)).step_by(2) { print!("{} ", n) }
}