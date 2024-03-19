// https://www.hackerrank.com/challenges/divisible-sum-pairs/problem?isFullScreen=true
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/// Returns the number of unordered pairs in arr whose sums are divisible by k.
fn divisible_sum_pairs(_n: i32, k: i32, arr: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..arr.len() {
        for j in (i+1)..arr.len() {
            let sum = arr[i] + arr[j];
            let is_divisible = sum % k == 0;
            if is_divisible {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = divisible_sum_pairs(n, k, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use crate::divisible_sum_pairs;

    #[test]
    fn sample_input_0() {
        let result = divisible_sum_pairs(6, 3, &[1, 3, 2, 6, 1, 2]);
        assert_eq!(result, 5);
    }
}
