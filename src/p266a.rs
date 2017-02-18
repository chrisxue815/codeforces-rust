use std::io;

pub fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    io::stdin().read_line(&mut line).unwrap();
    let line = line.trim_right();

    print!("{}", take_stone(line));
}

pub fn take_stone(s: &str) -> i32 {
    let mut s = s.bytes();
    let mut prev: u8 = s.next().unwrap();
    let mut result: i32 = 0;

    for curr in s {
        if curr == prev {
            result += 1;
        }
        prev = curr;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, take_stone("RRG"));
        assert_eq!(4, take_stone("RRRRR"));
        assert_eq!(0, take_stone("BRBG"));
    }
}
