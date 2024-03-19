// https://www.hackerrank.com/challenges/crush/problem
use std::io::{self, BufRead};

struct Query(usize, usize, i64);

fn array_manipulation(n: usize, queries: &[Query]) -> i64 {
    // fill an array with zeroes
    let mut arr = vec![0; n + 1];
    // populate the array with delta values from queries
    for &Query(a, b, k) in queries {
        arr[a - 1] += k;
        arr[b] -= k;
    }
    // take a running sum across the array
    let scan = arr.into_iter().scan(0, |sum, n| {
        *sum += n;
        Some(*sum)
    });
    // find the maximum
    scan.max().unwrap_or(0)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lines = stdin.lock().lines();
    // first line: n m
    let (array_size, query_count) = {
        let first_line = stdin_lines.next().unwrap().unwrap(); // unwrap result
        let mut first_multiple_input = first_line
            .split_whitespace() // split
            .filter_map(|s| s.parse::<usize>().ok()); // parse
        (
            first_multiple_input.next().unwrap(),
            first_multiple_input.next().unwrap(),
        )
    };
    // remaining lines: (a, b, k)
    let mut queries = Vec::with_capacity(query_count);
    stdin_lines
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<u32>>()
        })
        .map(|v| Query(v[0] as usize, v[1] as usize, v[2] as i64))
        .for_each(|q| queries.push(q));
    // invoke function with input
    let result = array_manipulation(array_size, &queries);
    // write output
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use crate::{array_manipulation, Query};

    #[test]
    fn sample_input_0() {
        assert_eq!(
            array_manipulation(5, &[Query(1, 2, 100), Query(2, 5, 100), Query(3, 4, 100)]),
            200
        );
    }
    #[test]
    fn sample_input_1() {
        assert_eq!(
            array_manipulation(10, &[Query(1, 5, 3), Query(4, 8, 7), Query(6, 9, 1)]),
            10
        );
    }
    #[test]
    fn sample_input_2() {
        assert_eq!(
            array_manipulation(
                10,
                &[
                    Query(2, 6, 8),
                    Query(3, 5, 7),
                    Query(1, 8, 1),
                    Query(5, 9, 15)
                ]
            ),
            31
        );
    }
}
