// https://www.hackerrank.com/challenges/sparse-arrays/problem
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn matching_strings(string_list: &[String], queries: &[String]) -> Vec<u32> {
    queries
        .iter()
        .map(|query| string_list.iter().filter(|&string| string == query).count() as u32)
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let string_list_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut string_list: Vec<String> = Vec::with_capacity(string_list_count as usize);

    for _ in 0..string_list_count {
        let string_list_item = stdin_iterator.next().unwrap().unwrap();
        string_list.push(string_list_item);
    }

    let queries_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut queries: Vec<String> = Vec::with_capacity(queries_count as usize);

    for _ in 0..queries_count {
        let queries_item = stdin_iterator.next().unwrap().unwrap();
        queries.push(queries_item);
    }

    let res = matching_strings(&string_list, &queries);

    for i in 0..res.len() {
        write!(&mut fptr, "{}", res[i]).ok();

        if i != res.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}

#[cfg(test)]
mod tests {
    use crate::matching_strings;

    #[test]
    fn sample_input_1() {
        let string_list: Vec<_> = ["aba", "baba", "aba", "xzxb"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let queries: Vec<_> = ["aba", "xzxb", "ab"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(matching_strings(&string_list, &queries), [2, 1, 0]);
    }
    #[test]
    fn sample_input_2() {
        let string_list: Vec<_> = ["def", "de", "fgh"].iter().map(|s| s.to_string()).collect();
        let queries: Vec<_> = ["de", "lmn", "fgh"].iter().map(|s| s.to_string()).collect();
        assert_eq!(matching_strings(&string_list, &queries), [1, 0, 1]);
    }
    #[test]
    fn sample_input_3() {
        let string_list: Vec<_> = [
            "abcde", "sdaklfj", "asdjf", "na", "basdn", "sdaklfj", "asdjf", "na", "asdjf", "na",
            "basdn", "sdaklfj", "asdjf",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let queries: Vec<_> = ["abcde", "sdaklfj", "asdjf", "na", "basdn"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(matching_strings(&string_list, &queries), [1, 3, 4, 3, 2]);
    }
}
