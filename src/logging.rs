use csv::StringRecord;
use tabled::{builder::Builder, settings::Style};
use std::iter::once;

const LIMIT: usize = 10;

pub fn log_csv_table(data: (StringRecord, Vec<StringRecord>)) {
    let header = data.0;
    let record = data.1;

    let mut builder = Builder::default();
    // before building we need to detect how many lines we want to render
    let lines = record.len();

    let (render_follow, limit_rows): (bool, usize);
    if lines > LIMIT {
        limit_rows = LIMIT;
        render_follow = true;
    } else {
        limit_rows = lines;
        render_follow = false;
    }

    // building the header first
    builder.push_record(header.iter());

    // building then the records
    for i in 0..limit_rows {
        let lines = record.iter()
            .nth(i)
            .unwrap();
        builder.push_record(lines.iter())
    }

    builder.insert_column(0,
        once(String::new())
            .chain(
                (0..limit_rows)
                    .map(|x| x.to_string())
            ));

    if render_follow {
        builder.push_record((0..=header.len()).map(|_| {
            format!("...")
        }));
    };

    let mut table = builder.build();
    table.with(Style::rounded());

    println!("{}", table);
}

