use crate::error::CommonError;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

pub fn get_lines_iterator<T>(filename: T) -> io::Result<Lines<BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn transform_lines<I, F, O, E>(line_iter: I, transformer: F) -> Result<Vec<O>, CommonError>
where
    I: Iterator<Item = io::Result<String>>,
    F: Fn(&str) -> Result<O, E>,
    CommonError: From<E>,
{
    let mut vec = Vec::new();
    for line in line_iter {
        vec.push(transformer(&line?)?);
    }

    Ok(vec)
}

pub fn transform_iter<'a, I, F, O, E, S>(iter: I, transformer: F) -> Result<Vec<O>, CommonError>
where
    S: AsRef<str>,
    I: Iterator<Item = S>,
    F: Fn(&S) -> Result<O, E>,
    CommonError: From<E>,
{
    let mut vec = Vec::new();
    for line in iter {
        vec.push(transformer(&line)?);
    }

    Ok(vec)
}

pub fn read_file_as_strings<T>(filename: T) -> Result<Vec<String>, CommonError>
where
    T: AsRef<Path>,
{
    let lines = get_lines_iterator(filename)?;
    let mut vec = Vec::new();

    for line in lines {
        vec.push(line?);
    }

    Ok(vec)
}

pub fn read_file_as_numbers<T>(filename: T) -> Result<Vec<i64>, CommonError>
where
    T: AsRef<Path>,
{
    let lines = get_lines_iterator(filename)?;
    transform_lines(lines, |s| s.parse::<i64>())
}

pub fn split_double_newline<S: AsRef<str>>(s: S) -> Vec<Vec<String>> {
    s.as_ref()
        .split("\n\n")
        .map(|chunk| chunk.lines().map(|s| s.to_string()).collect())
        .collect()
}

pub fn read_file_as_string_groups<T: AsRef<Path>>(
    filename: T,
) -> Result<Vec<Vec<String>>, CommonError> {
    let file_content = read_to_string(filename)?;
    let chunks = split_double_newline(file_content);

    Ok(chunks)
}
