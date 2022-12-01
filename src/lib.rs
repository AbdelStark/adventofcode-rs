//! Helper functions for the advent of code 2022

use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

/// Read a file into an iterator of lines
/// # Arguments
/// * `filename` - The name of the file to read
/// # Example
/// ```
/// let lines = read_lines("input.txt");
/// ```
/// # Returns
/// An iterator of lines
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
