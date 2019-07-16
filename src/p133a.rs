use std::io;
use std::str;

pub fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let line = line.trim_end();

    let output = if any_hq9p_output(line) { "YES" } else { "NO" };

    print!("{}", output);
}

pub fn any_hq9p_output(s: &str) -> bool {
    s.bytes().any(|c| c == b'H' || c == b'Q' || c == b'9')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, any_hq9p_output("Hi!"));
        assert_eq!(false, any_hq9p_output("Codeforces"));
    }
}
