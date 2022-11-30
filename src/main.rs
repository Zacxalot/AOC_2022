mod answer;
mod days;

use prettytable::{row, Row, Table};

use crate::answer::{total_duration, Answer};

fn main() {
    let mut table = Table::new();
    table.add_row(row!["Day", "Part 1", "Part 2", "Duration"]);

    let answers: Vec<Answer> = vec![days::day_1::execute()];

    let duration = total_duration(&answers);

    for answer in answers {
        table.add_row(Row::from(answer));
    }

    table.printstd();
    println!("\nTotal duration: {}μs", duration);
}
