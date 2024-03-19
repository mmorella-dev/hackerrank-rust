// https://www.hackerrank.com/challenges/lonely-integer/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/// Given an array of integers, where all elements but one occur twice, returns the unique element. 
fn lonely_integer(arr: &[i32]) -> i32 {
    arr.iter()
        .map(|i| *i)
        .reduce(|ac, v| ac ^ v)
        .expect("Array is not empty")
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = lonely_integer(&a);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use crate::lonely_integer;

    #[test]
    fn sample_input_0() {
        assert_eq!(lonely_integer(&[1]), 1);
    }
    #[test]
    fn sample_input_1() {
        assert_eq!(lonely_integer(&[1, 1, 2]), 2);
    }
    #[test]
    fn sample_input_2() {
        assert_eq!(lonely_integer(&[0, 0, 1, 2, 1]), 2);
    }
}
