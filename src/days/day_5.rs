use std::{fs, time::Instant, usize};

use itertools::Itertools;

use crate::Answer;

pub fn execute() -> Answer {
    let time_before = Instant::now();
    let file = fs::read_to_string("./input/day_5.txt").unwrap();
    let time_no_io = Instant::now();

    let stack_count = file
        .lines()
        .find(|line| line.starts_with(" 1"))
        .unwrap()
        .len()
        / 3;

    let mut crates: Vec<Vec<char>> = vec![vec![]; stack_count];

    let crates_lines = file.lines().filter(|line| line.contains('[')).rev();

    for line in crates_lines {
        for (index, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let id = chunk.nth(1).unwrap();
            if !id.is_whitespace() {
                crates[index].push(id)
            }
        }
    }

    let instructions = file
        .lines()
        .filter(|line| line.starts_with('m'))
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .map(|split| {
            (
                split[1].parse::<usize>().unwrap(),
                split[3].parse::<usize>().unwrap(),
                split[5].parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize, usize)>>();

    for (count, from, to) in instructions {
        let from_len = crates[from - 1].len();
        let mut to_move = crates[from - 1]
            .drain((from_len - count)..)
            .collect::<Vec<char>>();

        to_move.reverse();

        crates[to - 1].append(&mut to_move);
    }

    let part_1 = crates
        .iter()
        .map(|stack| stack.last().unwrap_or(&' '))
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    // for instruction in instructions {
    //     println!("{:?}", instruction);
    // }

    let duration = Instant::now() - time_before;
    let no_io_duration = Instant::now() - time_no_io;

    Answer {
        day: 5,
        part_1,
        part_2: "2".to_string(),
        duration,
        no_io_duration,
    }
}
