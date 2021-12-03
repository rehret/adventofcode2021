use super::common::{self, Direction};

pub fn puzzle_01(input: Vec<String>) -> Result<i64, String> {
    let movements: Vec<Direction> = common::parse_direction_from_input(input)?;

    let mut horizontal_position: i64 = 0;
    let mut depth: i64 = 0;

    for movement in movements {
        match movement {
            Direction::Up(amount) => depth -= amount as i64,
            Direction::Down(amount) => depth += amount as i64,
            Direction::Forward(amount) => horizontal_position += amount as i64,
        }
    }

    Ok(horizontal_position * depth)
}
