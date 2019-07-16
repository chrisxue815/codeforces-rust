use std::io;
use std::str;

pub fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let line = line.trim_end();

    print!("{}", capitalize(line));
}

pub fn capitalize(s: &str) -> String {
    let mut s = s.to_owned().into_bytes();
    s[0] = s[0].to_ascii_uppercase();
    unsafe { String::from_utf8_unchecked(s) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("ApPLe", capitalize("ApPLe"));
        assert_eq!("Konjac", capitalize("konjac"));
    }
}
