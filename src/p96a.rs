use std::io;

pub fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let line = line.trim_right();

    let dangerous = if is_dangerous(line) { "YES" } else { "NO" };

    print!("{}", dangerous);
}

pub fn is_dangerous(pos: &str) -> bool {
    let mut pos = pos.bytes();
    let mut prev = pos.next().unwrap();
    let mut count = 1;

    for c in pos {
        if c == prev {
            count += 1;
        } else {
            count = 1;
            prev = c;
        }

        if count == 7 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, is_dangerous("1000000001"));
        assert_eq!(false, is_dangerous("001001"));
    }
}
