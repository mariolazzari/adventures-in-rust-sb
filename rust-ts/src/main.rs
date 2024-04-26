use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("cannot load file");

    let counts = content
        .split("\n\n")
        .map(|chunk| -> usize { chunk.split('\n').map(|row| row.parse().unwrap_or(0)).sum() });

    let mut v = counts.collect::<Vec<_>>();
    v.sort();

    let last_idx = v.len() - 1;

    print!("Last highest is: {}", v[last_idx]);
}
