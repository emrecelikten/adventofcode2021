use std::fs::{read_to_string, File};
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::str::FromStr;

use crate::common::error::CommonError;
use crate::common::parse;

pub fn get_lines_iterator<T>(filename: T) -> io::Result<Lines<BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_as_strings<T>(filename: T) -> Result<Vec<String>, CommonError>
where
    T: AsRef<Path>,
{
    let lines = get_lines_iterator(filename)?;
    let mut strs = Vec::new();
    for line in lines {
        strs.push(line?);
    }
    Ok(strs)
}

pub fn read_lines_as_numbers<T>(filename: T) -> Result<Vec<i64>, CommonError>
where
    T: AsRef<Path>,
{
    let lines = get_lines_iterator(filename)?;
    parse::transform_lines(lines, |s| s.parse::<i64>())
}

/// Reads a file as String chunks separated by double newlines. e.g.
///
/// lorem
/// ipsum
///
/// dolor
/// sit
///
/// becomes ```[["lorem", "ipsum"], ["dolor", "sit"]]```
pub fn read_lines_as_string_groups<T: AsRef<Path>>(
    filename: T,
) -> Result<Vec<Vec<String>>, CommonError> {
    let file_content = read_to_string(filename)?;
    let chunks = parse::split_per_double_newline(file_content);

    Ok(chunks)
}

pub fn read_lines_as_structs<O, T>(filename: T) -> Result<Vec<O>, CommonError>
where
    O: FromStr,
    CommonError: From<<O as FromStr>::Err>,
    T: AsRef<Path>,
{
    let lines = get_lines_iterator(filename)?;
    parse::transform_lines(lines, |s| O::from_str(s))
}
