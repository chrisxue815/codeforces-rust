use std::io;
use std::str;

pub fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let n = line.trim_end().parse::<u32>().unwrap();

    for _ in 0..n {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        let line = line.trim_end();
        let line = line.as_bytes();
        let abbr = make_abbr(line);
        let abbr = str::from_utf8(abbr.as_slice()).unwrap();
        println!("{}", abbr);
    }
}

pub fn make_abbr(word: &[u8]) -> Vec<u8> {
    let len = word.len();
    if len > 10 {
        let mut abbr = Vec::with_capacity(4);
        let len_str = (len - 2).to_string();
        abbr.push(word[0]);
        abbr.extend_from_slice(len_str.as_bytes());
        abbr.push(word[len - 1]);
        abbr
    } else {
        word.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(b"a12345678a", make_abbr(b"a12345678a").as_slice());
        assert_eq!(b"a9a", make_abbr(b"a123456789a").as_slice());
        assert_eq!(b"i18n", make_abbr(b"internationalization").as_slice());
    }
}
