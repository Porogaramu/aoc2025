use std::{env::args, path::PathBuf, process::exit};

use aoc2025::run;

fn main() {
    // Get Command Line Arguments
    let mut args = args();
    // Skip the executable file
    args.next();
    // Gets the day
    let day = match args.next() {
        // Parses the day number
        Some(a) => match a.parse::<u32>() {
            Ok(n) => n,
            // If error, then print and exit
            Err(e) => {
                eprintln!("{e}");
                exit(0);
            }
        },
        None => {
            eprintln!("No day inputted");
            exit(0);
        }
    };
    // Gets the file
    let file = match args.next() {
        Some(f) => PathBuf::from(f),
        None => {
            eprintln!("No file provided");
            exit(0)
        }
    };
    // Runs the day with the specified file
    run(day, &file);
}
