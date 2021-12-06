use super::common;

pub fn puzzle_01(input: Vec<String>) -> Result<i64, String> {
    let binary_length = input[0].len();

    let mut common_string: String = String::default();

    // Build a string of the most common digits
    for index in 0..binary_length {
        let most_common_digit = get_most_common_digit_in_position(&input[..], index)?;
        common_string += most_common_digit.to_string().as_str();
    }

    // Parse common digits as a binary string into u32
    let gamma_rate = common::parse_binary_string(&common_string)?;

    // Calculate epsilon by inverting gamma, then XOR-ing against as mask of the digits used in the puzzle input
    // For example, if the value is stored in a u32 and there are 16 bits in the puzzle input, the mask
    // would be 0xFFFF0000.
    let epsilon_rate = !gamma_rate ^ (u32::MAX - (2_u32.pow(binary_length as u32) - 1));

    Ok(gamma_rate as i64 * epsilon_rate as i64)
}

pub fn get_most_common_digit_in_position(input: &[String], position: usize) -> Result<u8, String> {
    let mut counts: [u32; 2] = [0; 2];
    for line in input {
        let character = line.chars().nth(position);
        match character {
            Some('0') => counts[0] += 1,
            Some('1') => counts[1] += 1,
            _ => {
                return Err(String::from(format!(
                    "Invalid binary digit '{}'",
                    character.map_or(String::from("None"), |char| char.to_string())
                )));
            }
        }
    }

    if counts[0] > counts[1] {
        Ok(0)
    } else {
        Ok(1)
    }
}
