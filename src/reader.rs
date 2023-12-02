use std::fs;
use std::io;

// The output is wrapped in a result to allow matching on errors
// Returns an iterator to the Reader of the lines of a file
pub fn read_lines(filename: String) -> Result<Vec<String>, io::Error> {
    let file = fs::read_to_string(filename)?;
    Ok(file.lines().map(&str::to_string).collect())
}
