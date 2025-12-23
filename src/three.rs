use std::{fs, path::Path};

/// Prints out the solutions to day 3
pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let v = parse(&s);
    println!("{}", solve1(&v));
    println!("{}", solve2(&v));
}

/// Parses the input into a vector of vectors of digits
fn parse(s: &str) -> Vec<Vec<u8>> {
    // Each line is a number
    s.lines()
        // Map each line to a vector of digits
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Finds the sum of the largest numbers made by two digits in each row. This runs in O(n * d^2), where n is the number of numbers, and d is the number of digits in each number.
fn solve1(v: &[Vec<u8>]) -> u64 {
    let mut sum: u64 = 0;
    for v in v {
        let mut max = 0;
        // Take every pair of digits, combine them, and set the max
        for (i, &n1) in v.iter().enumerate() {
            for &n2 in v.iter().skip(i + 1) {
                max = max.max(n1 as u64 * 10 + n2 as u64);
            }
        }
        sum += max;
    }
    sum
}

/// Finds the sum of the largest numbers made by 12 digits in each row. This runs in O(n * d), where n is the number of numbers, and d is the number of digits in each number.
fn solve2(v: &[Vec<u8>]) -> u64 {
    v.iter().map(|v| max_combination(v, 12, 0)).sum()
}

/// Finds the largest number made by a certain number of digits
fn max_combination(v: &[u8], digs: usize, curr: u64) -> u64 {
    // Base case when no digits are left to add
    if digs == 0 {
        return curr;
    }
    let mut i = 0;
    let mut max = 0;
    let mut max_i = 0;
    // Loop through all the digits except for the last digs and find the max
    // This works because the largest number has to have the largest digit at the front
    while i <= v.len() - digs {
        if v[i] > max {
            max = v[i];
            max_i = i;
        }
        i += 1;
    }
    // Tail call recursion
    // Looks at the remaining digits in the vector
    max_combination(&v[(max_i + 1)..], digs - 1, curr * 10 + max as u64)
}

#[cfg(test)]
mod tests {
    use crate::three::{parse, solve1, solve2};

    const TEST: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";
    #[test]
    fn test_solve() {
        let v = parse(TEST);
        assert_eq!(solve1(&v), 357);
        assert_eq!(solve2(&v), 3121910778619u64);
    }
}
