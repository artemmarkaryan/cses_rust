use std::str::FromStr;

fn main() {
    let line: Vec<char>;
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
