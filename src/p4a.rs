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

pub fn main() {
    let w: u32 = read!();
    let result = dividable(w);
    print!("{}", result);
}

#[inline]
pub fn dividable(w: u32) -> &'static str {
    if w != 2 && w % 2 == 0 { "YES" } else { "NO" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("YES", dividable(8));
        assert_eq!("NO", dividable(7));
        assert_eq!("NO", dividable(2));
    }
}
