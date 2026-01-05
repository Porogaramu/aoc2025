use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs,
    path::Path,
};
/// Prints out the Solutions to Day 9
pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p, 1000));
    println!("{}", solve2(&p));
}
/// Parses the input as lines of 3-dimensional coordinates
fn parse(s: &str) -> Vec<(u64, u64, u64)> {
    s.lines()
        .map(|l| {
            let mut split = l.split(',');
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}
/// Creates a Min-Heap of all the pairs of points, where it is sorted by the distance between each pair.
fn heap(v: &[(u64, u64, u64)]) -> BinaryHeap<Reverse<(u64, usize, usize)>> {
    let mut heap = BinaryHeap::new();
    // Iterate through each pair
    for (i, &p1) in v.iter().enumerate() {
        for (j, &p2) in v.iter().enumerate().skip(i + 1) {
            // Distance
            let d = p1.0.abs_diff(p2.0).pow(2)
                + p1.1.abs_diff(p2.1).pow(2)
                + p1.2.abs_diff(p2.2).pow(2);
            heap.push(Reverse((d, i, j)));
        }
    }
    heap
}
/// A Union Find Data Structure
/// p is a vector of index pointers to the parent node. When the index is the same as the value, it is a root node.
#[derive(Debug)]
struct UnionFind {
    p: Vec<usize>,
    sets: usize,
}
impl UnionFind {
    /// Creates a new Union Find with length n
    pub fn new(n: usize) -> Self {
        Self {
            p: (0..n).collect(),
            sets: n,
        }
    }
    /// Calculates the sizes of all the disjoint sets
    fn sizes(&mut self) -> Vec<usize> {
        // Updates all the nodes
        for i in 0..self.p.len() {
            self.update_nodes(i);
        }
        let mut map = HashMap::new();
        // Calculates the sizes of all the disjoint sets
        for &n in self.p.iter() {
            if let Some(n) = map.get_mut(&n) {
                *n += 1;
            } else {
                map.insert(n, 1);
            }
        }
        map.into_values().collect()
    }
    // Updates the node's parent node to a root along with all its ancestors.
    fn update_nodes(&mut self, mut i: usize) {
        let mut stack = Vec::new();
        // Add to the stack while not on a root
        while self.p[i] != i {
            stack.push(i);
            i = self.p[i];
        }
        // Set all the nodes' parents to the root
        while let Some(j) = stack.pop() {
            self.p[j] = i;
        }
    }
    /// Connects together two sets
    pub fn connect(&mut self, x: usize, y: usize) {
        if self.same_set(x, y) {
            return;
        }
        // A set is removed
        self.sets -= 1;
        // Sets are connected
        let root = self.p[x];
        self.p[root] = y;
    }
    /// Checks if the two points are in the same set
    pub fn same_set(&mut self, x: usize, y: usize) -> bool {
        self.update_nodes(x);
        self.update_nodes(y);
        self.p[x] == self.p[y]
    }
}
/// Returns the product of the sizes of the three largest subsets after n closest pairs are joined together.
/// This is done by using a heap with a Union Find data structure.
fn solve1(v: &[(u64, u64, u64)], n: usize) -> u64 {
    let mut heap = heap(v);
    let mut union = UnionFind::new(v.len());
    // Connect the n closest pairs together
    for _ in 0..n {
        let Reverse((_, i, j)) = heap.pop().unwrap();
        union.connect(i, j);
    }
    let mut sizes = union.sizes();
    // Sort the sizes in descending order and take the top three
    sizes.sort_by(|a, b| b.cmp(a));
    sizes.iter().take(3).map(|n| *n as u64).product()
}
/// Returns the product of the last pair of points X coordinates.  The last pair is when only one set remains after the pairing.
/// This is done by using a heap with a Union Find data structure.
fn solve2(v: &[(u64, u64, u64)]) -> u64 {
    let mut heap = heap(v);
    let mut union = UnionFind::new(v.len());
    // Pop from the heap
    while let Some(Reverse((_, i, j))) = heap.pop() {
        union.connect(i, j);
        // If the remaining sets are 0, then return the result
        if union.sets == 1 {
            return v[i].0 * v[j].0;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::eight::{parse, solve1, solve2};

    const TEST: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
    #[test]
    fn test_solve() {
        let p = parse(TEST);
        assert_eq!(solve1(&p, 10), 40);
        assert_eq!(solve2(&p), 25272);
    }
}
