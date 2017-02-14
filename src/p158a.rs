use std::io;

pub fn main() {
    let numbers = read_numbers();
    let n = numbers[0];
    let k = numbers[1];

    let numbers = read_numbers();

    let num_advanced = num_advanced(n, k, &numbers);
    print!("{}", num_advanced);
}

pub fn read_numbers() -> Vec<u32> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn num_advanced(n: u32, k: u32, a: &[u32]) -> u32 {
    let (v1, v2) = a.split_at(k as usize);

    for (i, ai) in v1.iter().enumerate() {
        if *ai <= 0 {
            return i as u32;
        }
    }

    let ak = *v1.last().unwrap();

    for (i, ai) in v2.iter().enumerate() {
        if *ai != ak {
            return k + i as u32;
        }
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
