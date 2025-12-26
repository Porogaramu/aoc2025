use std::{fs, path::Path};

/// Prints out the solution of day four.
pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let mut v = parse(&s);
    println!("{}", solve1(&v));
    println!("{}", solve2(&mut v));
}
/// Checks if the cell at (i, j) can be removed, meaning it is surrounded by less than 4 rolls.
fn can_remove(v: &[Vec<bool>], i: usize, j: usize) -> bool {
    // Check if it's a roll
    if !v[i][j] {
        return false;
    }
    let mut rolls = 0;
    // Check the cells above the roll
    if i > 0 {
        if j > 0 && v[i - 1][j - 1] {
            rolls += 1;
        }
        if v[i - 1][j] {
            rolls += 1;
        }
        if j + 1 < v[0].len() && v[i - 1][j + 1] {
            rolls += 1;
        }
    }
    // Check the cells in the same row
    if j > 0 && v[i][j - 1] {
        rolls += 1;
    }
    if j + 1 < v[0].len() && v[i][j + 1] {
        rolls += 1;
    }
    // Check the cells in the cells below the roll
    if i + 1 < v.len() {
        if j > 0 && v[i + 1][j - 1] {
            rolls += 1;
        }
        if v[i + 1][j] {
            rolls += 1;
        }
        if j + 1 < v.len() && v[i + 1][j + 1] {
            rolls += 1;
        }
    }
    rolls < 4
}
/// Determines the number of rolls that can be removed on step 1.
/// This is done by iterating through each position and seeing if the roll can be removed (is surrounded by less than 4 rolls)
fn solve1(v: &[Vec<bool>]) -> u32 {
    let mut count = 0;
    // Iterates through each cell
    for (i, row) in v.iter().enumerate() {
        for (j, _) in row.iter().cloned().enumerate() {
            // If the cell is a roll, check if it can be removed
            if can_remove(v, i, j) {
                count += 1;
            }
        }
    }
    count
}
/// Recursively removes rolls from the board. This is done by recursively removing rolls that can be removed (are surrounded by less than 4 rolls) and calling the function on the surrounding rolls.
fn recursive_remove(v: &mut [Vec<bool>], i: usize, j: usize) -> u32 {
    // If the roll cannot be removed, then return 0
    if !can_remove(v, i, j) {
        return 0;
    }
    let mut count = 1;
    // Remove the roll
    v[i][j] = false;
    // Recursively call the function on the surrounding cells
    if i > 0 {
        if j > 0 {
            count += recursive_remove(v, i - 1, j - 1);
        }
        count += recursive_remove(v, i - 1, j);
        if j + 1 < v[0].len() {
            count += recursive_remove(v, i - 1, j + 1);
        }
    }
    if j > 0 {
        count += recursive_remove(v, i, j - 1);
    }
    if j + 1 < v[0].len() {
        count += recursive_remove(v, i, j + 1);
    }
    if i + 1 < v.len() {
        if j > 0 {
            count += recursive_remove(v, i + 1, j - 1);
        }
        count += recursive_remove(v, i + 1, j);
        if j + 1 < v[0].len() {
            count += recursive_remove(v, i + 1, j + 1);
        }
    }
    // Returns the total number of rolls removed
    count
}
/// Determines the number of rolls that can be removed in any number of steps.
/// This is done by recursively calling the remove function for each cell in the grid.
fn solve2(v: &mut [Vec<bool>]) -> u32 {
    let mut count = 0;
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            count += recursive_remove(v, i, j);
        }
    }
    count
}
/// Parses the input string into a 2D vector of booleans, where a roll is true and nothing is false.
/// '@' represents a roll, and '.' represents nothing.
fn parse(s: &str) -> Vec<Vec<bool>> {
    s.lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::four::{parse, solve1, solve2};

    const TEST: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
    #[test]
    fn test_solve() {
        let mut v = parse(TEST);
        assert_eq!(solve1(&v), 13);
        assert_eq!(solve2(&mut v), 43);
    }
}
