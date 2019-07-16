use std::io;

pub fn main() {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let s = s.trim_end();

    let result = sort(s);

    print!("{}", result);
}

pub fn sort(s: &str) -> String {
    let mut nums: Vec<u32> = s.split('+').map(|c| c.parse::<u32>().unwrap()).collect();
    nums.sort();
    let result: String = nums
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .as_slice()
        .join("+");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("1+2+3", sort("3+2+1"));
        assert_eq!("1+1+1+3+3", sort("1+1+3+1+3"));
        assert_eq!("2", sort("2"));
    }
}
