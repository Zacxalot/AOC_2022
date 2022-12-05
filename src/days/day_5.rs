use std::{fs, time::Instant, usize};

use itertools::Itertools;

use crate::Answer;

fn process_crates(
    crates: &mut [Vec<char>],
    instructions: &Vec<(usize, usize, usize)>,
    crate_mover_9000: bool,
) {
    for (count, from, to) in instructions {
        let from_len = crates[from - 1].len();
        let mut to_move = crates[from - 1]
            .drain((from_len - count)..)
            .collect::<Vec<char>>();

        if crate_mover_9000 {
            to_move.reverse();
        }

        crates[to - 1].append(&mut to_move);
    }
}

fn crates_to_output_string(crates: &[Vec<char>]) -> String {
    crates
        .iter()
        .map(|stack| stack.last().unwrap_or(&' '))
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
}

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
    let mut part_2_crates: Vec<Vec<char>> = vec![vec![]; stack_count];

    let crates_lines = file.lines().filter(|line| line.contains('[')).rev();

    for line in crates_lines {
        for (index, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let id = chunk.nth(1).unwrap();
            if !id.is_whitespace() {
                crates[index].push(id);
                part_2_crates[index].push(id);
            }
        }
    }

    let instructions = file
        .lines()
        .filter(|line| line.starts_with('m'))
        .map(|line| line.split(' '))
        .map(|mut split| {
            (
                split.nth(1).unwrap().parse::<usize>().unwrap(),
                split.nth(1).unwrap().parse::<usize>().unwrap(),
                split.nth(1).unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize, usize)>>();

    process_crates(&mut crates, &instructions, true);
    process_crates(&mut part_2_crates, &instructions, false);

    let part_1 = crates_to_output_string(&crates);
    let part_2 = crates_to_output_string(&part_2_crates);

    let duration = Instant::now() - time_before;
    let no_io_duration = Instant::now() - time_no_io;

    Answer {
        day: 5,
        part_1,
        part_2,
        duration,
        no_io_duration,
    }
}
