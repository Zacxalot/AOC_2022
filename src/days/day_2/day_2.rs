use std::{fs, time::Instant};

use crate::Answer;

enum Instruction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

pub fn day_2_main() -> Answer {
    let time_before = Instant::now();

    // Parse the file and turn into a vec of "Instructions"
    let contents = fs::read_to_string("src/days/day_2/input1.txt")
        .unwrap()
        .split('\n')
        .map(|line| line.trim().split(' ').collect::<Vec<&str>>())
        .map(|vec| {
            (
                *vec.first().unwrap(),
                vec.get(1).unwrap().parse::<i32>().unwrap(),
            )
        })
        .map(|tuple| match tuple {
            ("forward", x) => Instruction::Forward(x),
            ("up", x) => Instruction::Up(x),
            ("down", x) => Instruction::Down(x),
            _ => Instruction::Forward(0),
        })
        .collect::<Vec<Instruction>>();

    // Loop through instructions and execute them
    let mut depth = 0;
    let mut horizontal = 0;

    for instruction in &contents {
        match instruction {
            Instruction::Forward(x) => horizontal += x,
            Instruction::Up(x) => depth -= x,
            Instruction::Down(x) => depth += x,
        }
    }

    let result_part_1 = depth * horizontal;

    // Reset values and repeat loop with different instructions
    depth = 0;
    horizontal = 0;
    let mut aim = 0;

    for instruction in &contents {
        match instruction {
            Instruction::Forward(x) => {
                horizontal += x;
                depth += aim * x
            }
            Instruction::Up(x) => aim -= x,
            Instruction::Down(x) => aim += x,
        }
    }

    let result_part_2 = depth * horizontal;

    let duration = Instant::now() - time_before;

    Answer {
        day: 2,
        part_1: result_part_1.to_string(),
        part_2: result_part_2.to_string(),
        duration,
    }
}
