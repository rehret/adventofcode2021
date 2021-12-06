use super::common;

pub fn puzzle_01(input: Vec<String>) -> Result<i64, String> {
    let (random_values, mut boards) = common::parse_input(input)?;

    for value in random_values {
        if let Some(board_index) = common::mark_boards_and_check_for_winner(&mut boards, value) {
            return Ok(boards[board_index].get_sum_of_unmarked_spaces() as i64 * value as i64);
        }
    }

    Err(String::from("Did not find any winning boards"))
}
