use std::io;

pub fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let n: u32 = line.trim_end().parse().unwrap();

    let sum: i32 = (0..n)
        .map(|_| {
            let mut line = String::new();
            stdin.read_line(&mut line).unwrap();
            if line.contains('+') {
                1
            } else {
                -1
            }
        })
        .sum();

    print!("{}", sum);
}
