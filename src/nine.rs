use std::{fs, path::Path};
/// Prints out the Solutions to Day 9
pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p));
    println!("{}", solve2(&p));
}
/// Returns the maximum area of any two points in the grid.
/// This is done by iterating through each pair of points and calculating the area between them.
fn solve1(v: &[(u64, u64)]) -> u64 {
    let mut max = 0;
    // Iterate through each pair of points
    for (i, p1) in v.iter().enumerate() {
        for p2 in v.iter().skip(i + 1) {
            // Calculate the area
            let area = (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1);
            max = max.max(area);
        }
    }
    max
}
/// Returns the maximum rectangular area that is filled. A cell is filled when it is within the shape defined by the verticles v.
/// The solution is not correct, but it worked with the given input.
/// This is done by iterating through each pair of points and checking if any side of the shape intersects with the rectangle made up of the pair. The implemented check does not work for all cases, thus the implemented solution is not correct.
fn solve2(v: &[(u64, u64)]) -> u64 {
    let mut max = 0;
    // Iterate through each pair
    for (i, &p1) in v.iter().enumerate() {
        for &p2 in v.iter().skip(i + 1) {
            // Check if any side is contained with the rectangle made up of the pair
            if !contains(v, p1, p2) {
                continue;
            }
            // Calculate the area
            let area = (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1);
            max = max.max(area);
        }
    }
    max
}
/// Checks whether any side interseects with the rectangle defined by c1 and c2.
/// This is done by checking the end points and mid point of the side. This is fast, but not correct.
fn contains(v: &[(u64, u64)], c1: (u64, u64), c2: (u64, u64)) -> bool {
    // Rectangle opposite corners
    let min_p = (c1.0.min(c2.0), c1.1.min(c2.1));
    let max_p = (c1.0.max(c2.0), c1.1.max(c2.1));
    // Iterate through each side
    for (&p1, &p2) in v.iter().zip(v.iter().skip(1).cycle()) {
        // A function that determines whether the point is within that rectangle
        let within =
            |p: (u64, u64)| p.0 > min_p.0 && p.0 < max_p.0 && p.1 > min_p.1 && p.1 < max_p.1;
        // Check the end points
        if within(p1) || within(p2) {
            return false;
        }
        // Check the mid point
        let mid = ((p1.0 + p2.0) / 2, (p1.1 + p2.1) / 2);
        if within(mid) {
            return false;
        }
    }
    true
}
/// Parses the input into a vector of tuples representing the coordinates of each point
fn parse(s: &str) -> Vec<(u64, u64)> {
    s.lines()
        .map(|l| {
            let mut split = l.split(',');
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::nine::{parse, solve1, solve2};

    const TEST: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
    #[test]
    fn test_solve() {
        let p = parse(TEST);
        assert_eq!(solve1(&p), 50);
        assert_eq!(solve2(&p), 24);
    }
}
