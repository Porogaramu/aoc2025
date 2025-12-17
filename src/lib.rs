use std::{path::Path, process::exit};

/// An array of functions that run on a given day.
const FNS: &[fn(&Path)] = &[];

/// Runs the function for a given day.
pub fn run(n: u32, f: &Path) {
    // First index is 0
    let i = (n - 1) as usize;
    // Out of bounds check
    if i >= FNS.len() {
        eprintln!("Invalid day");
        exit(0);
    } else {
        // Calls the day's function
        FNS[i](f);
    }
}
