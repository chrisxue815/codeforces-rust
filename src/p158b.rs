// text_io macros

#[macro_export]
macro_rules! read(
    () => { read!("{}") };
    ($text:expr) => {{
        let value;
        scan!($text, value);
        value
    }};
    ($text:expr, $input:expr) => {{
        let value;
        scan!($input => $text, value);
        value
    }};
);

/// This macro allows to pass several variables so multiple values can be read
#[macro_export]
macro_rules! scan(
    ($text:expr, $($arg:expr),*) => {
        scan!(::std::io::stdin().bytes().map(|c| c.unwrap()) => $text, $($arg),*) ;
        format_args!($text, $($arg),*);
    };
    ($input:expr => $text:expr, $($arg:expr),*) => {{
        use ::std::io::Read;
        use ::std::str::FromStr;
        // typesafe macros :)
        let text: &'static str = $text;
        let stdin: &mut Iterator<Item = u8> = &mut ($input);

        let mut text = text.bytes();
        $(
        loop { match text.next() {
            Some(b'{') => match text.next() {
                Some(b'{') => assert_eq!(Some(b'{'), stdin.next()),
                Some(b'}') => {
                    let s: Vec<u8> = match text.next() {
                        Some(c) => stdin.take_while(|&ch| ch != c).collect(),
                        None => stdin.take_while(|ch| !b"\t\r\n ".contains(ch)).collect(),
                    };
                    let s = match ::std::str::from_utf8(&s) {
                        Ok(s) => s,
                        Err(e) => {
                            let n = e.valid_up_to();
                            if n == 0 {
                                panic!("input was not valid utf8: {:?}", s);
                            } else {
                                panic!("input was only partially valid utf8: \"{}\" followed by {:?}",
                                       ::std::str::from_utf8(&s[..n]).unwrap(), &s[n..]);
                            }
                        }
                    };
                    $arg = FromStr::from_str(s).expect(&format!("could not parse {} as target type of {}", s, stringify!($arg)));
                    break;
                }
                Some(_) => panic!("found bad curly brace"),
                None => panic!("found single open curly brace at the end of the format string"),
            },
            Some(c) => assert_eq!(Some(c), stdin.next()),
            None => panic!("Bad read! format string: did not contain {{}}"),
        } }
        )*
        for c in text {
            assert_eq!(Some(c), stdin.next());
        }
        format_args!($text, $($arg),*);
    }};
);

use std::io;

pub fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();

    let n = line.trim_right().parse::<u32>().unwrap();

    let mut s = vec![0, 0, 0, 0, 0];

    for _ in 0..n {
        let si: usize = read!();
        s[si] += 1;
    }

    print!("{}", num_cars(&mut s));
}

pub fn num_cars(s: &mut Vec<u32>) -> u32 {
    let mut num = s[4];

    if s[3] < s[1] {
        s[1] -= s[3];
    } else {
        s[1] = 0;
    }
    num += s[3];

    num += s[2] / 2;

    if s[2] % 2 != 0 {
        num += 1;
        if s[1] > 2 {
            s[1] -= 2;
        } else {
            s[1] = 0;
        }
    }

    if s[1] % 4 == 0 {
        num += s[1] / 4;
    } else {
        num += s[1] / 4 + 1;
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, num_cars(&mut vec![0, 1, 1, 2, 1]));
        assert_eq!(5, num_cars(&mut vec![0, 2, 2, 2, 2]));
    }
}
