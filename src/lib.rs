use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

/// Read a file into an iterator of lines
/// # Arguments
/// * `filename` - The name of the file to read
/// # Returns
/// An iterator of lines
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
