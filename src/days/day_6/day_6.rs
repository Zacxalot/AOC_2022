use std::fs;
use std::time::Instant;

use crate::Answer;

pub fn day_6_main() -> Answer{
    let time_before = Instant::now();

    // Read our starting values into a vec of usizes
    let contents = fs::read_to_string("src/days/day_6/input1.txt")
                      .unwrap()
                      .split(",")
                      .map(|val| val.parse::<usize>().unwrap())
                      .collect::<Vec<usize>>();

    let mut counts: [u64; 9] = [0;9];

    // For each value, increase that place in the counts array by 1
    for val in contents{
        counts[val] += 1;
    }

    // For each step find the value of new fish to be added
    // Rotate the regular fish left 1 (0,6)
    // Add the fish from 7 onto the fish that have just birthed new fish
    // Move the last steps baby fish onto 7
    // Set 8 to the new born fish count
    // Part 1:
    let mut add_to_end;
    for _ in 0..80{
        add_to_end = counts[0];
        counts[0..7].rotate_left(1);
        counts[6] += counts[7];
        counts[7] = counts[8];
        counts[8] = add_to_end;
        
    }
    
    let part_1 = counts.iter().sum::<u64>();

    // Part 2 - same as part 1 but for the remaining 176 steps:
    for _ in 0..176{
        add_to_end = counts[0];
        counts[0..7].rotate_left(1);
        counts[6] += counts[7];
        counts[7] = counts[8];
        counts[8] = add_to_end;
    }

    let part_2 = counts.iter().sum::<u64>();

    let duration = Instant::now() - time_before;

    Answer{day:6, part_1:part_1.to_string(), part_2:part_2.to_string(), duration:duration}
}