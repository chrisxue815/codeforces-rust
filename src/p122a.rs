use std::io;

pub fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let num: u32 = line.trim_end().parse().unwrap();

    let output = if almost_lucky(num) { "YES" } else { "NO" };

    print!("{}", output);
}

pub fn almost_lucky(num: u32) -> bool {
    let lucky_nums = vec![4, 7, 44, 47, 74, 77, 444, 447, 474, 477, 744, 747, 774, 777];
    lucky_nums.iter().any(|i| num % i == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, almost_lucky(47));
        assert_eq!(true, almost_lucky(16));
        assert_eq!(false, almost_lucky(78));
    }
}
