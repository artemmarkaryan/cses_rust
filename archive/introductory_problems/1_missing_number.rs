fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_line(&mut s).expect("err");
    let n: i32 = s.trim().parse().expect("err");

    let mut sum: i32 = (1..(n + 1)).sum();

    s = "".to_string();
    std::io::stdin().read_line(&mut s).expect("err");

    for n in s.split(" ").map(|v: &str| v.trim().parse::<i32>().expect("err")) {
        sum -= n
    }

    print!("{}", sum)
}
