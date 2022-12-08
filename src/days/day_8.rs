use crate::Answer;
use std::{collections::HashMap, fs, time::Instant};

pub fn execute() -> Answer {
    let time_before = Instant::now();
    let file = fs::read_to_string("./input/day_8.txt").unwrap();
    let time_no_io = Instant::now();

    let trees = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let trees_width = trees[0].len();
    let trees_height = trees.len();

    let mut total = trees_width * 2 + trees_height * 2 - 4;

    for y in 1..(trees_height - 1) {
        for x in 1..(trees_width - 1) {
            let height = trees[y][x];
            let up = (0..y).map(|y| trees[y][x]).any(|val| val >= height);
            let down = ((y + 1)..trees_height)
                .map(|y| trees[y][x])
                .any(|val| val >= height);
            let left = (0..x).map(|x| trees[y][x]).any(|val| val >= height);
            let right = ((x + 1)..trees_width)
                .map(|x| trees[y][x])
                .any(|val| val >= height);

            if !up || !down || !left || !right {
                total += 1;
            }
        }
    }

    let part_1 = total.to_string();
    let part_2 = "2".to_owned();

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
