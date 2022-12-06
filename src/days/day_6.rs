use crate::Answer;
use std::{fs, time::Instant, usize};

fn contains_dup(chars: &[char]) -> bool {
    for char in chars {
        if chars.iter().filter(|c| *c == char).count() > 1 {
            return true;
        }
    }

    false
}

fn find_packet(chars: &[char], packet_length: usize) -> String {
    (chars
        .windows(packet_length)
        .enumerate()
        .find(|(_index, chars)| !contains_dup(chars))
        .unwrap()
        .0
        + packet_length)
        .to_string()
}

pub fn execute() -> Answer {
    let time_before = Instant::now();
    let file = fs::read_to_string("./input/day_6.txt").unwrap();
    let time_no_io = Instant::now();

    let chars = file.trim().chars().collect::<Vec<char>>();

    let part_1 = find_packet(&chars, 4);
    let part_2 = find_packet(&chars, 14);

    let duration = Instant::now() - time_before;
    let no_io_duration = Instant::now() - time_no_io;

    Answer {
        day: 6,
        part_1,
        part_2,
        duration,
        no_io_duration,
    }
}
