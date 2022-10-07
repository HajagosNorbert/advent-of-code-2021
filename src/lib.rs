
use std::fs;
use std::io::{self, BufRead};
use std::str::FromStr;


pub fn read_one_per_line<T>(path: &str) -> Result<Vec<T>, T::Err>
where
    T: FromStr,
{
    let file = fs::File::open(path).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse())
        .collect()
}

