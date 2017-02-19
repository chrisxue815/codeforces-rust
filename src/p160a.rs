use std::io;
use std::str;

pub fn read_numbers() -> Vec<u32> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn main() {
    io::stdin().read_line(&mut String::new()).unwrap();

    let coins = read_numbers();

    print!("{}", min_num_coins(coins));
}

pub fn min_num_coins(mut coins: Vec<u32>) -> u32 {
    coins.sort_by(|a, b| b.cmp(a));
    let half = coins.iter().sum::<u32>() / 2;
    let mut sum = 0;
    let mut i = 0;

    for coin in coins {
        if sum > half {
            break;
        }
        sum += coin;
        i += 1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, min_num_coins(vec![3, 3]));
        assert_eq!(2, min_num_coins(vec![2, 1, 2]));
    }
}
