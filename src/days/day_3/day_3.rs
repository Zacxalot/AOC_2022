use std::{fs, time::Instant};

use crate::Answer;

pub fn day_3_main() -> Answer {
    let time_before = Instant::now();

    // Get the lines out
    let lines = fs::read_to_string("src/days/day_3/input1.txt")
        .unwrap()
        .split('\n')
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    let line_len = lines[0].len();

    // Turn the all of the lines into a big vec of u32
    let bits = lines_to_bits(&lines);

    // Convert bits array to columns
    let columns = bits_to_columns(&bits, line_len);

    // Part 1
    // Find find the highest frequency in each column
    let common: Vec<u32> = get_frequency(&columns, lines.len(), false);
    let uncommon = invert_vec(&common);
    // Calculate answer
    let part_1 = vec_to_num(&common) * vec_to_num(&uncommon);

    // Part 2

    // Search for the oxygen value
    // Using string matching and converting the list to columns each time
    // Probably a quicker way to do this
    let mut search_oxy = format!["{}", common.first().unwrap()];
    for i in 1..line_len {
        let filtered = lines
            .iter()
            .filter(|x| x.starts_with(&search_oxy))
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();
        if filtered.len() == 1 {
            search_oxy = filtered.first().unwrap().to_owned();
        } else {
            let bits = lines_to_bits(&filtered);
            let columns = bits_to_columns(&bits, line_len);
            let frequency = get_frequency(&columns, filtered.len(), true);
            let val_to_push = frequency.get(i).unwrap();
            if *val_to_push == 1 {
                search_oxy.push('1');
            } else {
                search_oxy.push('0');
            }
        }
    }

    // Practically the same as the oxy search but we invert the most frequent value before adding to the search string
    let mut search_co2 = format!["{}", uncommon.first().unwrap()];
    for i in 1..line_len {
        let filtered = lines
            .iter()
            .filter(|x| x.starts_with(&search_co2))
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();
        if filtered.len() == 1 {
            search_co2 = filtered.first().unwrap().to_owned();
        } else {
            let bits = lines_to_bits(&filtered);
            let columns = bits_to_columns(&bits, line_len);
            let frequency = get_frequency(&columns, filtered.len(), true);
            let val_to_push = frequency.get(i).unwrap();

            if *val_to_push == 1 {
                search_co2.push('0');
            } else {
                search_co2.push('1');
            }
        }
    }

    // Convert the final values to u32 vecs and convert them to digits giving us the final answer
    let oxy_vec = search_oxy
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let co2_vec = search_co2
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let part_2 = vec_to_num(&oxy_vec) * vec_to_num(&co2_vec);

    let duration = Instant::now() - time_before;

    Answer {
        day: 3,
        part_1: part_1.to_string(),
        part_2: part_2.to_string(),
        duration,
    }
}

fn vec_to_num(vals: &Vec<u32>) -> u32 {
    let mut sum = 0;

    for (i, val) in vals.iter().enumerate() {
        sum += val * 2_u32.pow((vals.len() - i - 1) as u32);
    }

    sum
}

fn invert_vec(vals: &Vec<u32>) -> Vec<u32> {
    let mut return_vec: Vec<u32> = vec![];

    for val in vals {
        if *val >= 1 {
            return_vec.push(0)
        } else {
            return_vec.push(1)
        }
    }

    return_vec
}

// Lines to bits
fn lines_to_bits(lines: &[String]) -> Vec<u32> {
    lines
        .iter()
        .map(|line| line.chars().map(|chr| chr.to_digit(10).unwrap()))
        .flatten()
        .collect::<Vec<u32>>()
}

// Transpose array direction from lines to columns
fn bits_to_columns(bits: &[u32], line_len: usize) -> Vec<Vec<u32>> {
    let mut columns: Vec<Vec<u32>> = vec![];

    for i in 0..line_len {
        columns.push(
            bits.iter()
                .skip(i)
                .step_by(line_len)
                .copied()
                .collect::<Vec<u32>>(),
        );
    }

    columns
}

// Find find the highest frequency in each column
fn get_frequency(columns: &[Vec<u32>], line_len: usize, gte_flag: bool) -> Vec<u32> {
    let mut common: Vec<u32> = vec![];

    for val in columns.iter().map(|column| column.iter().sum::<u32>()) {
        if val * 2 > line_len as u32 || (gte_flag && val * 2 >= line_len as u32) {
            common.push(1);
        } else {
            common.push(0);
        }
    }

    common
}
