use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub enum Direction {
    Up(u32),
    Down(u32),
    Forward(u32),
}

pub fn parse_direction_from_input(input: Vec<String>) -> Result<Vec<Direction>, String> {
    input
        .iter()
        .map(|line| line.as_str())
        .map(parse_direction)
        .collect::<Option<Vec<Direction>>>()
        .ok_or(String::from("Failed to parse movements from input"))
}

fn parse_direction(line: &str) -> Option<Direction> {
    lazy_static! {
        static ref DIRECTION_REGEX: Regex = Regex::new(r"(?i)(up|down|forward)\s+(\d+)").unwrap();
    }

    match DIRECTION_REGEX.captures(line) {
        Some(captures) => {
            let direction = captures.get(1)?.as_str();
            let amount = captures.get(2).map(|match_group| {
                match_group
                    .as_str()
                    .parse::<u32>()
                    .expect("Found a non-numeric distance")
            })?;
            match direction {
                "up" => Some(Direction::Up(amount)),
                "down" => Some(Direction::Down(amount)),
                "forward" => Some(Direction::Forward(amount)),
                _ => None,
            }
        }
        None => None,
    }
}
