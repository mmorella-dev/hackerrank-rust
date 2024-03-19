// https://www.hackerrank.com/challenges/sherlock-and-array/problem

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn balanced_sums(arr: &[i32]) -> bool {
    let mut left_sum: i32 = 0;
    let mut right_sum: i32 = arr.iter().sum();

    for n in arr {
        left_sum += n;
        if left_sum == right_sum {
            return true;
        }
        right_sum -= n;
    }
    return false;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let result = if balanced_sums(&arr) { "YES" } else { "NO" };

        writeln!(&mut fptr, "{}", result).ok();
    }
}

#[cfg(test)]
mod tests {
    use crate::balanced_sums;

    #[test]
    fn sample_input_0() {
        assert_eq!(balanced_sums(&[1, 2, 3]), false);
        assert_eq!(balanced_sums(&[1, 2, 3, 3]), true);
    }

    #[test]
    fn sample_input_1() {
        assert_eq!(balanced_sums(&[1, 1, 4, 1, 1]), true);
        assert_eq!(balanced_sums(&[2, 0, 0, 0]), true);
        assert_eq!(balanced_sums(&[0, 0, 2, 0]), true);
    }
}