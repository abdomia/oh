use csv::StringRecord;
use tabled::{builder::Builder, settings::Style};
use std::iter::once;

const LIMIT: usize = 10;

// TODO make kind of detecting the csv file and if it's large csv do operations to print it in terminal
// csv file with 1000_000 rows then print first 10 rows for example.
pub fn log_csv_table(data: (StringRecord, Vec<StringRecord>)) {
    let mut max_each_col = vec![
        0; data
            .1
            .iter()
            .nth(0)
            .unwrap()
            .len()
    ];

    for i in 0..max_each_col.len() {
        let mut col: Vec<String> = vec![];
        col.push(data
            .0
            .iter()
            .nth(i)
            .unwrap()
            .to_string()
        );

        col.extend(data.1.iter().map(|row| {
            let v = row.iter().nth(i);
            if let Some(val) = v {
                val.to_string()
            } else {
                "".to_string()
            }
        }));

        let max_len = col
            .iter()
            .max_by_key(|&x| x.chars().count())
            .unwrap()
            .chars()
            .count();
        max_each_col[i] = max_len;
    }

    let mut builder = Builder::default();
    // before building we need to detect how many lines we want to render
    let lines = data.1.len();

    let (render_follow, limit_rows): (bool, usize);
    if lines > LIMIT {
        limit_rows = LIMIT;
        render_follow = true;
    } else {
        limit_rows = lines;
        render_follow = false;
    }

    // building the header first
    builder.push_record(data.0.iter());

    // building then the records
    for i in 0..limit_rows {
        let record = data.1.iter()
            .nth(i)
            .unwrap();
        builder.push_record(record.iter())
    }

    builder.insert_column(0,
        once(String::new())
            .chain(
                (0..limit_rows)
                    .map(|x| x.to_string())
            ));

    if render_follow {
        builder.push_record((0..=data.0.len()).map(|_| {
            format!("...")
        }));
    };

    let mut table = builder.build();
    table.with(Style::rounded());

    println!("{}", table);
}

