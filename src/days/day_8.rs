use crate::Answer;
use std::{fs, time::Instant};

fn path_scorer<I, J>(
    trees: &[std::vec::Vec<u32>],
    x_range: I,
    y_range: J,
    height: u32,
) -> (bool, u32)
where
    I: Iterator<Item = usize> + Clone,
    J: Iterator<Item = usize>,
{
    let mut distance = 1;
    for y in y_range {
        for x in x_range.clone() {
            if trees[y][x] >= height {
                return (true, distance);
            }

            distance += 1;
        }
    }
    (false, distance - 1)
}

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
    let mut best_score = 0;

    for y in 1..(trees_height - 1) {
        for x in 1..(trees_width - 1) {
            let height = trees[y][x];
            let up = path_scorer(&trees, x..(x + 1), (0..y).rev(), height);
            let down = path_scorer(&trees, x..(x + 1), (y + 1)..trees_height, height);
            let left = path_scorer(&trees, (0..x).rev(), y..(y + 1), height);
            let right = path_scorer(&trees, (x + 1)..trees_width, y..(y + 1), height);

            if !up.0 || !down.0 || !left.0 || !right.0 {
                total += 1;
            }

            let score = up.1 * down.1 * left.1 * right.1;

            if score > best_score {
                best_score = score;
            }
        }
    }

    let part_1 = total.to_string();
    let part_2 = best_score.to_string();

    let duration = Instant::now() - time_before;
    let no_io_duration = Instant::now() - time_no_io;

    Answer {
        day: 8,
        part_1,
        part_2,
        duration,
        no_io_duration,
    }
}
