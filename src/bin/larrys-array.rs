// https://www.hackerrank.com/challenges/larrys-array/problem?isFullScreen=true

use std::cell::Cell;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn is_sorted(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}
/// determines whether an array can be sorted by rotating elements ABC -> BCA
fn larrys_array(arr: &[i32]) -> bool {
    let mut arr = arr.to_owned();
    // run iterations of the rotation step until none are possible
    while sort_step(&mut arr) {}
    // if the array is sorted, yes! otherwise no.
    is_sorted(&arr)
}
/// for each triple of elements, rotates them until the smallest element is first.
/// returns true if at least one rotation was performed
fn sort_step(arr:  &mut [i32]) -> bool {
    // use slice_of_cells as a workaround to slice::windows_mut
    let cells = Cell::from_mut(&mut arr[..]).as_slice_of_cells();
    let mut swapped_at_some_point = false;
    for w in cells.windows(3) {
        if let [a, b, c] = w {
            while a > b || a > c {
                a.swap(&b); // C B A
                b.swap(&c); // B C A
                swapped_at_some_point = true;
            }
        }
    }
    swapped_at_some_point
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..t {
        let _n = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();

        let a: Vec<i32> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
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
    use crate::{is_sorted, larrys_array};

    #[test]
    fn test_is_sorted() {
        assert_eq!(is_sorted(&[1, 2, 3]), true);
        assert_eq!(is_sorted(&[1, 3, 2]), false);
    }

    #[test]
    fn sample_input_1() {
        assert_eq!(larrys_array(&[3, 1, 2]), true);
    }
    #[test]
    fn sample_input_2() {
        assert_eq!(larrys_array(&[1, 3, 4, 2]), true);
    }
    
    #[test]
    fn sample_input_3() {
        assert_eq!(larrys_array(&[1, 2, 3, 5, 4]), false);
    }
}
