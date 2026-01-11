use std::{collections::HashMap, fs, path::Path};
/// Prints out the Solutions to Day 11
pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p));
    println!("{}", solve2(&p));
}
/// Parses the input into a hashmap where each key is a device and the value is a list of devices it can connect to.
/// This is just a directed graph
fn parse(s: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    // Each line is a device and its connections
    for l in s.lines() {
        let mut split = l.split(": ");
        // First device is the key
        let key = split.next().unwrap().to_string();
        // Connections
        let v: Vec<String> = split
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();
        map.insert(key, v);
    }
    map
}
/// Calculates the number of paths from curr to end
fn number_paths(
    map: &HashMap<String, Vec<String>>,
    counts: &mut HashMap<String, u64>,
    curr: &str,
    end: &str,
) -> u64 {
    // Base case
    if curr == end {
        return 1;
    }
    // Memoization
    if let Some(&c) = counts.get(curr) {
        return c;
    }
    if let Some(v) = map.get(curr) {
        // Calculate the sum of the nubmer of paths from all the connections to the end
        let count: u64 = v.iter().map(|s| number_paths(map, counts, s, end)).sum();
        counts.insert(curr.to_string(), count);
        return count;
    }
    0
}
/// Returns the number of paths from `you` to `out`
fn solve1(map: &HashMap<String, Vec<String>>) -> u64 {
    number_paths(map, &mut HashMap::new(), "you", "out")
}
/// Returns the number of paths from `svr` to `out` that visit `dac` and `fft`
fn solve2(map: &HashMap<String, Vec<String>>) -> u64 {
    // The number of paths from svr -> dac -> fft -> out
    let a = number_paths(map, &mut HashMap::new(), "svr", "dac");
    let b = number_paths(map, &mut HashMap::new(), "dac", "fft");
    let c = number_paths(map, &mut HashMap::new(), "fft", "out");
    // The number of paths from svr -> fft -> dac -> out
    let a2 = number_paths(map, &mut HashMap::new(), "svr", "fft");
    let b2 = number_paths(map, &mut HashMap::new(), "fft", "dac");
    let c2 = number_paths(map, &mut HashMap::new(), "dac", "out");
    // The total number of paths
    a * b * c + a2 * b2 * c2
}

#[cfg(test)]
mod tests {
    use crate::eleven::{parse, solve1, solve2};

    const TEST: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const TEST2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";
    #[test]
    fn test_solve() {
        let map = parse(TEST);
        assert_eq!(solve1(&map), 5);
        let map = parse(TEST2);
        assert_eq!(solve2(&map), 2);
    }
}
