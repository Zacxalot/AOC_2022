use std::{fs, time::Instant};

use crate::Answer;

pub fn execute() -> Answer {
    let time_before = Instant::now();
    let file = fs::read_to_string("./input/day_2.txt").unwrap();
    let time_no_io = Instant::now();

    let plays = file
        .lines()
        .map(|line| line.chars())
        .map(|mut chars| (chars.next().unwrap(), chars.nth(1).unwrap()));

    let part_1 = plays
        .clone()
        .map(|(l, r)| scorer(l, r))
        .sum::<u32>()
        .to_string();

    let part_2 = plays
        .map(|(l, r)| part_2_scorer(l, r))
        .sum::<u32>()
        .to_string();

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

pub fn scorer(l: char, r: char) -> u32 {
    if l == 'A' {
        if r == 'X' {
            return 4;
        }
        if r == 'Y' {
            return 8;
        }

        return 3;
    }

    if l == 'B' {
        if r == 'Y' {
            return 5;
        }
        if r == 'Z' {
            return 9;
        }
        return 1;
    }

    if r == 'Z' {
        return 6;
    }

    if r == 'X' {
        return 7;
    }

    2
}

pub fn part_2_scorer(l: char, r: char) -> u32 {
    let l_ascii = (l as u32) - 64;

    if r == 'X' {
        if l == 'A' {
            return 3;
        }

        return l_ascii - 1;
    }

    if r == 'Y' {
        return l_ascii + 3;
    }

    if r == 'Z' {
        if l == 'C' {
            return 7;
        }

        return l_ascii + 7;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_scorer() {
        assert_eq!(scorer('A', 'Y'), 8);
        assert_eq!(scorer('B', 'X'), 1);
        assert_eq!(scorer('C', 'Z'), 6);
    }

    #[test]
    fn check_part_2_scorer() {
        assert_eq!(part_2_scorer('A', 'Y'), 4);
        assert_eq!(part_2_scorer('B', 'X'), 1);
        assert_eq!(part_2_scorer('C', 'Z'), 7);
    }
}
