use std::i32;
use std::io;

pub fn main() {
    let n = read_i32() as usize;
    let mut p = Vec::with_capacity(n + 1);
    p.push(-1);

    for _ in 0..n {
        p.push(read_i32());
    }

    let result = num_groups(&mut p);
    print!("{}", result);
}

pub fn read_i32() -> i32 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().parse::<i32>().unwrap()
}

pub fn num_groups(p: &mut [i32]) -> i32 {
    let n = p.len() - 1;
    const HIGHEST: i32 = -1;
    let mut lowest = HIGHEST;

    for i in 1..=n {
        if p[i] < 0 {
            continue;
        }

        let mut j = i;
        let mut levels = 0;
        while p[j] >= 1 {
            j = p[j] as usize;
            levels += 1;
        }

        let top = p[j];
        let bottom = top - levels;
        if lowest > bottom {
            lowest = bottom
        }

        j = i;
        for level in bottom..top {
            let next = p[j];
            p[j] = level;
            j = next as usize;
        }
    }

    HIGHEST - lowest + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, num_groups(&mut vec![5, -1, 1, 2, 1, -1]));
        assert_eq!(4, num_groups(&mut vec![4, 4, 1, -1, 3]));
    }
}
