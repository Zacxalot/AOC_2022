use std::{fs, time::Instant};

use crate::Answer;

pub fn execute() -> Answer {
    let time_before = Instant::now();

    let lines = fs::read_to_string("./input/day_1.txt")
        .unwrap()
        .split('\n')
        .map(|line| line.to_owned())
        .collect::<Vec<String>>();

    let mut chunk_total = 0;
    let mut highest_total = 0;
    for line in lines {
        if line.trim().is_empty() {
            if chunk_total > highest_total {
                highest_total = chunk_total
            }

            chunk_total = 0
        } else {
            println!("{}", line);
            chunk_total += line.trim().parse::<i32>().unwrap();
        }
    }

    let part_1 = highest_total.to_string();

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
