use std::io;

pub fn main() {
    let mut a = String::new();
    let mut b = String::new();

    io::stdin().read_line(&mut a).unwrap();
    io::stdin().read_line(&mut b).unwrap();

    let a = a.trim_end();
    let b = b.trim_end();

    let result = compare(a, b);

    print!("{}", result);
}

pub fn compare(a: &str, b: &str) -> i32 {
    let a = a.bytes().map(|c| c.to_ascii_lowercase());
    let b = b.bytes().map(|c| c.to_ascii_lowercase());

    for (i, j) in a.zip(b) {
        if i < j {
            return -1;
        } else if i > j {
            return 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(0, compare("aaaa", "aaaA"));
        assert_eq!(-1, compare("abs", "Abz"));
        assert_eq!(1, compare("abcdefg", "AbCdEfF"));
    }
}
