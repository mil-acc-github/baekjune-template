use std::io;



fn main() {
    // main block
}

#[allow(unused)]
fn readline() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

#[allow(unused)]
fn readline_splitted() -> Vec<u64> {
    readline()
        .split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}
