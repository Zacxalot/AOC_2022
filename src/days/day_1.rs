use std::{fs, time::Instant};

use crate::Answer;

pub fn execute() -> Answer {
    let time_before = Instant::now();

    let lines = fs::read_to_string("./input/day_1.txt")
        .unwrap()
        .split('\n')
        .map(|line| line.to_owned())
        .collect::<Vec<String>>();

    let mut highest_totals: [i32; 3] = [0; 3];

    let mut chunk_total = 0;
    for line in lines {
        if line.trim().is_empty() {
            if chunk_total >= highest_totals[2] {
                highest_totals[0] = highest_totals[1];
                highest_totals[1] = highest_totals[2];
                highest_totals[2] = chunk_total;
            } else if chunk_total >= highest_totals[1] {
                highest_totals[0] = highest_totals[1];
                highest_totals[1] = chunk_total;
            } else if chunk_total > highest_totals[0] {
                highest_totals[0] = chunk_total;
            }
            chunk_total = 0;
        } else {
            chunk_total += line.trim().parse::<i32>().unwrap();
        }
    }

    let part_1 = highest_totals[2].to_string();

    let part_2 = highest_totals.iter().sum::<i32>().to_string();

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
