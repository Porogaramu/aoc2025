use std::{fs, ops::RangeInclusive, path::Path};

/// Prints out the Solutions to Day 5
pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let parsed = parse(&s);
    println!("{}", solve1(&parsed.0, &parsed.1));
    println!("{}", solve2(&parsed.0));
}
/// Parses the input string into a vector of ranges and a vector of numbers
/// The ranges are fresh ingredient ID ranges and the numbers are the available ingredient IDs
/// The range string is `a-b` where `a` and `b` are the start and end of the range.
/// The ranges and numbers are separated by two newline characters.
fn parse(s: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut split = s.split("\n\n");
    let ranges = split.next().unwrap();
    let available_ids = split.next().unwrap();
    (
        ranges
            .lines()
            .map(|l| {
                let range = l.split_once('-').unwrap();
                range.0.parse().unwrap()..=range.1.parse().unwrap()
            })
            .collect(),
        available_ids.lines().map(|l| l.parse().unwrap()).collect(),
    )
}
/// Finds the number of fresh available ingredient IDs
/// This loops through each available ingredient ID and checks if it is contained within any of the ranges, and counts how many are true.
fn solve1(r: &[RangeInclusive<u64>], v: &[u64]) -> u32 {
    let mut count = 0;
    for &n in v {
        for r in r {
            if r.contains(&n) {
                count += 1;
                break;
            }
        }
    }
    count
}
/// Finds the number of fresh ingredient IDs
/// This is done by sorting the ranges based on their starting number. It keeps track of the last fresh ID used. It iterates through the sorted ranges, and counts the number of fresh ingredient IDs in it.
fn solve2(r: &[RangeInclusive<u64>]) -> u64 {
    let mut count = 0;
    let mut r = r.to_vec();
    // Sort the ranges based on the starting number of each range.
    r.sort_by(|a, b| a.start().cmp(b.start()));
    // Tracks the last fresh ID counted
    let mut last = 0;
    // Iterate through the sorted ranges and count the number of new fresh ingredient IDs in each range.
    for r in r {
        last = last.max(*r.start());
        // Skip range because it's already counted.
        if *r.end() < last {
            continue;
        }
        count += *r.end() - last + 1;
        last = *r.end() + 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::five::{parse, solve1, solve2};

    const TEST: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    #[test]
    fn test_solve1() {
        let parsed = parse(TEST);
        assert_eq!(solve1(&parsed.0, &parsed.1), 3);
        assert_eq!(solve2(&parsed.0), 14);
    }
}
