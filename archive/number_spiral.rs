
fn main() {
    let mut s = "".to_string();
    std::io::stdin().read_line(&mut s).expect("err");
    let n: u128 = s.trim().parse().expect("err");

    for _ in 0..n {
        let mut s = "".to_string();
        std::io::stdin().read_line(&mut s).expect("err");

        let mut split = s.
            trim().
            split(" ").
            map(|n| n.parse::<u128>().expect("err"));

        let (ji, ii) = (split.next().unwrap(), split.next().unwrap());

        let j = ji-1;
        let i = ii-1;


        let c = if i > j {
            if i % 2 == 0 {
                u128::pow(i + 1, 2) - j
            } else {
                u128::pow(i, 2) + 1 + j
            }
        } else {
            if j % 2 == 0 {
                u128::pow(j, 2) + 1 + i
            } else {
                u128::pow(j + 1, 2) - i
            }
        };

        println!("{}", c);
    }
}