use crate::answer::Answer;
use prettytable::{format, row, Row, Table};

pub fn print_table(answers: Vec<Answer>) {
    let mut table = Table::new();

    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separator(
            format::LinePosition::Title,
            format::LineSeparator::new('-', '|', '|', '|'),
        )
        .padding(1, 1)
        .build();

    table.set_format(format);

    table.set_titles(row![
        "Day",
        "Part 1",
        "Part 2",
        "Duration",
        "No IO Duration"
    ]);

    for answer in answers {
        table.add_row(Row::from(answer));
    }

    table.printstd();
}
