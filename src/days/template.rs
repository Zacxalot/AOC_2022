use std::time::Instant;

use crate::Answer;

#[allow(dead_code)]
pub fn execute() -> Answer {
    let time_before = Instant::now();

    // Code goes here!

    let part_1 = "Part 1".to_owned();

    let part_2 = "Part 2".to_owned();

    let duration = Instant::now() - time_before;

    Answer {
        day: 1,
        part_1,
        part_2,
        duration,
    }
}

#[cfg(test)]
mod tests {}
