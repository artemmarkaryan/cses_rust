use std::{collections::HashMap, io};

fn generate_permutations(s: &str) -> Vec<String> {
    let mut chars: Vec<char> = s.chars().collect();

    chars.sort_unstable();

    let mut freq: HashMap<char, i32> = HashMap::new();
    for c in chars.iter() {
        freq.entry(*c).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut result: Vec<String> = Vec::new();

    fn next_permutation(
        result: &mut Vec<String>,
        freq: &mut HashMap<char, i32>,
        chars: &Vec<char>,
        permutation: &mut Vec<char>,
        position: i32,
    ) {
        if position == chars.len() as i32 {
            result.push(permutation.iter().collect());
            return;
        }

        let mut used: HashMap<char, bool> = HashMap::new();

        for c in chars {
            if used.contains_key(c) {
                continue;
            }
            if *freq.get(c).unwrap() == 0 {
                continue;
            }

            permutation.push(*c);

            used.insert(*c, true);
            freq.entry(*c).and_modify(|e| *e -= 1);

            next_permutation(result, freq, chars, permutation, position + 1);

            permutation.pop();
            freq.entry(*c).and_modify(|e| *e += 1);
        }
    }

    next_permutation(&mut result, &mut freq, &chars, &mut Vec::new(), 0);

    return result;
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let buf = buf.trim();

    let permutations = generate_permutations(buf);

    println!("{}", permutations.len());
    for permutation in permutations.iter() {
        println!("{}", permutation);
    }
}
