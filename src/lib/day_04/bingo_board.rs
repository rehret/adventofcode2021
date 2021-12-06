use lazy_static::lazy_static;
use regex::Regex;

pub const BOARD_SIZE: usize = 5;

#[derive(Copy, Clone, Debug)]
pub struct BingoBoard {
    board: [[BingoSpace; BOARD_SIZE]; BOARD_SIZE],
}

impl BingoBoard {
    pub fn mark(&mut self, value: u32) -> bool {
        let mut space_was_marked = false;
        for space in self.board.iter_mut().flat_map(|row| row) {
            if space.value == value {
                space.is_marked = true;
                space_was_marked = true;
            }
        }

        space_was_marked
    }

    pub fn is_winner(&self) -> bool {
        // collection of rows
        let mut cardinal_collections: Vec<Vec<BingoSpace>> =
            self.board.iter().map(|row| row.to_vec()).collect();

        // collection of columns
        for col_index in 0..BOARD_SIZE {
            cardinal_collections.push(self.board.iter().map(|row| row[col_index]).collect())
        }

        cardinal_collections
            .iter()
            .any(|collection| collection.iter().all(|space| space.is_marked))
    }

    pub fn get_sum_of_unmarked_spaces(&self) -> u32 {
        self.board
            .iter()
            .flat_map(|row| row)
            .filter(|space| !space.is_marked)
            .map(|space| space.value)
            .fold(0, |sum, value| sum + value)
    }

    pub fn parse(input: &[String]) -> Result<BingoBoard, String> {
        lazy_static! {
            static ref WHITESPACE_REGEX: Regex = Regex::new(r"\s+").unwrap();
        }

        if input.len() != BOARD_SIZE {
            return Err(format!(
                "Found board with invalid number of rows: {}",
                input.len()
            ));
        }

        let mut board = BingoBoard {
            board: [[BingoSpace {
                value: 0,
                is_marked: false,
            }; BOARD_SIZE]; BOARD_SIZE],
        };

        for (row_index, row) in input.iter().enumerate() {
            let cols: Vec<&str> = WHITESPACE_REGEX.split(row).collect();
            if cols.len() != BOARD_SIZE {
                return Err(format!(
                    "Found board with invalid number of columns: {}",
                    cols.len()
                ));
            }

            for (col_index, col) in cols.iter().enumerate() {
                board.board[row_index][col_index].value = col
                    .parse()
                    .or(Err(format!("Could not parse numeric value from '{}'", col)))?
            }
        }

        Ok(board)
    }
}

#[derive(Copy, Clone, Debug)]
struct BingoSpace {
    pub value: u32,
    pub is_marked: bool,
}
