use super::common;

pub fn puzzle_02(input: Vec<String>) -> Result<i64, String> {
    let binary_length = input[0].len();
    let mut oxygen_set: Vec<String> = input.clone();
    let mut co2_set: Vec<String> = input.clone();

    let mut position: u8 = 0;
    while oxygen_set.len() != 1 && (position as usize) < binary_length {
        let groups = split_lines_into_groups_by_digit_in_position(&oxygen_set[..], position)?;
        oxygen_set = get_oxygen_generator_rating_set(groups);
        position += 1;
    }

    let mut position: u8 = 0;
    while co2_set.len() != 1 && (position as usize) < binary_length {
        let groups = split_lines_into_groups_by_digit_in_position(&co2_set[..], position)?;
        co2_set = get_co2_scrubber_rating_set(groups);
        position += 1;
    }

    let oxygen_rating = common::parse_binary_string(&oxygen_set[0])?;
    let co2_rating = common::parse_binary_string(&co2_set[0])?;

    Ok(oxygen_rating as i64 * co2_rating as i64)
}

fn split_lines_into_groups_by_digit_in_position(lines: &[String], position: u8) -> Result<[Vec<String>; 2], String> {
    let mut groups: [Vec<String>; 2] = [Vec::new(), Vec::new()];

    for line in lines {
        let character = line.chars().nth(position as usize);
        match character {
            Some('0') => groups[0].push(line.clone()),
            Some('1') => groups[1].push(line.clone()),
            _ => {
                return Err(String::from(format!(
                    "Invalid binary digit '{}'",
                    character.map_or(String::from("None"), |char| char.to_string())
                )));
            }
        }
    }

    Ok(groups)
}

fn get_oxygen_generator_rating_set(groups: [Vec<String>; 2]) -> Vec<String> {
    let [zero_in_pos, one_in_pos] = groups;
    if one_in_pos.len() >= zero_in_pos.len() {
        one_in_pos
    } else {
        zero_in_pos
    }
}

fn get_co2_scrubber_rating_set(groups: [Vec<String>; 2]) -> Vec<String> {
    let [zero_in_pos, one_in_pos] = groups;
    if zero_in_pos.len() <= one_in_pos.len() {
        zero_in_pos
    } else {
        one_in_pos
    }
}
