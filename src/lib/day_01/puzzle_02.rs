use crate::input_helpers;

pub fn puzzle_02(input: Vec<String>) -> Result<i64, String> {
    let measurements: Vec<u32> = input_helpers::parse(input.iter());

    let mut previous_sum: Option<u32> = None;
    let mut increase_count = 0;

    // Count all sliding window sum increases
    for i in 0..measurements.len() - 2 {
        // Get the sliding window starting at i
        let triple = &measurements[i..i + 3];
        let triple_sum = sum_slice(&triple);

        if let Some(previous) = previous_sum {
            if triple_sum > previous {
                increase_count += 1;
            }
        }

        previous_sum = Some(triple_sum);
    }

    Ok(increase_count)
}

fn sum_slice(slice: &[u32]) -> u32 {
    slice.into_iter().fold(0, |sum, value| sum + value)
}
