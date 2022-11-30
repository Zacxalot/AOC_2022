use std::fs;
use std::time::Instant;

use crate::Answer;

struct BingoCard {
    card: Vec<(bool, u32)>,
    won: bool,
    winning_val: u32,
    win_pos: i32,
}

pub fn day_4_main() -> Answer {
    let time_before = Instant::now();

    let contents = fs::read_to_string("src/days/day_4/input1.txt").unwrap();

    // Get the numbers from the first line of the file
    let numberlist = contents
        .split('\n')
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let cards_strings = contents
        .split("\n\r")
        .map(|card| card.to_string())
        .skip(1)
        .collect::<Vec<String>>();

    // Turn each card string into a BingoCard
    // Parses numbers from a flattened version of the card and pairs each a marker flag with value before
    // collecting and putting it into the BingoCard, which is then collected into the final BingoCard Vec
    let mut cards = cards_strings
        .iter()
        .map(|card| {
            card.split_whitespace()
                .map(|val| (false, val.parse::<u32>().unwrap()))
                .collect::<Vec<(bool, u32)>>()
        })
        .map(|card| BingoCard {
            card,
            won: false,
            winning_val: 0,
            win_pos: -1,
        })
        .collect::<Vec<BingoCard>>();

    let mut win_counter = 0;

    // Closure used in fold to check all values in the iterator are true
    let fold_true = |state, (val, _): &(bool, u32)| state && *val;

    // Loop through all of the numbers and check each bingo card against it
    for val in numberlist {
        for bingo_card in cards.iter_mut() {
            // Marks the number on the card if found
            for (used, num) in bingo_card.card.iter_mut() {
                if num == &val {
                    *used = true
                }
            }

            // Only allow the card to win once
            if !bingo_card.won {
                // Gets length 5 chunks of the card and runs the fold_true closure
                let won_rows = bingo_card
                    .card
                    .chunks(5)
                    .map(|line| line.iter().fold(true, fold_true))
                    .any(|x| x);

                let mut won_cols = false;
                for x in 0..5 {
                    // Check columns for bingo with fold_true
                    if bingo_card
                        .card
                        .iter()
                        .skip(x)
                        .step_by(5)
                        .fold(true, fold_true)
                    {
                        won_cols = true
                    }
                }

                // If we found a bingo
                if won_cols || won_rows {
                    // Mark the card as won and when it won
                    bingo_card.won = true;
                    bingo_card.win_pos = win_counter;

                    // Calculate the score by finding the remaining numbers and multiplying
                    // it by the value that it won on
                    let remaining = bingo_card
                        .card
                        .iter()
                        .filter(|(used, _)| !used)
                        .map(|(_, val)| val)
                        .sum::<u32>();
                    bingo_card.winning_val = remaining * val;
                    win_counter += 1;
                }
            }
        }
    }

    // Part one is found by looking for the card with winning pos 0
    let part_1 = cards
        .iter()
        .find(|card| card.win_pos == 0)
        .unwrap()
        .winning_val;

    // Part 2 is found by finding the card with the largest win position
    let part_2 = cards
        .iter()
        .reduce(|a, b| if a.win_pos > b.win_pos { a } else { b })
        .unwrap()
        .winning_val;

    let duration = Instant::now() - time_before;

    Answer {
        day: 4,
        part_1: part_1.to_string(),
        part_2: part_2.to_string(),
        duration,
    }
}
