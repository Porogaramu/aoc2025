use std::{fs, ops::RangeInclusive, path::Path};

/// Prints out the solution to Day 2
pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let ranges = parse(&s);
    println!("{}", solve1(&ranges));
    println!("{}", solve2(&ranges));
}

/// Checks if n is made up of some number repeated twice
fn is_repeated_twice(n: u64) -> bool {
    let mut i = 0;
    // Calculates the length of n in base 10
    while 10u64.pow(i) < n {
        i += 1;
    }
    // If the length is odd, then a number cannot be repeated to make n
    if i % 2 == 1 {
        return false;
    }
    // Check if the first half and the second half of n are the same
    n % 10u64.pow(i / 2) == n / 10u64.pow(i / 2)
}

/// Checks if n is made up of some number repeated more than once
fn is_repeated(n: u64) -> bool {
    // Turns the number into a string
    let str = n.to_string();
    // Checks each length
    'outer: for l in (1..).take_while(|&e| e <= str.len() / 2) {
        let slice = &str[0..l];
        // Checks if the lengths work
        if str.len().is_multiple_of(l) {
            continue;
        }
        // Checks each repeated position
        for j in (0..(str.len())).step_by(l) {
            if &str[j..(j + l)] != slice {
                continue 'outer;
            }
        }
        return true;
    }
    false
}

/// Solves part 1
fn solve1(v: &[RangeInclusive<u64>]) -> u64 {
    let mut sum = 0;
    for r in v.iter().cloned() {
        for n in r {
            if is_repeated_twice(n) {
                sum += n;
            }
        }
    }
    sum
}

/// Solves part 2
fn solve2(v: &[RangeInclusive<u64>]) -> u64 {
    let mut sum = 0;
    for r in v.iter().cloned() {
        for n in r {
            if is_repeated(n) {
                sum += n;
            }
        }
    }
    sum
}

/// Parses the input string into a vector of ranges
/// 11-22,95-115 is turned into [11..=22, 95..=115]
fn parse(s: &str) -> Vec<RangeInclusive<u64>> {
    // Each range is separated by a comma
    s.split(',')
        // Maps the string range into a range
        .map(|ids| {
            let mut split = ids.split('-');
            let a1 = split.next().unwrap().parse().unwrap();
            let a2 = split.next().unwrap().parse().unwrap();
            a1..=a2
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::two::{is_repeated_twice, parse, solve1, solve2};

    const TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    #[test]
    fn test_solve() {
        assert!(is_repeated_twice(123123));
        let v = parse(TEST);
        assert_eq!(solve1(&v), 1227775554);
        assert_eq!(solve2(&v), 4174379265);
    }
}
