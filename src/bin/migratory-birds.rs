// https://www.hackerrank.com/challenges/migratory-birds/problem?isFullScreen=true
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/// given an array of values 1-N, returns an array [2, 1, 2, 0, 1] containing the frequency of each value.
/// cheaper than a hashmap.
fn frequency_array<const N: usize>(bird_sightings: &[usize]) -> [usize; N] {
    let mut counts = [0; N];
    for &id in bird_sightings {
        let counter = counts.get_mut(id - 1).expect("id should be within range");
        *counter += 1;
    }
    counts
}
/// returns the index of the first instance of the largest value in the slice
fn index_of_first_max<V: std::cmp::Ord>(arr: &[V]) -> Option<usize> {
    arr.iter()
        .enumerate()
        .rev() // reverse so if there are multiple maximums, we return the first rather than last
        .max_by_key(|(_idx, v)| *v)
        .map(|(idx, _v)| idx)
}
/// given an array of values 1-5, returns the value which appears most frequently.
fn migratory_birds(bird_sightings: &[usize]) -> usize {
    let frequency = frequency_array::<5>(bird_sightings);
    let max_id = index_of_first_max(&frequency).unwrap();
    max_id + 1
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let arr: Vec<_> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse().unwrap())
        .collect();

    let result = migratory_birds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use crate::migratory_birds;

    #[test]
    fn sample_input_0() {
        let result = migratory_birds(&[1, 4, 4, 4, 5, 3]);
        assert_eq!(result, 4);
    }

    #[test]
    fn sample_input_1() {
        let result = migratory_birds(&[1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4]);
        assert_eq!(result, 3);
    }
}
