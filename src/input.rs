use std::fs::{self, File};
use std::io::{BufRead, BufReader, Result};

pub fn read_lines(day: &str) -> Result<Vec<String>> {
    let file = File::open(filename(day))?;
    let lines = BufReader::new(file).lines();

    Ok(lines
        .filter(|x| x.is_ok() )
        .map(   |x| x.unwrap())
        .collect())
}

pub fn read_all(day: &str) -> Result<String> {
    fs::read_to_string(filename(day))
}

fn filename(day: &str) -> String {
    format!("input/{}.txt", day)
}