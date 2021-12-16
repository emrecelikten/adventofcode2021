use crate::common::error::CommonError;
use std::io;

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

pub fn transform_iter<I, F, O, E, S>(iter: I, transformer: F) -> Result<Vec<O>, CommonError>
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

pub fn split_per_double_newline<S: AsRef<str>>(s: S) -> Vec<Vec<String>> {
    s.as_ref()
        .split("\n\n")
        .map(|chunk| chunk.lines().map(|s| s.to_string()).collect())
        .collect()
}
