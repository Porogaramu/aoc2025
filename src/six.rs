use std::{fs, path::Path};

/// Prints out the Solutions to Day 6
pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse1(&s);
    println!("{}", solve1(&p.0, &p.1));
    let p2 = parse2(&s);
    println!("{}", solve2(&p2.0, &p2.1));
}
/// Finds the sum of the operations on the vertical numbers.
/// This is done through simple iterations
/// p and o come from parse1
fn solve1(p: &[Vec<u64>], o: &[Operator]) -> u64 {
    let mut sum = 0;
    // Iterate through each column
    for i in 0..p[0].len() {
        let operator = o[i];
        // Sets the default value depending on the operation
        let mut add = operator.default_value();
        // Loop through each row and do the operation
        for j in 0..p.len() {
            match operator {
                Operator::Mult => add *= p[j][i],
                Operator::Plus => add += p[j][i],
            }
        }
        sum += add;
    }
    sum
}
/// Does the same as part 1, but the numbers are the digits in each column.
/// p and o come from parse2
fn solve2(p: &[Vec<u64>], o: &[Operator]) -> u64 {
    let mut sum = 0;
    // Loop through each row
    for i in 0..p.len() {
        let operator = o[i];
        let mut add = operator.default_value();
        // Do the operation on each column
        for j in 0..p[i].len() {
            match operator {
                Operator::Mult => add *= p[i][j],
                Operator::Plus => add += p[i][j],
            }
        }
        sum += add;
    }
    sum
}
/// Parases the input string into a vector of vectors and a vector of signs
/// The format contains a list of list of numbers and a list of signs. All the numbers in the same column correspond to the operator in that column.
fn parse1(s: &str) -> (Vec<Vec<u64>>, Vec<Operator>) {
    // Split based on lines
    let mut split = s.lines();
    // The operators are on the last line
    let operators = split
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|s| match s {
            "+" => Operator::Plus,
            "*" => Operator::Mult,
            _ => unreachable!(),
        })
        .collect();
    // Parse the rest of the numbers through whitespace splitting
    let numbers = split
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();
    (numbers, operators)
}
/// Parases the input string into a vector of vectors and a vector of signs
/// The format contains a list of list of numbers and a list of signs. The numbers are in columns with the most significant at the top. The numbers are chunked into groups of 3 corresponding to one operator.
fn parse2(s: &str) -> (Vec<Vec<u64>>, Vec<Operator>) {
    let mut lines = s.lines();
    // The operators line
    let operators = lines.next_back().unwrap();
    // The numbers lines
    let numbers: Vec<_> = lines.collect();
    let mut nums = Vec::new();
    let mut ops = Vec::new();
    // Loop through each column
    for column in 0..operators.as_bytes().len() {
        // If a new operator is in that column
        if operators.as_bytes()[column] != b' ' {
            ops.push(Operator::new(operators.as_bytes()[column]));
            nums.push(Vec::new());
        }
        // Gets the previous numbers to append a new number
        let v = nums.last_mut().unwrap();
        let mut n = 0;
        // Go through each row and create a new number
        for row in 0..numbers.len() {
            if numbers[row].as_bytes()[column] == b' ' {
                continue;
            }
            n *= 10;
            n += (numbers[row].as_bytes()[column] - b'0') as u64;
        }
        // Push that number to the numbers vector
        if n != 0 {
            v.push(n);
        }
    }
    (nums, ops)
}
/// An enum denoting the operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    /// Multiplication
    Mult,
    /// Addition
    Plus,
}
impl Operator {
    /// The default value for the operator
    pub fn default_value(self) -> u64 {
        match self {
            Operator::Mult => 1,
            Operator::Plus => 0,
        }
    }
    /// Parses the operator from a byte
    pub fn new(b: u8) -> Self {
        match b {
            b'+' => Operator::Plus,
            b'*' => Operator::Mult,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::six::{parse1, parse2, solve1, solve2};

    const TEST: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
    #[test]
    fn test_solve() {
        let p = parse1(TEST);
        assert_eq!(solve1(&p.0, &p.1), 4277556);
        let p2 = parse2(TEST);
        assert_eq!(solve2(&p2.0, &p2.1), 3263827);
    }
}
