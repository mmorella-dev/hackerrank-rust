// https://www.hackerrank.com/challenges/larrys-array/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'larrysArray' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts INTEGER_ARRAY A as parameter.
 */

fn larrys_array(arr: &[i32]) -> bool {
    todo!()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let result = if larrys_array(&a) { "YES" } else { "NO" };

        writeln!(&mut fptr, "{}", result).ok();
    }
}

#[cfg(test)]
mod tests {
    use crate::larrys_array;

    #[test]
    fn sample_input_1() {
        assert_eq!(larrys_array(&[3, 1, 2]), true);
        assert_eq!(larrys_array(&[1, 3, 4, 2]), true);
        assert_eq!(larrys_array(&[1, 2, 3, 5, 4]), false);
    }
}