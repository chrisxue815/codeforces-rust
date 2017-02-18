use std::ascii::AsciiExt;
use std::io;
use std::str;

pub trait AsciiCheckExt {
    fn is_ascii_uppercase(&self) -> bool;
    fn is_ascii_lowercase(&self) -> bool;
}

impl AsciiCheckExt for u8 {
    fn is_ascii_uppercase(&self) -> bool {
        let c = *self;
        c >= b'A' && c <= b'Z'
    }

    fn is_ascii_lowercase(&self) -> bool {
        let c = *self;
        c >= b'a' && c <= b'z'
    }
}

pub fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let line = line.trim_right();

    print!("{}", fix_case(line));
}

pub fn fix_case(s: &str) -> String {
    let result = s.to_owned();
    let mut iter = s.bytes();

    let first = iter.next().unwrap();

    if iter.all(|c| c.is_ascii_uppercase()) {
        let mut result = result.into_bytes();
        result.make_ascii_lowercase();
        if first.is_ascii_lowercase() {
            result[0] = result[0].to_ascii_uppercase();
        }
        String::from_utf8(result).unwrap()
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("Hello", fix_case("hELLO"));
        assert_eq!("http", fix_case("HTTP"));
        assert_eq!("Z", fix_case("z"));
        assert_eq!("Caps", fix_case("cAPS"));
        assert_eq!("Lock", fix_case("Lock"));
    }
}
