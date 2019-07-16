use std::io;

pub fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let line = line.trim_end();

    let output = if hello(&line) { "YES" } else { "NO" };

    print!("{}", output);
}

pub fn hello(s: &str) -> bool {
    let mut iter = "hello".bytes();
    let mut curr = iter.next().unwrap();

    for c in s.bytes() {
        if c == curr {
            match iter.next() {
                Some(next) => curr = next,
                None => return true,
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, hello("ahhellllloou"));
        assert_eq!(false, hello("hlelo"));
    }
}
