use super::bingo_board::BingoBoard;
use super::common;

pub fn puzzle_01(input: Vec<String>) -> Result<i64, String> {
    let (random_values, mut boards) = common::parse_input(input)?;

    for value in random_values {
        let (winning_boards, remaining_boards): (Vec<BingoBoard>, Vec<BingoBoard>) =
            common::mark_boards_and_check_for_winners(boards, value);

        if let Some(winning_board) = winning_boards.first() {
            return Ok(winning_board.get_sum_of_unmarked_spaces() as i64 * value as i64);
        }

        boards = remaining_boards;
    }

    Err(String::from("Did not find any winning boards"))
}
