use super::bingo_board::BingoBoard;
use super::common;

pub fn puzzle_02(input: Vec<String>) -> Result<i64, String> {
    let (random_values, mut boards) = common::parse_input(input)?;

    let mut last_winning_board: Option<BingoBoard> = None;
    let mut last_winning_value: Option<u32> = None;

    for value in random_values {
        let (winning_boards, remaining_boards) =
            common::mark_boards_and_check_for_winners(boards, value);

        if let Some(&winning_board) = winning_boards.first() {
            last_winning_value = Some(value);
            last_winning_board = Some(winning_board);
        }

        boards = remaining_boards;
    }

    last_winning_board
        .and_then(|board| {
            last_winning_value.map(|value| board.get_sum_of_unmarked_spaces() as i64 * value as i64)
        })
        .ok_or(String::from("Did not find any winning boards"))
}
