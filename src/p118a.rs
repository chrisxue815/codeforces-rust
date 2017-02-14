use std::io;
use std::ascii::AsciiExt;

pub fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let line = line.trim_right();

    print!("{}", modify(&line));
}

pub fn modify(input: &str) -> String {
    let mut output = String::new();

    for c in input.bytes().map(|c| c.to_ascii_lowercase()) {
        match c {
            b'a' | b'o' | b'y' | b'e' | b'u' | b'i' => {}
            _ => {
                output.push('.');
                output.push(c as char);
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(".t.r", modify("tour"));
        assert_eq!(".c.d.f.r.c.s", modify("Codeforces"));
        assert_eq!(".b.c.b", modify("aBAcAba"));
    }
}
