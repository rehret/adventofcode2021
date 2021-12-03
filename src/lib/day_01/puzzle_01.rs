use crate::input_helpers;

pub fn puzzle_01(input: Vec<String>) -> Result<i64, String> {
    let measurements: Vec<u32> = input_helpers::parse(input.iter());

    let mut previous_measurement = measurements[0];
    let mut increase_count = 0;

    // Count all measurement increases
    for &measurement in &measurements[1..] {
        if measurement > previous_measurement {
            increase_count += 1;
        }

        previous_measurement = measurement;
    }

    Ok(increase_count)
}
