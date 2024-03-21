// https://www.hackerrank.com/challenges/day-of-the-programmer/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'dayOfProgrammer' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts INTEGER year as parameter.
 */

fn day_of_programmer(year: i32) -> String {
    todo!()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let year = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = day_of_programmer(year);

    writeln!(&mut fptr, "{}", result).ok();
}
