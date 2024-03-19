// https://www.hackerrank.com/challenges/the-birthday-bar/problem
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn birthday(arr: &[i32], sum_to_match: i32, window_length: i32) -> i32 {
    arr.windows(window_length as usize)
        .filter(|&w| w.iter().sum::<i32>() == sum_to_match)
        .count() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let s: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let d = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let result = birthday(&s, d, m);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use crate::birthday;

    #[test]
    fn sample_input_0() {
        let result = birthday(&[1, 2, 1, 3, 2], 3, 2);
        assert_eq!(result, 2);
    }
    #[test]
    fn sample_input_1() {
        let result = birthday(&[1, 1, 1, 1, 1, 1], 3, 2);
        assert_eq!(result, 0);
    }
    #[test]
    fn sample_input_2() {
        let result = birthday(&[4], 4, 1);
        assert_eq!(result, 1);
    }
}
