use std::{fs, path::Path};
/// Prints out the Solution for Day 12
pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    let p = parse(&s);
    println!("{}", solve1(&p.0, &p.1));
}
/// Determines if the shapes can fit into the regions.
/// This is not a general solution, but it works for the actual input because it's a troll problem.
/// This is done by seeing whether the sizes of the shapes can fit into the region.
fn solve1(shapes: &[Vec<Vec<bool>>], regions: &[(u64, u64, Vec<u64>)]) -> u64 {
    let mut count = 0;
    // The size of each shape (constant across all shapes)
    let size = (shapes[0].len(), shapes[0][0].len());
    // Check each region and its shape requirements
    for r in regions {
        let num_shapes: u64 = r.2.iter().cloned().sum();
        // The total number of shapes that can fit into the region
        let total = r.0 * r.1 / size.0 as u64 / size.1 as u64;
        if total >= num_shapes {
            count += 1;
        }
    }
    count
}
/// Parses the input into a 3D vector of a list of shapes and a list of regions, where the size are the first two elements, and the shape requirements is the vector.
fn parse(s: &str) -> (Vec<Vec<Vec<bool>>>, Vec<(u64, u64, Vec<u64>)>) {
    // The shapes and regions are separated by double new lines
    let split1 = s.split("\n\n");
    let v: Vec<&str> = split1.collect();
    // The shapes are all but the last part of the input
    let shapes = v[0..(v.len() - 1)]
        .iter()
        .map(|shape| {
            shape
                .lines()
                // Skip the first line because it is an index
                .skip(1)
                // Map each line into a 2d vector of booleans representing the shape
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect()
        })
        .collect();
    // The regions are the last part
    let regions = v
        .last()
        .unwrap()
        .lines()
        .map(|l| {
            let mut split = l.split(": ");
            // Sizes
            let mut region = split.next().unwrap().split('x');
            let (x, y) = (
                region.next().unwrap().parse().unwrap(),
                region.next().unwrap().parse().unwrap(),
            );
            // Shape requirements
            let num = split
                .next()
                .unwrap()
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect();
            (x, y, num)
        })
        .collect();
    (shapes, regions)
}
