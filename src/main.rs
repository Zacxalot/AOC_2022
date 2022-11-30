mod days;

use std::time::Duration;

use days::day_1::day_1::day_1_main;
use days::day_10::day_10::day_10_main;
use days::day_11::day_11::day_11_main;
use days::day_12::day_12::day_12_main;
use days::day_13::day_13::day_13_main;
use days::day_14::day_14::day_14_main;
use days::day_2::day_2::day_2_main;
use days::day_3::day_3::day_3_main;
use days::day_4::day_4::day_4_main;
use days::day_5::day_5::day_5_main;
use days::day_6::day_6::day_6_main;
use days::day_7::day_7::day_7_main;
use days::day_8::day_8::day_8_main;
use days::day_9::day_9::day_9_main;

use prettytable::{cell, row, Row, Table};

pub struct Answer {
    day: u32,
    part_1: String,
    part_2: String,
    duration: Duration,
}

impl Answer {
    fn to_row(&self) -> Row {
        row![
            &self.day,
            &self.part_1,
            &self.part_2,
            &format!("{}μs", &self.duration.as_micros())
        ]
    }
}

fn main() {
    let mut table = Table::new();

    table.add_row(row!["Day", "Part 1", "Part 2", "Duration"]);

    let mut answers: Vec<Answer> = Vec::new();

    answers.push(day_1_main());
    answers.push(day_2_main());
    answers.push(day_3_main());
    answers.push(day_4_main());
    answers.push(day_5_main());
    answers.push(day_6_main());
    answers.push(day_7_main());
    answers.push(day_8_main());
    answers.push(day_9_main());
    answers.push(day_10_main());
    answers.push(day_11_main());
    answers.push(day_12_main());
    answers.push(day_13_main());
    answers.push(day_14_main());

    for answer in &answers {
        table.add_row(answer.to_row());
    }

    let total_duration = answers
        .iter()
        .map(|x| x.duration)
        .fold(Duration::from_secs(0), |acc, x| acc + x);

    table.printstd();
    println!("\nTotal duration: {}μs", total_duration.as_micros());
}
