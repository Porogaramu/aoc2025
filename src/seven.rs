use std::{fs, path::Path};
/// Prints out the Solutions to Day 7
pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p));
    println!("{}", solve2(&p));
}
/// Returns the total number of times the beam was split (Only 1 beam per spot)
fn solve1(v: &[Vec<u8>]) -> u32 {
    // Cloned to allow mutations
    let mut v = v.to_vec();
    let mut count = 0;
    // Loop through each row
    for i in 0..(v.len() - 1) {
        // Loop through each column
        for j in 0..v[i].len() {
            let b = v[i][j];
            // If the current spot is a beam, skip
            if b != b'S' {
                continue;
            }
            // Splits the beam or go straight depending on the spot below
            match v[i + 1][j] {
                b'.' => v[i + 1][j] = b'S',
                b'^' => {
                    count += 1;
                    v[i + 1][j - 1] = b'S';
                    v[i + 1][j + 1] = b'S';
                }
                _ => {}
            }
        }
    }
    count
}
/// Returns the total number of times the beam was split (Any number of beams per spot)
fn solve2(v: &[Vec<u8>]) -> u64 {
    // Sets the total number of beams at each cell at the starting step
    let mut v_count: Vec<Vec<_>> = (0..v.len())
        .map(|i| {
            (0..v[i].len())
                .map(|j| if v[i][j] == b'S' { 1 } else { 0 })
                .collect()
        })
        .collect();
    // Iterate through each row
    for i in 0..(v.len() - 1) {
        // Iterate through each column
        for j in 0..v[i].len() {
            // Check the cell below
            match v[i + 1][j] {
                b'.' | b'S' => {
                    // If the beam can go to it, then add the number of beams to the cell below
                    v_count[i + 1][j] += v_count[i][j];
                }
                b'^' => {
                    // If it's a splitter, then add the number of beams to the cells beside the splitter
                    v_count[i + 1][j - 1] += v_count[i][j];
                    v_count[i + 1][j + 1] += v_count[i][j];
                }
                _ => {}
            }
        }
    }
    // Return the total number of beams
    v_count.last().unwrap().iter().sum()
}
/// Parses the string into a vector of vector of bytes
fn parse(s: &str) -> Vec<Vec<u8>> {
    s.lines().map(|l| l.as_bytes().to_vec()).collect()
}

#[cfg(test)]
mod tests {
    use crate::seven::{parse, solve1, solve2};

    const TEST: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
    #[test]
    fn test_solve() {
        let p = parse(TEST);
        assert_eq!(solve1(&p), 21);
        assert_eq!(solve2(&p), 40);
    }
}
