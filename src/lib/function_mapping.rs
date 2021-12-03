pub fn get_puzzle_function(
    day: u32,
    puzzle: u32,
) -> Option<fn(Vec<String>) -> Result<i64, String>> {
    match (day, puzzle) {
        (1, 1) => Some(crate::lib::day_01::puzzle_01),
        _ => None,
    }
}
