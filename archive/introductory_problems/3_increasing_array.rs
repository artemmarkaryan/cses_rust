use std::cmp::Ordering;
use std::io;
use std::io::Read;

fn main() {
    let mut s = "".to_string();
    io::stdin().
        read_line(&mut s).
        expect("err");

    s = "".to_string();

    io::stdin().
        read_line(&mut s).
        expect("err");

    let mut b = 0;
    let mut c: u128 = 0;
    for n in s.
        split(" ").
        map(|ch| { ch.trim().parse::<u128>().expect("err") }) {
        if b == 0 {
            b = n;
            continue;
        }

        match n.cmp(&b) {
            Ordering::Less => { c += b - n }
            Ordering::Equal => {}
            Ordering::Greater => { b = n }
        }
    }

    print!("{}", c);
}