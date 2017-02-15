use std::io;

pub fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let n: u32 = line.trim_right().parse().unwrap();

    let mut output = 0;

    for _ in 0..n {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        let mut count = 0;

        for i in line.trim_right().split_whitespace().map(|s| s.parse::<u32>().unwrap()) {
            if i == 1 {
                count += 1;
            }
        }

        if count >= 2 {
            output += 1;
        }
    }

    print!("{}", output);
}
