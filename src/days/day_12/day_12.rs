use std::collections::{HashMap, HashSet};
use std::time::Instant;
use std::{fs, vec};

use crate::Answer;

pub fn day_12_main() -> Answer {
    let time_before = Instant::now();

    let mut mapped_pairs: HashMap<&String, Vec<(&String, bool)>> = HashMap::new();

    // Get all of the pairs from the list
    let pairs = fs::read_to_string("src/days/day_12/input1.txt")
        .unwrap()
        .split('\n')
        .map(|line| {
            line.trim()
                .split('-')
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let mut node_set: HashSet<String> = HashSet::new();

    for pair in &pairs {
        node_set.insert(pair[0].to_owned());
        node_set.insert(pair[1].to_owned());
    }

    for node in &node_set {
        mapped_pairs.insert(node, vec![]);
    }

    for pair in &pairs {
        let val_l = node_set.get(&pair[0]).unwrap();
        let val_r = node_set.get(&pair[1]).unwrap();

        let val_l_upper = val_l.chars().next().unwrap().is_uppercase();
        let val_r_upper = val_r.chars().next().unwrap().is_uppercase();

        mapped_pairs
            .get_mut(val_l)
            .unwrap()
            .push((val_r, val_r_upper));

        mapped_pairs
            .get_mut(val_r)
            .unwrap()
            .push((val_l, val_l_upper));
    }

    let mut current_path: Vec<&String> = vec![];
    let mut to_check_stack: Vec<Vec<&String>> = vec![];
    let mut total = 0;
    let mut dup_total = 0;

    let start = node_set.get("start").unwrap();
    let mut is_dup = false;

    to_check_stack.push(vec![start]);

    while !to_check_stack.is_empty() {
        let back_of_to_check = to_check_stack.last_mut().unwrap();
        let next_node = back_of_to_check.pop().unwrap();

        if next_node == "end" {
            if !is_dup {
                total += 1;
            }
            dup_total += 1;
        } else {
            if current_path.contains(&next_node)
                && !&next_node.chars().next().unwrap().is_uppercase()
            {
                is_dup = true;
            }

            // Get potential next nodes in the path
            let get_potentials = mapped_pairs.get(next_node).unwrap();
            {
                let filtered_potentials = get_potentials
                    .iter()
                    .filter(|(node, upper)| {
                        *upper || !current_path.contains(node) || (!is_dup && *node != "start")
                    })
                    .map(|(node, _upper)| *node)
                    .collect::<Vec<&String>>();

                // println!("{:?}", filtered_potentials);

                to_check_stack.push(filtered_potentials);
            }

            current_path.push(next_node);

            // println!("{:?} - is dup {}", current_path, is_dup);
        }

        while !to_check_stack.is_empty() && to_check_stack.last().unwrap().is_empty() {
            to_check_stack.pop();

            if let Some(node) = current_path.pop() {
                if current_path.contains(&node) && !node.chars().next().unwrap().is_uppercase() {
                    is_dup = false
                }
            }
        }
    }

    let part_1 = total;
    let part_2 = dup_total;

    let duration = Instant::now() - time_before;

    Answer {
        day: 12,
        part_1: part_1.to_string(),
        part_2: part_2.to_string(),
        duration,
    }
}
