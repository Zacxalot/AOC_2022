use itertools::Itertools;
use prettytable::format;

use crate::Answer;
use std::{collections::HashMap, fs, time::Instant};

#[derive(Debug)]
enum Instruction {
    CD(String),
    LS,
}

#[derive(Debug)]
enum Output {
    Dir(String),
    File(String, usize),
}

#[derive(Debug)]
struct Directory {
    local_size: usize,
    children: Vec<String>,
}

fn str_to_output(output: &str) -> Output {
    let mut split = output.split(' ');
    let left = split.next().unwrap();
    let right = split.next().unwrap();

    if left == "dir" {
        Output::Dir(right.to_string())
    } else {
        let size = left.parse::<usize>().unwrap();
        Output::File(right.to_string(), size)
    }
}

pub fn execute() -> Answer {
    let time_before = Instant::now();
    let file = fs::read_to_string("./input/day_7.txt").unwrap();
    let time_no_io = Instant::now();

    let instruction_lines = file.lines().filter(|line| line.starts_with('$'));

    let mut instructions: Vec<Instruction> = vec![];
    for line in instruction_lines {
        let mut split = line.split(' ');
        let inst_str = split.nth(1).unwrap();

        if inst_str == "cd" {
            instructions.push(Instruction::CD(split.next().unwrap().to_owned()))
        } else {
            instructions.push(Instruction::LS)
        }
    }

    let output_groups = file.lines().group_by(|line| line.starts_with('$'));

    let mut filtered_output_groups = output_groups
        .into_iter()
        .filter(|(matched, _group)| !*matched)
        .map(|(_matched, group)| group.map(str_to_output).collect::<Vec<Output>>());

    let mut directories: HashMap<String, Directory> = HashMap::new();

    let mut current_dir = "".to_string();
    for instruction in instructions {
        println!("{current_dir}");
        println!("{:?}", instruction);
        match instruction {
            Instruction::CD(directory) => {
                if directory == ".." {
                    let split_path = current_dir.split('\\').collect::<Vec<&str>>();
                    current_dir = split_path[0..split_path.len() - 1].join("\\");
                } else {
                    current_dir = format!("{current_dir}\\{directory}");
                }
            }
            Instruction::LS => {
                let output = filtered_output_groups.next().unwrap();
                let mut dir = directories.entry(current_dir.clone()).or_insert(Directory {
                    local_size: 0,
                    children: vec![],
                });

                for line in output {
                    println!("{:?}", line);
                    match line {
                        Output::Dir(dir_name) => {
                            dir.children.push(format!("{current_dir}\\{dir_name}"))
                        }
                        Output::File(_name, size) => dir.local_size += size,
                    }
                }
            }
        }
    }

    let mut leq_100000_total = 0;

    println!();
    // println!("{:?}", directories.get(&"mlm".to_owned()));

    for key in directories.keys() {
        println!("Dir {}", key);
        let mut children = vec![key.clone()];
        let mut total = 0;

        while let Some(dir) = children.pop() {
            // println!("Child {}", dir);
            let dir = directories.get(&dir).unwrap();

            for child in &dir.children {
                children.push(child.clone())
            }

            total += dir.local_size;
        }

        if total <= 100000 {
            leq_100000_total += total;
        }
    }

    let part_1 = leq_100000_total.to_string();
    let part_2 = "day".to_string();

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