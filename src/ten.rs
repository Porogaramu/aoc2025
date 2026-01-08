use std::{
    collections::{HashSet, VecDeque},
    fs,
    path::Path,
    vec,
};
/// Prints out the Solutions to Day 10
pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p));
    println!("{}", solve2(&p));
}

type MachineConfig = (Vec<bool>, Vec<Vec<usize>>, Vec<u64>);

/// Finds the fewest number of button presses for each configuration and sums them up
/// This is done through a Breadth First Search
fn solve1(p: &[MachineConfig]) -> u64 {
    // Iterate through each configuration
    p.iter()
        // Map it to the least number of button presses
        .map(|config| {
            // Uses a Breadth First Search to find the minimum
            let mut queue = VecDeque::new();
            // The first element in the pair is the state, and the second is the number of button presses
            queue.push_back((vec![false; config.0.len()], 0));
            // Set to remove any duplicates
            let mut set = HashSet::new();
            // Pop from the queue
            while let Some((mut lights, c)) = queue.pop_front() {
                // Set checks
                if set.contains(&lights) {
                    continue;
                }
                set.insert(lights.clone());
                // Try every button press and add it to the queue
                for switch in config.1.iter() {
                    // Button press
                    let toggle = |v: &mut Vec<bool>| {
                        for &i in switch.iter() {
                            if i >= v.len() {
                                continue;
                            }
                            v[i] = !v[i];
                        }
                    };
                    toggle(&mut lights);
                    if lights == config.0 {
                        return c + 1;
                    }
                    queue.push_back((lights.clone(), c + 1));
                    toggle(&mut lights);
                }
            }
            unreachable!()
        })
        .sum()
}
/// Finds the sum of the minimum number of button presses needed to get to the correct joltage levels for each configuration.
/// This is done by using linear algebra (row reduction form) to find the free variables for the button presses. Then trying a bunch of combinations of free variable values to get the minimum.
fn solve2(p: &[MachineConfig]) -> u64 {
    p.iter()
        .map(|config| {
            // Create a matrix with the button presses and the joltage levels as columns.
            // Each row is the corresponding joltage indices
            let mut matrix = vec![vec![0i64; config.1.len() + 1]; config.2.len()];
            (0..config.1.len()).for_each(|i| {
                for &j in config.1[i].iter() {
                    matrix[j][i] = 1;
                }
            });
            // Add the joltage levels' column.
            (0..config.2.len()).for_each(|i| {
                *matrix[i].last_mut().unwrap() = config.2[i] as i64;
            });
            // Convert the matrix to row echelon form
            row_echelon(&mut matrix);
            // Find the free variables
            let free = free_vars(&matrix);
            // If there are no free variables, sum all the remaining variables to get the minimum
            if free.is_empty() {
                return matrix
                    .iter()
                    .map(|v| v.last().unwrap().unsigned_abs())
                    .sum();
            }
            let mut min = u64::MAX;
            // Try a bunch of combinations of free variables and select the minimum
            free_combinations(free.len(), |frees| {
                // Mult is the free variable button presses
                let mut mult = vec![0; matrix[0].len() - 1];
                for (&n, &m) in frees.iter().zip(free.iter()) {
                    mult[m] = n;
                }
                // The button presses for each button
                let values: Option<Vec<i64>> = matrix
                    .iter()
                    .map(|row| {
                        // This finds the joltage level of each index in terms of the free variables and substituting the free vartiables in
                        let mut value = *row.last().unwrap()
                            - row
                                .iter()
                                .cloned()
                                .zip(mult.iter().cloned())
                                // Substitution
                                .map(|(r, m)| r * m)
                                .sum::<i64>();
                        // Check if the pivot is negative. If so, then negate the previous joltage level
                        let pivot = *row.iter().find(|&&n| n != 0).unwrap();
                        if pivot < 0 {
                            value *= -1;
                        }
                        let pivot = pivot.abs();
                        // Check if the value is negative, then it is impossible to get that specific joltage level
                        // Check if the value is not divisble by the pivot. If so, then it is impossible to get that specific joltage level
                        if value < 0 || value % pivot != 0 {
                            return None;
                        }
                        // Return the quotient to get the number of button presses
                        Some(value / pivot)
                    })
                    .collect();
                // If any of the joltage levels are imppossible, then skip calculating any minimums
                if let Some(v) = values {
                    // Gets the number of button presses
                    let sum = v.iter().sum::<i64>() + frees.iter().sum::<i64>();
                    min = min.min(sum.unsigned_abs());
                }
            });
            min
        })
        .sum()
}
/// Returns the greatest common divisor of a and b using Euclid's Algorithm
fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
/// Returns the least common multiple of a and b using the formula lcm(a, b) = abs(a*b) / gcd(a, b)
fn lcm(a: i64, b: i64) -> i64 {
    let a = a.abs();
    let b = b.abs();
    if a == 0 || b == 0 {
        return 0;
    }
    a * (b / gcd(a, b))
}
/// Converts the matrix into row echelon form
fn row_echelon(matrix: &mut Vec<Vec<i64>>) {
    // The finished rows
    let mut row = 0;
    // Makes pivots and zeroes below it
    for j in 0..matrix[0].len() {
        // Find the first non zero element (in a row that does not have a pivot already) in the column and designate it as a pivot
        let pivot = (row..matrix.len()).find(|&i| matrix[i][j] != 0);
        if let Some(pivot) = pivot {
            // Swap the pivot row with the current row
            matrix.swap(row, pivot);
            // Make the pivot column
            for i in 0..matrix.len() {
                if i == row || matrix[i][j] == 0 {
                    continue;
                }
                // LCM of the pivot element and the corresponding element in the row
                let lcm = lcm(matrix[row][j], matrix[i][j]);
                let temp_m = lcm / matrix[row][j];
                // Multiply the pivot row to get the pivot element to be the lcm
                let temp: Vec<_> = matrix[row].iter().cloned().map(|n| n * temp_m).collect();
                let mult = lcm / matrix[i][j];
                // Multiply the other row to get the corresponding element to be the lcm and subtract by the pivot row
                matrix[i] = matrix[i]
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(i, n)| n * mult - temp[i])
                    .collect();
            }
            // Finished a row
            row += 1;
            if row >= matrix.len() {
                break;
            }
        }
    }
    // Reduces each row to have a gcd of all elements to equal 1
    for v in matrix.iter_mut() {
        // Iterator through all non-zero elements
        let mut iterator = v.iter().copied().filter(|&n| n != 0);
        let mut gcd_n = 0;
        // Loop through all elements and check if any are negative and get the gcd.
        if let Some(r) = iterator.next() {
            gcd_n = r;
            gcd_n = iterator.fold(gcd_n, gcd);
        }
        // Divide by that gcd (absolute value in case any negatives were in the row)
        v.iter_mut().for_each(|n| *n /= gcd_n.abs());
    }
    // Remove 0 rows
    for i in (0..matrix.len()).rev() {
        if matrix[i].iter().all(|n| *n == 0) {
            matrix.remove(i);
        }
    }
}
/// Finds the free variables in the matrix
fn free_vars(m: &[Vec<i64>]) -> Vec<usize> {
    // Free variables with indices
    let mut free = vec![true; m[0].len() - 1];
    m.iter().for_each(|row| {
        // Find the pivot element in the row and set that index as nonfree
        free[(0..(row.len() - 1)).find(|&i| row[i] != 0).unwrap()] = false;
    });
    // Change the free variable indices to a vector of numbers
    free.iter()
        .enumerate()
        .filter_map(|(i, &b)| if b { Some(i) } else { None })
        .collect()
}
/// Calls f with all possible combinations of free variables (0-500)
fn free_combinations(n: usize, mut f: impl FnMut(&[i64])) {
    fn helper(v: &mut Vec<i64>, n: usize, f: &mut impl FnMut(&[i64])) {
        if n == 0 {
            f(v);
            return;
        }
        for a in 0..=500 {
            v[n - 1] = a;
            helper(v, n - 1, f);
        }
    }
    helper(&mut vec![0; n], n, &mut f);
}
/// Parses the input into light switches, buttons, and joltage levels.
fn parse(s: &str) -> Vec<MachineConfig> {
    let mut parsed = Vec::new();
    // Each line is a configuration
    for l in s.lines() {
        let mut split = l.split(' ');
        // Lights is before the first space
        let lights = split.next().unwrap();
        let lights = lights.as_bytes()[1..(lights.len() - 1)]
            .iter()
            .map(|&b| b == b'#')
            .collect();
        // Joltage is after the last space
        let joltage = split.next_back().unwrap();
        let joltage = joltage[1..(joltage.len() - 1)]
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        // Buttons are the rest
        let v = split
            .map(|v| {
                v[1..(v.len() - 1)]
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect();
        parsed.push((lights, v, joltage));
    }
    parsed
}

#[cfg(test)]
mod tests {
    use crate::ten::{parse, solve1, solve2};

    const TEST: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
    #[test]
    fn test_solve() {
        let p = parse(TEST);
        assert_eq!(solve1(&p), 7);
        assert_eq!(solve2(&p), 33);
    }
}
