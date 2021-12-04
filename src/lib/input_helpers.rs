use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::slice::Iter;
use std::str::FromStr;

pub fn get_contents_of_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect(format!("Could not open file: '{}'", path).as_str());
    let mut reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        let result = reader
            .read_line(&mut line)
            .expect(format!("Failed to read from file '{}'", path).as_str());

        if result == 0 {
            break;
        }

        lines.push(line.trim().to_string());
    }

    lines
}

pub fn parse<TContainer, TType>(input: Iter<String>) -> TContainer
where
    TContainer: FromIterator<TType>,
    TType: FromStr,
    <TType as FromStr>::Err: Debug,
{
    input
        .map(|line| line.trim())
        .map(|line| {
            line.parse::<TType>()
                .expect(format!("Could not parse '{}'", line).as_str())
        })
        .collect()
}
