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
    let (pos1, pos2) = pos.split_at(1);
    let mut last = pos1.bytes().nth(0).unwrap();
    let mut count = 1;

    for c in pos2.bytes() {
        if c == last {
            count += 1;
        } else {
            count = 1;
            last = c;
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
