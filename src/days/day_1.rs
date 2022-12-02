use itertools::Itertools;
use std::{fs, time::Instant};

use crate::Answer;

pub fn execute() -> Answer {
    let time_before = Instant::now();
    let file = fs::read_to_string("./input/day_1.txt").unwrap();
    let time_no_io = Instant::now();

    let lines = file.lines();
    let elf_groups = lines.group_by(|val| !val.is_empty());

    let mut totals: Vec<usize> = vec![];

    for (matched, group) in &elf_groups {
        if matched {
            totals.push(
                group
                    .map(|val| val.parse::<usize>().unwrap())
                    .sum::<usize>(),
            );
        }
    }

    totals.sort();

    let totals_len = totals.len();

    let part_1 = totals[totals_len - 1].to_string();
    let part_2 = totals[totals_len - 4..totals_len - 1]
        .iter()
        .sum::<usize>()
        .to_string();

    let duration = Instant::now() - time_before;
    let no_io_duration = Instant::now() - time_no_io;

    Answer {
        day: 1,
        part_1,
        part_2,
        duration,
        no_io_duration,
    }
}

#[cfg(test)]
mod tests {}
