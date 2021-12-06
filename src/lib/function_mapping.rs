pub fn get_puzzle_function(
    day: u32,
    puzzle: u32,
) -> Option<fn(Vec<String>) -> Result<i64, String>> {
    match (day, puzzle) {
        (1, 1) => Some(crate::lib::day_01::puzzle_01),
        (1, 2) => Some(crate::lib::day_01::puzzle_02),

        (2, 1) => Some(crate::lib::day_02::puzzle_01),
        (2, 2) => Some(crate::lib::day_02::puzzle_02),

        (3, 1) => Some(crate::lib::day_03::puzzle_01),
        (3, 2) => Some(crate::lib::day_03::puzzle_02),

        (4, 1) => Some(crate::lib::day_04::puzzle_01),

        _ => None,
    }
}
