use std::time::Duration;

use prettytable::{cell, row, Row};

pub struct Answer {
    pub day: u32,
    pub part_1: String,
    pub part_2: String,
    pub duration: Duration,
}

impl From<Answer> for Row {
    fn from(answer: Answer) -> Row {
        row![
            &answer.day,
            &answer.part_1,
            &answer.part_2,
            &format!("{}μs", &answer.duration.as_micros())
        ]
    }
}

pub fn total_duration(answers: &[Answer]) -> u128 {
    answers
        .iter()
        .map(|x| x.duration)
        .fold(Duration::from_secs(0), |acc, x| acc + x)
        .as_micros()
}
