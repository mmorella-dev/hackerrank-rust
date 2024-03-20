// https://www.hackerrank.com/challenges/magic-square-forming/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/// a 3x3 grid
type Order3 = [[i32; 3]; 3];

/// flip a matrix along its diagonal and return the copy
fn flip_diagonal(s: &Order3) -> Order3 {
    let &[[a, b, c], [d, e, f], [g, h, i]] = s;
    [[a, d, g], [b, e, h], [c, f, i]]
}
/// rotate a matrix 90deg clockwise and return the copy
fn rotate_cw(s: &Order3) -> Order3 {
    let &[[a, b, c], [d, e, f], [g, h, i]] = s;
    [[g, d, a], [h, e, b], [i, f, c]]
}
/// create all of the possible magic squares with order 3
fn get_magic_squares() -> Vec<Order3> {
    let mut squares = Vec::with_capacity(8);
    let seed = [[8, 1, 6], [3, 5, 7], [4, 9, 2]];
    let mut spin = seed;
    for _ in 0..4 {
        squares.push(spin);
        squares.push(flip_diagonal(&spin));
        spin = rotate_cw(&spin);
    }
    squares
}

// given a square, find the cost of converting it to another square.
fn distance(s1: &Order3, s2: &Order3) -> i32 {
    s1.into_iter()
        .flatten()
        .zip(s2.into_iter().flatten())
        .map(|(s1, s2)| (s1 - s2).abs())
        .sum()
}

fn forming_magic_square(s: &Order3) -> i32 {
    let all_squares = get_magic_squares();
    all_squares.iter().map(|magic| distance(magic, s)).min().unwrap()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let mut s = [[0; 3]; 3];

    for i in 0..3_usize {
        let row : Vec<_> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
        let mut it = row.into_iter();
        for j in 0..3_usize {
            s[i][j] = it.next().unwrap();
        }
        
    }

    let result = forming_magic_square(&s);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use crate::forming_magic_square;

    #[test]
    fn sample_input_0() {
        assert_eq!(
            forming_magic_square(&[[4, 9, 2], [3, 5, 7], [8, 1, 5]]),
            1
        );
    }
    #[test]
    fn sample_input_1() {
        assert_eq!(
            forming_magic_square(&[[4, 8, 2], [4, 5, 7], [6, 1, 6]]),
            4
        );
    }
}
