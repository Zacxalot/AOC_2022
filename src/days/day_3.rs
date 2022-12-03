use std::{fs, time::Instant, usize};

use crate::Answer;

fn char_to_priority(c: char) -> usize {
    let ascii = c as u8;
    if ascii >= 97 {
        (ascii - 96) as usize
    } else {
        (ascii - 38) as usize
    }
}

pub fn execute() -> Answer {
    let time_before = Instant::now();
    let file = fs::read_to_string("./input/day_3.txt").unwrap();
    let time_no_io = Instant::now();

    let bags = file
        .lines()
        .map(|line| {
            (
                line[0..line.len() / 2].to_owned(),
                line[line.len() / 2..line.len()].to_owned(),
            )
        })
        .collect::<Vec<(String, String)>>();

    let pairs: usize = bags
        .iter()
        .map(|(c1, c2)| c1.chars().find(|c1_char| c2.contains(*c1_char)).unwrap())
        .map(char_to_priority)
        .sum();

    let part_1 = pairs.to_string();

    let part_2 = "2".to_owned();

    let duration = Instant::now() - time_before;
    let no_io_duration = Instant::now() - time_no_io;

    Answer {
        day: 2,
        part_1,
        part_2,
        duration,
        no_io_duration,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
