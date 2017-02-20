use std::io;

pub fn main() {
    let n = read_num();
    let mut p = Vec::with_capacity(n as usize);

    for _ in 0..n {
        p.push(read_num());
    }

    print!("{}", num_groups(&p));
}

pub fn read_num() -> i32 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_right().parse::<i32>().unwrap()
}

pub fn num_groups(p: &[i32]) -> i32 {
    let n = p.len();
    let levels = vec![0; n].as_mut_ptr();
    let mut stack = Vec::with_capacity(n);
    let mut max_level = 0;

    unsafe {
        for i in 0..n {
            if *levels.offset(i as isize) != 0 {
                continue;
            }

            let mut curr_index = i;
            let mut level = 1;

            loop {
                let next_index = p[curr_index] - 1;
                let curr_level = *levels.offset(curr_index as isize);

                // the current node is a new root
                if next_index == -2 {
                    *levels.offset(curr_index as isize) = level;
                }

                // the current node is already in a tree
                if curr_level != 0 {
                    level = curr_level;
                }

                if next_index == -2 || curr_level != 0 {
                    while let Some(curr_index) = stack.pop() {
                        level += 1;
                        *levels.offset(curr_index as isize) = level;
                    }
                    break;
                } else {
                    stack.push(curr_index);
                    curr_index = next_index as usize;
                }
            }

            if max_level < level {
                max_level = level;
            }
        }
    }

    max_level
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, num_groups(&vec![-1, 1, 2, 1, -1]));
        assert_eq!(4, num_groups(&vec![4, 1, -1, 3]));
    }
}
