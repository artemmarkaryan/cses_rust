use std::cmp::max;

fn main() {
    let mut s: String = "".to_string();
    std::io::stdin().read_line(&mut s).expect("err");

    let ss = s.trim();

    let mut mseq = 1;
    let mut seq = 1;
    let mut lch = ss.chars().nth(0).expect("err");
    for ch in ss.chars().skip(1) {
        if ch == lch {
            seq += 1;
            mseq = max(seq, mseq);
        } else {
            seq = 1;
        }

        lch = ch;
    }

    print!("{}", max(seq, mseq));
}