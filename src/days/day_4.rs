use std::{fs, time::Instant, usize};

use crate::Answer;

fn contains(left: &[usize], right: &[usize]) -> bool {
    (left[0] <= right[0] && left[1] >= right[1]) || (right[0] <= left[0] && right[1] >= left[1])
}

fn overlaps(left: &[usize], right: &[usize]) -> bool {
    left[1] >= right[0] && left[0] <= right[1]
}

pub fn execute() -> Answer {
    let time_before = Instant::now();
    let file = fs::read_to_string("./input/day_4.txt").unwrap();
    let time_no_io = Instant::now();

    let parsed = file.lines().map(|line| {
        line.split([',', '-'])
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    });

    let part_1 = parsed
        .clone()
        .filter(|v| contains(&v[0..2], &v[2..4]))
        .count()
        .to_string();

    let part_2 = parsed
        .filter(|v| overlaps(&v[0..2], &v[2..4]))
        .count()
        .to_string();

    let duration = Instant::now() - time_before;
    let no_io_duration = Instant::now() - time_no_io;

    Answer {
        day: 4,
        part_1,
        part_2,
        duration,
        no_io_duration,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlaps_check() {
        assert!(!overlaps(&[2, 4], &[6, 8]));
        assert!(overlaps(&[5, 7], &[7, 9]));
    }
}
