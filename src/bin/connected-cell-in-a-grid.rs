use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Cell(u8, u8);

fn grid_get(grid: &[Vec<bool>], (i, j): (i32, i32)) -> bool {
    if i.is_negative() || j.is_negative() {
        return false;
    }
    grid.get(i as usize)
        .and_then(|row| row.get(j as usize))
        .map(|&b| b)
        .unwrap_or(false)
}
fn grid_get_mut(grid: &mut [Vec<bool>], (i, j): (i32, i32)) -> Option<&mut bool> {
    if i.is_negative() || j.is_negative() {
        return None;
    }
    grid.get_mut(i as usize)
        .and_then(|row| row.get_mut(j as usize))
}

// returns whether or not the specified cell is true.
// after the function call, the cell will be true.
fn visit_cell(visited: &mut [Vec<bool>], (i, j): (i32, i32)) -> Option<bool> {
    grid_get_mut(visited, (i, j)).and_then(|cell_visited| {
        let original_value = *cell_visited;
        *cell_visited = true;
        Some(original_value)
    })
}

fn find_connected_cells(grid: &[Vec<bool>], visited: &mut [Vec<bool>], (i, j): (i32, i32)) -> i32 {
    let is_empty = !grid_get(grid, (i, j));
    if is_empty {
        return 0;
    }
    let already_visited = visit_cell(visited, (i, j)).expect("coords should be in bounds now");
    if already_visited {
        return 0;
    }
    let mut count = 1;
    // traverse the moore neighborhood
    for x in -1..=1 {
        for y in -1..=1 {
            if x != 0 || y != 0 {
                let recursed = find_connected_cells(grid, visited, (i + x, j + y));
                count += recursed;
            }
        }
    }
    return count;
}

fn connected_cell(grid: &Vec<Vec<bool>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut max_count = 0;
    let mut visited = vec![vec![false; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            let pos = (i as i32, j as i32);
            if grid_get(grid, pos) {
                let count = find_connected_cells(grid, &mut visited, pos);
                max_count = max_count.max(count);
                println!("Loop done. Count: {count}. Max count: {max_count}");
            }
        }
    }
    return max_count;
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
