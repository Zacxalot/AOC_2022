use std::fs;
use std::time::Instant;

use ndarray::{s, Array2};

use crate::Answer;

pub fn day_11_main() -> Answer {
    let time_before = Instant::now();

    let mut num_map: Array2<u32> = Array2::zeros((10, 10));

    fs::read_to_string("src/days/day_11/input1.txt")
        .unwrap()
        .split('\n')
        .map(|line| line.trim().chars())
        .flatten()
        .enumerate()
        .for_each(|(pos, char)| {
            num_map[[pos.div_euclid(10), pos % 10]] = char.to_digit(10).unwrap()
        });

    let mut flash_count = 0;
    let mut sync_pos = 0;
    let mut step_counter = 0;

    while sync_pos == 0 || step_counter < 100 {
        step_counter += 1;

        // Add 1 to each octopus
        num_map += 1;

        // List of all of the values that need to be flashed
        let mut set_to_flash: Vec<(usize, usize)> = vec![];
        let mut checked = 0;

        // Keep track of what has already flashed
        let mut checked_poses: Array2<u32> = Array2::zeros((10, 10));

        // Find the first octopuses that need flashing and add them to the list
        for (pos, val) in num_map.iter().enumerate() {
            if val > &9 {
                set_to_flash.push((pos.div_euclid(10), pos % 10))
            }
        }

        while checked < set_to_flash.len() {
            let pos = set_to_flash[checked];

            if checked_poses[[pos.0, pos.1]] != 1 {
                // Adds 1 to each value surrounding the flashing octopus
                // Sorry about the horrible s![], not sure how to get edges of the array
                // without doing something like this yet
                num_map
                    .slice_mut(s![
                        ((pos.0 - 1) as i32).max(0)..(pos.0 + 2).min(10) as i32,
                        ((pos.1 - 1) as i32).max(0)..(pos.1 + 2).min(10) as i32
                    ])
                    .iter_mut()
                    .for_each(|v| *v += 1);
                checked_poses[[pos.0, pos.1]] = 1;
            }

            checked += 1;

            // Once we have got to the end of our list
            // look through the num_map and find octopuses that
            // have a value >= 10 and that haven't been flashed yet
            // and add them to the set_to_flash vec
            if checked == set_to_flash.len() {
                for (pos_check, val) in num_map.iter().enumerate() {
                    if val > &9 && checked_poses[[pos_check.div_euclid(10), pos_check % 10]] != 1 {
                        set_to_flash.push((pos_check.div_euclid(10), pos_check % 10))
                    }
                }
            }
        }

        // Turn all values that flashed to 0
        num_map.iter_mut().for_each(|v| {
            if *v > 9 {
                *v = 0
            }
        });

        // Checks if all are synced
        if num_map.iter().sum::<u32>() == 0 {
            sync_pos = step_counter;
        }

        flash_count += checked
    }

    let part_1 = flash_count;
    let part_2 = sync_pos;

    let duration = Instant::now() - time_before;

    Answer {
        day: 11,
        part_1: part_1.to_string(),
        part_2: part_2.to_string(),
        duration: duration,
    }
}
