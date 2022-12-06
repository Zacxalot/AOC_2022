mod answer;
mod days;

use prettytable::{row, Row, Table};

use crate::answer::{total_duration, Answer};

fn main() {
    let mut table = Table::new();
    table.add_row(row![
        "Day",
        "Part 1",
        "Part 2",
        "Duration",
        "No IO Duration"
    ]);

    let answers: Vec<Answer> = vec![
        days::day_1::execute(),
        days::day_2::execute(),
        days::day_3::execute(),
        days::day_4::execute(),
        days::day_5::execute(),
        days::day_6::execute(),
    ];

    let (duration, no_io_duration) = total_duration(&answers);

    for answer in answers {
        table.add_row(Row::from(answer));
    }

    table.printstd();
    println!("\nTotal duration: {duration}μs (Total no IO duration {no_io_duration}μs)",);
}
