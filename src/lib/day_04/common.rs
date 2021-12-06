use super::bingo_board::{BingoBoard, BOARD_SIZE};

pub fn parse_input(input: Vec<String>) -> Result<(Vec<u32>, Vec<BingoBoard>), String> {
    // Assume first line is randomly drawn numbers
    let random_values = input[0]
        .split(',')
        .map(|value| {
            value.parse().or(Err(format!(
                "Could not parse numeric value from '{}'",
                value
            )))
        })
        .collect::<Result<Vec<u32>, String>>()?;

    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut line_index = 2; // Assume empty line after random numbers
    while line_index < input.len() {
        boards.push(BingoBoard::parse(
            &input[line_index..line_index + BOARD_SIZE],
        )?);

        line_index += BOARD_SIZE + 1; // Assume empty line between boards
    }

    Ok((random_values, boards))
}

pub fn mark_boards_and_check_for_winners(
    boards: Vec<BingoBoard>,
    value: u32,
) -> (Vec<BingoBoard>, Vec<BingoBoard>) {
    let mut winning_boards: Vec<BingoBoard> = Vec::default();
    let mut remaining_boards: Vec<BingoBoard> = Vec::default();

    for mut board in boards {
        if board.mark(value) && board.is_winner() {
            winning_boards.push(board);
        } else {
            remaining_boards.push(board);
        }
    }

    (winning_boards, remaining_boards)
}
