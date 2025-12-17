use std::{fs, path::Path};

/// Prints out the solution to day one.
pub fn run(p: &Path) {
    // Read the input file.
    let s = fs::read_to_string(p).unwrap();
    // Parse the input into a vector of integers.
    let v = parse(&s);

    println!("{}", solve(&v));
    println!("{}", solve2(&v));
}

/// Solves day 1
/// Finds the number of times the dial is left pointing at 0 after any rotation in sequence s.
fn solve(s: &[i32]) -> u32 {
    // The number of times the pointer hits 0
    let mut count = 0;
    // Start at 50
    let mut curr = 50;
    // Look through each instruction
    for &n in s {
        // Adds the current instruction
        // rem_euclid wraps negative numbers to positive numbers (i.e. -10 = 90)
        curr = (curr + n).rem_euclid(100);
        // If the pointer is at 0, then increment count
        if curr == 0 {
            count += 1;
        }
    }
    count
}

/// Solves day 2
/// Finds the number of times the dial touches 0 during of after any rotation in sequence s.
fn solve2(s: &[i32]) -> u32 {
    let mut count = 0;
    let mut curr = 50;
    // Loop through each instruction
    for &n in s {
        // Adds the current instruction
        let new = curr + n;
        // If the new value is greater than 100, then add the number of times it passes 0 to count
        if new >= 100 {
            count += new as u32 / 100;
        // If the new value is negative
        } else if new <= 0 {
            // Gets the number of times it passes 0
            count += (new / 100).unsigned_abs() 
            // If the pointer is already at 0, then don't add 1 to count
            + if curr != 0 { 1 } else { 0 };
        }
        // Sets the current pointer location
        curr = (curr + n).rem_euclid(100);
    }
    count
}

/// Parses the file.
/// L12 -> -12
/// R24 -> 24
fn parse(s: &str) -> Vec<i32> {
    // Iterate through each line
    s.lines()
        // Map each line to a number
        .map(|l| {
            // Make an iterator over each character
            let mut bytes = l.bytes();
            // The first character is the direction
            let sign = match bytes.next().unwrap() {
                b'R' => 1,
                b'L' => -1,
                _ => unreachable!(),
            };
            let mut n = 0;
            // The remaining characters are numbers
            for d in bytes {
                n *= 10;
                n += (d - b'0') as i32;
            }
            // Multiply the direction by the number
            sign * n
        })
        // Collect them all into a vector
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::one::{parse, solve, solve2};

    const TEST1: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    #[test]
    fn test_solve() {
        let s = parse(TEST1);
        assert_eq!(solve(&s), 3);
        assert_eq!(solve2(&s), 6);
    }
}
