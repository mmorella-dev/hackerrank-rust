use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn connected_cell(grid: &Vec<Vec<bool>>) -> i32 {
    let mut grid = grid.to_owned();
    let mut max_count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let count = count_connected_cells(&mut grid, (i as i32, j as i32));
            max_count = max_count.max(count);
        }
    }
    return max_count;
}

fn count_connected_cells(grid: &mut [Vec<bool>], (i, j): (i32, i32)) -> i32 {
    if !get_cell(grid, (i, j)) {
        return 0;
    }
    set_cell(grid, (i, j), false);
    let mut count = 1;
    // traverse the moore neighborhood
    for x in -1..=1 {
        for y in -1..=1 {
            count += count_connected_cells(grid, (i + x, j + y));
        }
    }
    return count;
}

fn get_cell(grid: &[Vec<bool>], (i, j): (i32, i32)) -> bool {
    if grid_bounds(grid, (i, j)) {
        grid[i as usize][j as usize]
    } else {
        false
    }
}
fn set_cell(grid: &mut [Vec<bool>], (i, j): (i32, i32), val: bool) {
    if grid_bounds(grid, (i, j)) {
        grid[i as usize][j as usize] = val;
    }
}
fn grid_bounds(grid: &[Vec<bool>], (i, j): (i32, i32)) -> bool {
    !i.is_negative()
        && !j.is_negative()
        && (i as usize) < grid.len()
        && (j as usize) < grid[i as usize].len()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let m = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut grid: Vec<Vec<bool>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        grid.push(Vec::with_capacity(m as usize));

        grid[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .map(|n| n != 0)
            .collect();
    }

    let result = connected_cell(&grid);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use crate::connected_cell;

    #[test]
    fn sample_input_0() {
        let grid = [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0], [1, 0, 0, 0]]
            .iter()
            .map(|row| row.iter().map(|&i| i == 1).collect())
            .collect();
        let output = connected_cell(&grid);
        assert_eq!(output, 5);
    }
    #[test]
    fn sample_input_1() {
        let grid = [
            [1, 1, 0, 0, 0],
            [0, 1, 1, 0, 0],
            [0, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 1],
        ]
        .iter()
        .map(|row| row.iter().map(|&i| i == 1).collect())
        .collect();
        let output = connected_cell(&grid);
        assert_eq!(output, 5);
    }
}
