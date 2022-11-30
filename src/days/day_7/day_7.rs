use std::fs;
use std::time::Instant;

use crate::Answer;

pub fn day_7_main() -> Answer{
    let time_before = Instant::now();

    // Read our starting values into a vec of i32s and sort them
    let mut contents = fs::read_to_string("src/days/day_7/input1.txt")
                      .unwrap()
                      .split(",")
                      .map(|val| val.parse::<i32>().unwrap())
                      .collect::<Vec<i32>>();
    contents.sort();

    // Find the range of values to attempt
    let contents_min = *contents.first().unwrap();
    let contents_max = *contents.last().unwrap();

    let mut shortest_total = i32::MAX;

    // For each value in our range
    // find the distance to each submarine and sum
    for i in contents_min..contents_max+1{
        let mut total_distance = 0;
        for val in &contents{
            total_distance += (i - val).abs()
        }

        //Set shortest total out of the two
        shortest_total = shortest_total.min(total_distance);
    }

    let part_1 = shortest_total;

    // For each value in our range
    // find the distance to each submarine and sum
    let mut shortest_total = i32::MAX;
    for i in contents_min..contents_max+1{
        let mut total_distance = 0;
        for val in &contents{
            let x = (i-val).abs();
            // Part 2 equation
            // https://en.wikipedia.org/wiki/1_%2B_2_%2B_3_%2B_4_%2B_%E2%8B%AF
            total_distance += (x.pow(2) + x) / 2;
        }

        shortest_total = shortest_total.min(total_distance);
    }

    
    let part_2 = shortest_total;

    let duration = Instant::now() - time_before;

    Answer{day:7, part_1:part_1.to_string(), part_2:part_2.to_string(), duration:duration}
}