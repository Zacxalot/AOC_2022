use std::collections::HashMap;
use std::fs;
use std::ops::Add;
use std::time::Instant;

use crate::Answer;

pub fn day_14_main() -> Answer {
    let time_before = Instant::now();

    let lines = fs::read_to_string("src/days/day_14/input1.txt")
        .unwrap()
        .split('\n')
        .map(|line| line.to_owned())
        .collect::<Vec<String>>();

    let mut lines_iter = lines.iter();

    let current_string = lines_iter.next().unwrap().trim().to_owned();

    let pairs = current_string
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|chars| chars.iter().cloned().collect::<String>())
        .collect::<Vec<String>>();

    println!("{:?}", pairs);

    //Skip empty line
    lines_iter.next();

    let mut mappings: HashMap<String, (String, String)> = HashMap::new();

    for line in lines_iter {
        let mut split = line.split(" -> ");

        let key = split.next().unwrap();
        let mut front = key.chars();
        let back = split.next().unwrap().chars().next().unwrap();

        let left = [front.next().unwrap(), back]
            .iter()
            .cloned()
            .collect::<String>();

        let right = [back, front.next().unwrap()]
            .iter()
            .cloned()
            .collect::<String>();

        println!("{} -> {} & {}", key, left, right);

        mappings.insert(key.to_owned(), (left, right));
    }

    let mut pair_count: HashMap<String, u32> = HashMap::new();

    for val in pairs {
        pair_count.insert(val, 1);
    }

    for _i in 0..10 {
        let mut new_pair_count: HashMap<String, u32> = HashMap::new();
        for (pair, count) in pair_count.iter_mut() {
            println!("{:?}", pair);
            if let Some(mapping) = mappings.get(pair) {
                if !new_pair_count.contains_key(&mapping.0) {
                    new_pair_count.insert(mapping.0.to_owned(), *count);
                } else {
                    let cur_count = new_pair_count.get(&mapping.0).unwrap();
                    new_pair_count.insert(mapping.0.to_owned(), cur_count + *count);
                }

                if !new_pair_count.contains_key(&mapping.1) {
                    new_pair_count.insert(mapping.1.to_owned(), *count);
                } else {
                    let cur_count = new_pair_count.get(&mapping.1).unwrap();
                    new_pair_count.insert(mapping.1.to_owned(), cur_count + *count);
                }
            }
        }
        println!("\n");
        pair_count = new_pair_count;
    }

    println!("{:?}", pair_count);

    let part_1 = "A";

    let part_2 = "B";

    let duration = Instant::now() - time_before;

    Answer {
        day: 14,
        part_1: part_1.to_string(),
        part_2: part_2.to_string(),
        duration,
    }
}
